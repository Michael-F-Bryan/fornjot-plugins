use std::{collections::HashMap, str::FromStr};

/// Contextual information passed to a [`Model`][crate::Model] when it is being
/// initialized.
///
/// Check out the [`ContextExt`] trait for some helper methods.
pub trait Context {
    /// The arguments dictionary associated with this [`Context`].
    fn arguments(&self) -> &HashMap<String, String>;
}

impl<C: Context + ?Sized> Context for &'_ C {
    fn arguments(&self) -> &HashMap<String, String> {
        (**self).arguments()
    }
}

impl<C: Context + ?Sized> Context for Box<C> {
    fn arguments(&self) -> &HashMap<String, String> {
        (**self).arguments()
    }
}

impl<C: Context + ?Sized> Context for std::rc::Rc<C> {
    fn arguments(&self) -> &HashMap<String, String> {
        (**self).arguments()
    }
}

impl<C: Context + ?Sized> Context for std::sync::Arc<C> {
    fn arguments(&self) -> &HashMap<String, String> {
        (**self).arguments()
    }
}

impl Context for HashMap<String, String> {
    fn arguments(&self) -> &HashMap<String, String> {
        self
    }
}

/// Extension methods for the [`Context`] type.
///
/// By splitting these methods out into a separate trait, [`Context`] can stay
/// object-safe while allowing convenience methods that use generics.
pub trait ContextExt {
    /// Get an argument that was passed to this model.
    fn get_argument(&self, name: &str) -> Option<&str>;

    /// Get an argument, returning a [`MissingArgument`] error if it doesn't
    /// exist.
    fn get_required_argument(&self, name: &str) -> Result<&str, MissingArgument>;

    /// Parse an argument from its string representation using [`FromStr`].
    fn parse_argument<T>(&self, name: &str) -> Result<T, ContextError>
    where
        T: FromStr,
        T::Err: std::error::Error + Send + Sync + 'static;

    /// Try to parse an argument, if it is present.
    fn parse_optional_argument<T>(&self, name: &str) -> Result<Option<T>, ParseFailed>
    where
        T: FromStr,
        T::Err: std::error::Error + Send + Sync + 'static;
}

impl<C: Context + ?Sized> ContextExt for C {
    fn get_argument(&self, name: &str) -> Option<&str> {
        self.arguments().get(name).map(|s| s.as_str())
    }

    fn get_required_argument(&self, name: &str) -> Result<&str, MissingArgument> {
        self.get_argument(name).ok_or_else(|| MissingArgument {
            name: name.to_string(),
        })
    }

    fn parse_argument<T>(&self, name: &str) -> Result<T, ContextError>
    where
        T: FromStr,
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        let value = self.get_required_argument(name)?;

        value
            .parse()
            .map_err(|e| ParseFailed {
                name: name.to_string(),
                value: value.to_string(),
                error: Box::new(e),
            })
            .map_err(ContextError::from)
    }

    fn parse_optional_argument<T>(&self, name: &str) -> Result<Option<T>, ParseFailed>
    where
        T: FromStr,
        T::Err: std::error::Error + Send + Sync + 'static,
    {
        let value = match self.get_argument(name) {
            Some(value) => value,
            None => return Ok(None),
        };

        let parsed = value.parse().map_err(|e| ParseFailed {
            name: name.to_string(),
            value: value.to_string(),
            error: Box::new(e),
        })?;

        Ok(Some(parsed))
    }
}

/// An error that may be returned from a [`Context`] method.
#[derive(Debug, thiserror::Error)]
pub enum ContextError {
    /// An argument was missing.
    #[error(transparent)]
    MissingArgument(#[from] MissingArgument),
    /// An argument was present, but we were unable to parse it into the final
    /// type.
    #[error(transparent)]
    ParseFailed(#[from] ParseFailed),
}

/// The error returned when a required argument wasn't provided.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("The \"{name}\" argument is missing")]
pub struct MissingArgument {
    /// The argument's name.
    pub name: String,
}

/// The error returned when [`ContextExt::parse_argument()`] is unable to parse
/// the argument's value.
#[derive(Debug, thiserror::Error)]
#[error("Unable to parse the \"{name}\" argument (\"{value:?}\")")]
pub struct ParseFailed {
    /// The argument's name.
    pub name: String,
    /// The actual value.
    pub value: String,
    /// The error that occurred.
    #[source]
    pub error: crate::Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_is_object_safe() {
        let _: &dyn Context;
    }
}
