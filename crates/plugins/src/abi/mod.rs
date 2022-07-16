//! Adapters that let us export a [`crate::Host`]-based plugin using the
//! platform-specific ABI.
//!
//! The crux of how these shims work is the [`fornjot_plugin_init()`] function.
//! Similar to how Rust's standard library [implements its
//! `#[global_allocator]`][global-alloc] attribute, we forward-declare an
//! `extern "Rust"` function that downstream users will provide via the
//! [`crate::register_plugin!()`] macro.
//!
//! This also gives us a "seam" where progress can still be made using the
//! "native" plugin system, while also experimenting with a WebAssembly-based
//! way of doing plugins. The correct interface will be exposed depending on
//! how this crate is compiled (i.e. compiling to `wasm32-unknown-unknown` will
//! use the `wasm` ABI).
//!
//! [global-alloc]: https://github.com/rust-lang/rust/blob/3a8b0144c82197a70e919ad371d56f82c2282833/library/alloc/src/alloc.rs#L22-L39

use std::collections::HashMap;

use crate::{Error, PluginMetadata};

cfg_if::cfg_if! {
    if #[cfg(target_family = "wasm")] {
        mod wasm;
    } else {
        mod native;
    }
}

extern "Rust" {
    /// The "entrypoint" for all plugins. This will be called once when the
    /// plugin is first loaded and the result will be cached.
    pub(crate) fn fornjot_plugin_init(host: &mut dyn crate::Host) -> Result<PluginMetadata, Error>;
}

/// Declare the function that will be called when a plugin is first initialized.
///
/// This is where you'll do things like registering a model with the host and so
/// on.
#[macro_export]
macro_rules! register_plugin {
    ($init:expr) => {
        #[no_mangle]
        #[doc(hidden)]
        pub fn fornjot_plugin_init(
            host: &mut dyn $crate::Host,
        ) -> Result<$crate::PluginMetadata, $crate::Error> {
            // Note: explicitly require a particular function signature.
            let init: fn(&mut dyn $crate::Host) -> Result<$crate::PluginMetadata, $crate::Error> =
                $init;

            init(host)
        }
    };
}

/// A simple [`crate::Context`] implementation backed by a [`HashMap`].
pub(crate) struct Context<'a>(pub &'a HashMap<String, String>);

impl crate::Context for Context<'_> {
    fn get_argument(&self, name: &str) -> Option<&str> {
        self.0.get(name).map(|arg| arg.as_str())
    }
}
