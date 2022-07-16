use crate::{model::ModelFromContext, Error, Model};

/// A type-erased function that is called to construct a [`Model`].
pub type ModelConstructor =
    Box<dyn Fn(&dyn crate::Context) -> Result<Box<dyn Model>, Error> + Send + Sync>;

/// An abstract interface to the Fornjot host.
pub trait Host {
    /// Register a model.
    ///
    /// This is mainly for more advanced use cases (e.g. when you need to close
    /// over extra state to load the model). For simpler models, you probably
    /// want to use [`HostExt::register_model()`] instead.
    fn register_model_constructor(&mut self, constructor: ModelConstructor);
}

impl<H: Host + ?Sized> Host for &'_ mut H {
    fn register_model_constructor(&mut self, constructor: ModelConstructor) {
        (*self).register_model_constructor(constructor);
    }
}

impl<H: Host + ?Sized> Host for Box<H> {
    fn register_model_constructor(&mut self, constructor: ModelConstructor) {
        (**self).register_model_constructor(constructor);
    }
}

/// Extension methods to augment the [`Host`] API.
pub trait HostExt {
    /// Register a model with the Fornjot runtime.
    fn register_model<M>(&mut self)
    where
        M: Model + ModelFromContext + 'static;
}

impl<H: Host + ?Sized> HostExt for H {
    fn register_model<M>(&mut self)
    where
        M: Model + ModelFromContext + 'static,
    {
        self.register_model_constructor(Box::new(|ctx| {
            let model = M::from_context(ctx)?;
            Ok(Box::new(model))
        }))
    }
}
