//! A wrapper plugin which makes the WebAssembly plugin pointed to by
//! `$WASM_BINARY` and makes it callable using the native ABI.

use std::{
    fmt::{self, Display, Formatter},
    sync::{Arc, Mutex},
    time::SystemTime,
};

use anyhow::Context;
use fj_plugins::{Model, PluginMetadata};
use wasmtime::{Engine, Linker, Module, Store};

wit_bindgen_wasmtime::import!("../../wit-files/guest.wit");
wit_bindgen_wasmtime::export!("../../wit-files/host.wit");

fj_plugins::register_plugin!(init);

fn init(host: &mut dyn fj_plugins::Host) -> Result<PluginMetadata, fj_plugins::Error> {
    // Note: We need to initialize our own logger because we've been loaded by
    // Fornjot at runtime with no way to access their `static` logger variable.
    let _ = tracing_subscriber::fmt::try_init();

    let wasm_binary = std::env::var("WASM_BINARY")
        .context("Unable to read the $WASM_BINARY environment variable")?;

    tracing::debug!(?wasm_binary, "Reading WebAssembly from disk");

    let wasm =
        std::fs::read(&wasm_binary).with_context(|| format!("Unable to read \"{wasm_binary}\""))?;

    let engine = Engine::default();
    let module = Module::new(&engine, &wasm).context("Unable to parse the WebAssembly module")?;
    let mut store = Store::new(&engine, State::default());
    let mut linker = Linker::new(&engine);
    host::add_to_linker(&mut linker, |s: &mut State| &mut s.host)?;

    tracing::debug!("Instantiating the WebAssembly module");

    let (guest, _instance) = guest::Guest::instantiate(&mut store, &module, &mut linker, |state| {
        &mut state.guest_data
    })
    .context("Unable to instantiate the WebAssembly module")?;

    tracing::debug!("Initializing the plugin");
    let plugin = guest
        .init(&mut store)
        .context("Calling into WebAssembly triggered a trap")?
        .context("Unable to initialize the plugin")?;

    let store = Arc::new(Mutex::new(store));
    let guest = Arc::new(guest);

    host.register_model_constructor(Box::new(move |ctx| {
        tracing::debug!(arguments=?ctx.arguments(), "Loading a model");
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

        let model = guest
            .plugin_load_model(&mut *store.lock().unwrap(), &plugin, &args)
            .context("Calling into WebAssembly triggered a trap")?
            .context("Unable to load the model")?;

        Ok(Box::new(WebAssemblyModel {
            guest: Arc::clone(&guest),
            store: Arc::clone(&store),
            model,
        }))
    }));

    Ok(PluginMetadata::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
    ))
}

struct WebAssemblyModel {
    guest: Arc<guest::Guest<State>>,
    store: Arc<Mutex<Store<State>>>,
    model: guest::Model,
}

impl Model for WebAssemblyModel {
    fn shape(&self) -> fj::Shape {
        let WebAssemblyModel {
            guest,
            store,
            model,
        } = self;
        let mut store = store.lock().unwrap();
        guest
            .model_shape(&mut *store, model)
            .expect("Calling the model's shape function raised a trap")
            .into()
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
