use crate::{Error, Model};

/// A type-erased function that is called to construct a [`Model`].
pub type ModelConstructor = fn(&dyn crate::Context) -> Result<Box<dyn Model>, Error>;

/// An abstract interface to the Fornjot host.
pub trait Host {
    /// Register a model.
    ///
    /// Don't use this directly. You probably want to use the
    /// [`HostExt::register_model()`] method instead.
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
