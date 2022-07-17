//! Abstractions for defining your own Fornjot plugins.
//!
//! The typical workflow is to first define a [`Model`] type. This comes with
//! two main methods,
//!
//! 1. Getting metadata about the model, and
//! 2. Calculating the model's geometry using arguments from the host context
//!
//! ```rust
//! use fj_plugins::{Model, Context, ContextExt, Error, ModelMetadata, ArgumentMetadata};
//! use fj::{Shape, Circle, Sketch, syntax::*};
//!
//! struct Cylinder;
//!
//! impl Model for Cylinder {
//!     fn metadata(&self) -> ModelMetadata {
//!         ModelMetadata::new("Cylinder")
//!             .with_argument("radius")
//!             .with_argument(
//!                 ArgumentMetadata::new("height")
//!                     .with_default_value("10.0"),
//!             )
//!     }
//!
//!     fn shape(&self, ctx: &dyn Context) -> Result<Shape, Error>
//!     {
//!         let radius = ctx.parse_argument("radius")?;
//!         let height = ctx.parse_optional_argument("height")?.unwrap_or(10.0);
//!         let circle = Circle::from_radius(radius);
//!         Ok(Sketch::from_circle(circle).sweep([height, 0.0, 0.0]).into())
//!     }
//! }
//! ```
//!
//! Once you've done that, you can use the [`register_plugin!()`] macro to
//! set up the plugin (possibly registering a model) and tell the host some
//! basic information about it.
//!
//! ```rust
//! use fj_plugins::{Host, HostExt, PluginMetadata};
//! # use fj_plugins::{Model, Context, Error, ModelMetadata};
//!
//! fj_plugins::register_plugin!(|host: &mut dyn Host| {
//!     host.register_model(Cylinder);
//!
//!     Ok(PluginMetadata::new(
//!         env!("CARGO_PKG_NAME"),
//!         env!("CARGO_PKG_VERSION"),
//!     ))
//! });
//! # struct Cylinder;
//! # impl Model for Cylinder {
//! #     fn metadata(&self) -> ModelMetadata { unimplemented!() }
//! #     fn shape(&self, ctx: &dyn Context) -> Result<fj::Shape, Error> { unimplemented!() }
//! # }
//! ```

#![warn(
    missing_copy_implementations,
    missing_docs,
    missing_debug_implementations
)]

mod abi;
mod host;
mod metadata;
mod model;

pub use crate::{
    host::{Host, HostExt},
    metadata::{ArgumentMetadata, ModelMetadata, PluginMetadata},
    model::{Context, ContextExt, MissingArgument, Model},
};

/// The common error type used by this crate.
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
