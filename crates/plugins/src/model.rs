use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crate::Error;

/// A model.
pub trait Model {
    /// Try to initialize this [`Model`] using contextual information it has
    /// been provided.
    fn from_context(ctx: &dyn Context) -> Result<Self, Error>
    where
        Self: Sized;

    /// Calculate this model's concrete geometry.
    fn shape(&self) -> fj::Shape;
}

/// Contextual information passed to a [`Model`] when it is being initialized.
pub trait Context {
    /// Get an argument that was passed to this model.
    fn get_argument(&self, name: &str) -> Option<&str>;
}

impl<C: Context + ?Sized> Context for &'_ C {
    fn get_argument(&self, name: &str) -> Option<&str> {
        (*self).get_argument(name)
    }
}

impl<C: Context + ?Sized> Context for Box<C> {
    fn get_argument(&self, name: &str) -> Option<&str> {
        (**self).get_argument(name)
    }
}

impl<C: Context + ?Sized> Context for std::rc::Rc<C> {
    fn get_argument(&self, name: &str) -> Option<&str> {
        (**self).get_argument(name)
    }
}

impl<C: Context + ?Sized> Context for std::sync::Arc<C> {
    fn get_argument(&self, name: &str) -> Option<&str> {
        (**self).get_argument(name)
    }
}

/// Extension methods for the [`Context`] type.
///
/// By splitting these methods out into a separate trait, [`Context`] can stay
/// object-safe while allowing convenience methods that use generics.
pub trait ContextExt {
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
    #[error(transparent)]
    MissingArgument(#[from] MissingArgument),
    #[error(transparent)]
    ParseFailed(#[from] ParseFailed),
}

/// The error returned when a required argument wasn't provided.
#[derive(Debug, Clone, PartialEq)]
pub struct MissingArgument {
    /// The argument's name.
    pub name: String,
}

impl Display for MissingArgument {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let MissingArgument { name } = self;
        write!(f, "The \"{name}\" argument is missing")
    }
}

impl std::error::Error for MissingArgument {}

#[derive(Debug)]
pub struct ParseFailed {
    pub name: String,
    pub value: String,
    pub error: Box<dyn std::error::Error + Send + Sync>,
}

impl Display for ParseFailed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ParseFailed { name, value, .. } = self;
        write!(f, "Unable to parse the \"{name}\" argument (\"{value:?}\")")
    }
}

impl std::error::Error for ParseFailed {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.error)
    }
}
