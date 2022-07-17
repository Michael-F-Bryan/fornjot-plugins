use crate::{Context, Error, ModelMetadata};

/// A model.
pub trait Model: Send + Sync {
    /// Calculate this model's concrete geometry.
    fn shape(&self, ctx: &dyn Context) -> Result<fj::Shape, Error>;

    /// Get metadata for the model.
    fn metadata(&self) -> ModelMetadata;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_is_object_safe() {
        let _: &dyn Model;
    }
}
