use std::collections::HashMap;

use anyhow::Context as _;
use once_cell::sync::Lazy;

use crate::ModelConstructor;

extern "Rust" {
    /// The "entrypoint" for all plugins. This will be called once when the
    /// plugin is first loaded and the result will be cached.
    fn fornjot_plugin_init(host: &mut dyn crate::Host);
}

static HOST: Lazy<Host> = Lazy::new(|| {
    let mut host = Host::default();

    // Safety: We use the register_plugin!() macro to ensure this function was
    // declared with the correct signature.
    //
    // If a downstream crate forgets to register their plugin then they're going
    // to run into linker errors.
    unsafe {
        fornjot_plugin_init(&mut host);
    }

    host
});

/// A shim function which lets us implement the binary interface Fornjot
/// expects while retaining [`fornjot_plugin_init()`] semantics.
///
/// # Safety
///
/// This function is kinda unsound.
///
/// For one, we are using a `HashMap` as an argument to an `extern "C"` function
/// and a `HashMap` isn't FFI-safe (not `#[repr(C)]`).
///
/// We also don't handle the possibility of panicking because the function is
/// assumed to succeed unconditionally. Down the track we should probably use
/// [`std::panic::catch_unwind()`] and return an appropriate error, but for now
/// ... YOLO 🙃
#[no_mangle]
pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
    let ctx = Context(args);
    let model = (HOST.constructor)(&ctx)
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

struct Host {
    constructor: ModelConstructor,
}

impl Default for Host {
    fn default() -> Self {
        Self {
            constructor: |_| {
                panic!("No model registered. Did you forget to call the register_plugin!() macro?")
            },
        }
    }
}

impl crate::Host for Host {
    fn register_model_constructor(&mut self, constructor: ModelConstructor) {
        self.constructor = constructor;
    }
}

/// Declare the function that will be called when a plugin is first initialized.
///
/// This is where you'll do things like registering a model with the host.
#[macro_export]
macro_rules! register_plugin {
    ($init:expr) => {
        #[no_mangle]
        #[doc(hidden)]
        pub fn fornjot_plugin_init(host: &mut dyn $crate::Host) {
            let init: fn(&mut dyn $crate::Host) = $init;
            init(host);
        }
    };
}
