//! A wrapper plugin which makes the WebAssembly plugin pointed to by
//! `$WASM_BINARY` and makes it callable using the native ABI.

use std::{
    fmt::{self, Display, Formatter},
    sync::{Arc, Mutex},
};

use anyhow::Context;
use fj_plugins::{Model, PluginMetadata};
use wasmtime::{Engine, Linker, Module, Store};

wit_bindgen_wasmtime::import!("../../wit-files/guest.wit");
wit_bindgen_wasmtime::export!("../../wit-files/host.wit");

fj_plugins::register_plugin!(|host| {
    let wasm_binary = std::env::var("WASM_BINARY")
        .context("Unable to read the $WASM_BINARY environment variable")?;

    let wasm =
        std::fs::read(&wasm_binary).with_context(|| format!("Unable to read \"{wasm_binary}\""))?;

    let engine = Engine::default();
    let module = Module::new(&engine, &wasm).context("Unable to parse the WebAssembly module")?;
    let mut store = Store::new(&engine, State::default());
    let mut linker = Linker::new(&engine);
    host::add_to_linker(&mut linker, |s: &mut State| &mut s.host)?;
    let (guest, _instance) = guest::Guest::instantiate(&mut store, &module, &mut linker, |state| {
        &mut state.guest_data
    })
    .context("Unable to instantiate the WebAssembly module")?;

    let plugin = guest
        .init(&mut store)
        .context("Calling into WebAssembly triggered a trap")?
        .context("Unable to initialize the plugin")?;

    let store = Arc::new(Mutex::new(store));
    let guest = Arc::new(guest);

    host.register_model_constructor(Box::new(move |ctx| {
        let args: Vec<_> = ctx
            .arguments()
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

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
});

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
    fn log_enabled(&mut self, metadata: host::LogMetadata<'_>) -> bool {
        let metadata = log::Metadata::from(metadata);
        log::logger().enabled(&metadata)
    }

    fn log(&mut self, metadata: host::LogMetadata<'_>, payload: host::LogRecord<'_>) {
        // Note: we use the `log` crate for logging because `tracing` makes it
        // incredibly hard to dynamically emit log messages that retain things
        // like the line number and target.
        //
        // Luckily, tracing provide a compatibility layer where `log` records
        // still make their way to a `tracing` subscriber.
        //
        // See also:
        // - https://docs.rs/tracing/latest/tracing/#emitting-log-records
        // - https://github.com/tokio-rs/tracing/issues/1047

        let host::LogRecord {
            message,
            module_path,
            file,
            line,
        } = payload;

        log::logger().log(
            &log::Record::builder()
                .metadata(metadata.into())
                .args(format_args!("{message}"))
                .module_path(module_path)
                .file(file)
                .line(line)
                .build(),
        );
    }
}

impl<'a> From<host::LogMetadata<'a>> for log::Metadata<'a> {
    fn from(m: host::LogMetadata<'a>) -> Self {
        let host::LogMetadata { level, target } = m;
        log::Metadata::builder()
            .level(level.into())
            .target(target.into())
            .build()
    }
}

impl From<host::LogLevel> for log::Level {
    fn from(level: host::LogLevel) -> Self {
        match level {
            host::LogLevel::Error => log::Level::Error,
            host::LogLevel::Warn => log::Level::Warn,
            host::LogLevel::Info => log::Level::Info,
            host::LogLevel::Debug => log::Level::Debug,
            host::LogLevel::Trace => log::Level::Trace,
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
