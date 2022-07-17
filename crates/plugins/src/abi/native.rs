use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{abi::fornjot_plugin_init, Model, ModelMetadata};

static HOST: Lazy<Host> = Lazy::new(|| {
    let mut host = Host::default();

    // Safety: We use the register_plugin!() macro to ensure this function was
    // declared with the correct signature.
    unsafe {
        let _metadata = fornjot_plugin_init(&mut host).expect("Unable to initialize the plugin");
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
    let ctx = crate::abi::Context(args);
    let model = HOST
        .models
        .first()
        .expect("No models were registered by the register_plugin!() initializer");

    model.shape(&ctx).expect("Unable to generate the shape")
}

#[derive(Default)]
struct Host {
    models: Vec<Box<dyn Model>>,
}

impl crate::Host for Host {
    fn register_boxed_model(&mut self, model: Box<dyn crate::Model>) {
        if !self.models.is_empty() {
            let ModelMetadata { name, .. } = model.metadata();
            tracing::warn!(
                model=%name,
                "Fornjot's model architecture only allows registering *one* model per plugin. This model will never be called.",
            );
        }

        self.models.push(model);
    }
}
