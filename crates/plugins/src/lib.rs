use anyhow::Error;

mod shim;

/// A prelude module containing various traits that will be required by most
/// plugin implementations.
pub mod prelude {
    pub use crate::{Context, Host, HostExt, Model};
}

/// A model.
pub trait Model {
    /// Try to initialize this [`Model`] using contextual information it has
    /// been provided.
    fn from_context(ctx: &dyn Context) -> Result<Self, anyhow::Error>
    where
        Self: Sized;

    /// Calculate this model's concrete geometry.
    fn shape(&self) -> fj::Shape;
}

/// Contextual information passed to a [`Model`] when it is being initialized.
pub trait Context {
    /// Get an argument that was passed to this model.
    fn get_argument(&self, name: &str) -> Option<&str>;

    fn get_required_argument(&self, name: &str) -> Result<&str, MissingArgument> {
        self.get_argument(name).ok_or_else(|| MissingArgument {
            name: name.to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[error("The \"{name}\" argument is missing")]
pub struct MissingArgument {
    pub name: String,
}

type ModelConstructor = fn(&dyn crate::Context) -> Result<Box<dyn Model>, Error>;

/// The Fornjot host.
pub trait Host {
    /// Register a model. You are probably looking for the
    /// [`HostExt::register_model()`] method.
    fn register_model_constructor(&mut self, constructor: ModelConstructor);
}

/// Extension methods to augment the [`Host`] API.
pub trait HostExt {
    /// Register a model with the Fornjot runtime.
    fn register_model<M>(&mut self)
    where
        M: Model + 'static;
}

impl<H: Host + ?Sized> HostExt for H {
    fn register_model<M>(&mut self)
    where
        M: Model + 'static,
    {
        self.register_model_constructor(|ctx| {
            let model = M::from_context(ctx)?;
            Ok(Box::new(model))
        })
    }
}
