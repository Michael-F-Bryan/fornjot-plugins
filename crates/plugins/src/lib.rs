//! Abstractions for defining your own Fornjot plugins.
//!
//! The typical workflow is to first define a [`Model`] type. This comes with
//! two main methods,
//!
//! 1. Use the [`ModelFromContext`] trait to make the model loadable from the
//!    host context, and
//! 2. Calculate the model's shape with the [`Model`] trait
//!
//! ```rust
//! use fj_plugins::{Model, Context, Error, ModelFromContext};
//!
//! struct MyModel;
//!
//! impl ModelFromContext for MyModel {
//!     fn from_context(ctx: &dyn Context) -> Result<Self, Error>
//!     where
//!         Self: Sized,
//!     {
//!         todo!("Load arguments from the context and initialize the model");
//!     }
//! }
//!
//! impl Model for MyModel {
//!     fn shape(&self) -> fj::Shape { todo!("Calcualte the model's geometry") }
//! }
//! ```
//!
//! Once you've done that, you can use the [`register_plugin!()`] macro to
//! set up the plugin (possibly registering a model) and tell the host some
//! basic information about it.
//!
//! ```rust
//! use fj_plugins::{Host, HostExt, PluginMetadata};
//! # use fj_plugins::{Model, Context, Error, ModelFromContext};
//!
//! fj_plugins::register_plugin!(|host: &mut dyn Host| {
//!     host.register_model::<MyModel>();
//!
//!     Ok(PluginMetadata::new(
//!         env!("CARGO_PKG_NAME"),
//!         env!("CARGO_PKG_VERSION"),
//!     ))
//! });
//! # struct MyModel;
//! # impl Model for MyModel {
//! #   fn shape(&self) -> fj::Shape { todo!("Calcualte the model's geometry") }
//! # }
//! # impl ModelFromContext for MyModel {
//! #   fn from_context(ctx: &dyn Context) -> Result<Self, Error> where Self: Sized { todo!(); }
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
    host::{Host, HostExt, ModelConstructor},
    metadata::PluginMetadata,
    model::{Context, ContextExt, MissingArgument, Model, ModelFromContext},
};

/// The common error type used by this crate.
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
