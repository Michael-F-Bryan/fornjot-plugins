//! A wrapper plugin which makes the WebAssembly plugin pointed to by
//! `$FJ_WASM_PLUGIN` and makes it callable using the native ABI.

use std::{
    fmt::{self, Display, Formatter},
    path::Path,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use anyhow::Context as _;
use fj_plugins::{ArgumentMetadata, Context, HostExt, Model, ModelMetadata, PluginMetadata};
use wasmtime::{Engine, Linker, Module, Store};

wit_bindgen_wasmtime::import!("../../wit-files/guest.wit");
wit_bindgen_wasmtime::export!("../../wit-files/host.wit");

pub const ENV_NAME: &str = "FJ_WASM_PLUGIN";

fj_plugins::register_plugin!(|host| {
    // Note: This library has been compiled as its own self-contained cdylib,
    // complete with our own static variables, meaning the logger in our version
    // of tracing isn't related to the logger set up by Fornjot.
    let _ = tracing_subscriber::fmt::try_init();

    let wasm_binary = std::env::var(ENV_NAME)
        .with_context(|| format!("Unable to read the ${ENV_NAME} environment variable"))?;

    let plugin = Plugin::load(&wasm_binary)?;

    for model in plugin.models()? {
        host.register_model(model);
    }

    Ok(plugin.metadata())
});

/// The instantiated WebAssembly module that implements a Fornjot plugin.
pub struct Plugin {
    store: Arc<Mutex<Store<State>>>,
    guest: Arc<guest::Guest<State>>,
    plugin: guest::Plugin,
}

impl Plugin {
    /// Load the [`Plugin`] from disk.
    pub fn load(path: impl AsRef<Path>) -> Result<Plugin, fj_plugins::Error> {
        let path = path.as_ref();
        tracing::debug!(path = %path.display(), "Reading WebAssembly from disk");

        let wasm = std::fs::read(&path)
            .with_context(|| format!("Unable to read \"{}\"", path.display()))?;

        Plugin::from_memory(&wasm)
    }

    fn from_memory(wasm: &[u8]) -> Result<Plugin, fj_plugins::Error> {
        let engine = Engine::default();

        tracing::debug!("Parsing the WebAssembly module");
        let module =
            Module::new(&engine, wasm).context("Unable to parse the WebAssembly module")?;

        let mut store = Store::new(&engine, State::default());
        let mut linker = Linker::new(&engine);
        host::add_to_linker(&mut linker, |s: &mut State| &mut s.host)?;

        tracing::debug!("Instantiated the WebAssembly module");
        let (guest, _instance) =
            guest::Guest::instantiate(&mut store, &module, &mut linker, |state| {
                &mut state.guest_data
            })
            .context("Unable to instantiate the WebAssembly module")?;

        tracing::debug!("Initializing the plugin");
        let plugin = guest
            .init(&mut store)?
            .context("Unable to initialize the plugin")?;

        Ok(Plugin {
            plugin,
            store: Arc::new(Mutex::new(store)),
            guest: Arc::new(guest),
        })
    }

    pub fn metadata(&self) -> PluginMetadata {
        let Plugin {
            store,
            guest,
            plugin,
        } = self;

        guest
            .plugin_metadata(&mut *store.lock().unwrap(), plugin)
            .unwrap()
            .into()
    }

    pub fn models(&self) -> Result<Vec<impl Model>, fj_plugins::Error> {
        let Plugin {
            store,
            guest,
            plugin,
        } = self;

        let models = guest
            .plugin_models(&mut *store.lock().unwrap(), &plugin)?
            .into_iter()
            .map(|model| WebAssemblyModel {
                guest: Arc::clone(&guest),
                store: Arc::clone(&store),
                model,
            })
            .collect();

        Ok(models)
    }
}

struct WebAssemblyModel {
    guest: Arc<guest::Guest<State>>,
    store: Arc<Mutex<Store<State>>>,
    model: guest::Model,
}

impl Model for WebAssemblyModel {
    fn metadata(&self) -> fj_plugins::ModelMetadata {
        let WebAssemblyModel {
            guest,
            store,
            model,
        } = self;

        guest
            .model_metadata(&mut *store.lock().unwrap(), model)
            .expect("Call to WebAssembly failed")
            .into()
    }

    fn shape(&self, ctx: &dyn Context) -> Result<fj::Shape, fj_plugins::Error> {
        let WebAssemblyModel {
            guest,
            store,
            model,
        } = self;
        let mut store = store.lock().unwrap();

        let mut args: Vec<_> = ctx
            .arguments()
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        // HACK: It looks like wit-bindgen will unconditionally try to
        // deallocate zero-length arrays, which corrupts the allocator because
        // zero-length arrays point to garbage (null + align_of<T>). To avoid
        // this, we make sure there's at least 1 argument in the list.
        args.push(("", ""));

        guest
            .model_shape(&mut *store, model, &args)
            .expect("Calling the model's shape function raised a trap")
            .map(Into::into)
            .map_err(Into::into)
    }
}

#[derive(Default)]
struct State {
    guest_data: guest::GuestData,
    host: Host,
}

#[derive(Default)]
struct Host;

impl host::Host for Host {
    fn print(&mut self, msg: &str) {
        print!("{msg}");
    }

    fn abort(&mut self, msg: &str) {
        let err = anyhow::anyhow!("{msg}");
        // SAFETY: This method will only ever be called by wit-bindgen-wasmtime
        // so we'll always be within a WebAssembly context.
        unsafe {
            wasmtime_runtime::raise_user_trap(err);
        }
    }

    fn time(&mut self) -> host::Timespec {
        let delta = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Current time is before the UNIX epoch");

        host::Timespec {
            secs: delta.as_secs() as i64,
            // Note: Truncate to microseconds because we don't need 9
            // significant figures for a timestamp.
            nanos: 1000 * delta.subsec_micros() as u32,
        }
    }

    fn log_filter(&mut self) -> Option<String> {
        std::env::var("RUST_LOG").ok()
    }
}

impl Display for guest::Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.message, f)
    }
}

impl std::error::Error for guest::Error {}

impl From<guest::Shape> for fj::Shape {
    fn from(s: guest::Shape) -> Self {
        match s {
            guest::Shape::Shape2d(s) => fj::Shape::Shape2d(s.into()),
            guest::Shape::Sweep(s) => fj::Shape::Sweep(s.into()),
        }
    }
}

impl From<guest::Sweep> for fj::Sweep {
    fn from(s: guest::Sweep) -> Self {
        let guest::Sweep {
            shape,
            path: (a, b, c),
        } = s;
        fj::Sweep::from_path(shape.into(), [a, b, c])
    }
}

impl From<guest::Shape2d> for fj::Shape2d {
    fn from(s: guest::Shape2d) -> Self {
        match s {
            guest::Shape2d::Sketch(s) => fj::Shape2d::Sketch(s.into()),
        }
    }
}

impl From<guest::Sketch> for fj::Sketch {
    fn from(s: guest::Sketch) -> Self {
        let guest::Sketch {
            chain,
            color: (r, g, b, a),
        } = s;
        let color = [r, g, b, a];

        match chain {
            guest::Chain::Circle(c) => fj::Sketch::from_circle(c.into()).with_color(color),
            guest::Chain::PolyChain(p) => fj::Sketch::from_points(p.into()).with_color(color),
        }
    }
}

impl From<guest::Circle> for fj::Circle {
    fn from(c: guest::Circle) -> Self {
        fj::Circle::from_radius(c.radius)
    }
}

impl From<guest::PolyChain> for Vec<[f64; 2]> {
    fn from(p: guest::PolyChain) -> Self {
        p.points.into_iter().map(|(x, y)| [x, y]).collect()
    }
}

impl From<guest::ModelMetadata> for ModelMetadata {
    fn from(m: guest::ModelMetadata) -> ModelMetadata {
        let guest::ModelMetadata {
            name,
            description,
            arguments,
        } = m;
        ModelMetadata {
            name,
            description,
            arguments: arguments.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<guest::ArgumentMetadata> for ArgumentMetadata {
    fn from(m: guest::ArgumentMetadata) -> ArgumentMetadata {
        let guest::ArgumentMetadata {
            name,
            description,
            default_value,
        } = m;
        ArgumentMetadata {
            name,
            description,
            default_value,
        }
    }
}

impl From<guest::PluginMetadata> for PluginMetadata {
    fn from(m: guest::PluginMetadata) -> Self {
        let guest::PluginMetadata {
            name,
            version,
            short_description,
            description,
            homepage,
            repository,
            license,
        } = m;

        PluginMetadata {
            name,
            version,
            short_description,
            description,
            homepage,
            repository,
            license,
        }
    }
}
