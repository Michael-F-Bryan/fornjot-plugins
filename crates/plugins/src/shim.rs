use std::collections::HashMap;

use anyhow::Context as _;
use once_cell::sync::Lazy;

use crate::ModelConstructor;

extern "Rust" {
    /// The "entrypoint" for all plugins.
    ///
    /// This will be called once when the plugin is first loaded. If downstream
    /// users don't provide a `#[no_mangle] fn plugin_init()` function then
    /// we'll run into linker errors.
    fn fornjot_plugin_init(host: &mut dyn crate::Host);
}

static CONSTRUCTOR: Lazy<ModelConstructor> = Lazy::new(|| {
    let mut host = Host::default();

    // Safety: We rely on the downstream crate to provide this function and
    // use the right signature. In the future the signature (and therefore
    // lifetimes and all that) will be statically validated by a custom
    // attribute, but for now... YOLO.
    unsafe {
        fornjot_plugin_init(&mut host);
    }

    let Host { constructor } = host;

    constructor.expect("The plugin didn't register a model")
});

/// A shim function which lets us implement the binary interface Fornjot
/// expects while retaining [`plugin_init()`] semantics.
#[no_mangle]
pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
    let ctx = Context(args);
    let model = CONSTRUCTOR(&ctx)
        .context("Unable to initialize the model")
        .unwrap();
    model.shape()
}

struct Context<'a>(&'a HashMap<String, String>);

impl crate::Context for Context<'_> {
    fn get_argument(&self, name: &str) -> Option<&str> {
        self.0.get(name).map(|arg| arg.as_str())
    }
}

#[derive(Default)]
struct Host {
    constructor: Option<ModelConstructor>,
}

impl crate::Host for Host {
    fn register_model_constructor(&mut self, constructor: ModelConstructor) {
        self.constructor = Some(constructor);
    }
}
