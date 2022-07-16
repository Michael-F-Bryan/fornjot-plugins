//! Abstractions for defining your own Fornjot plugins.
//!
//! The typical workflow is to first define a [`Model`] type. This comes with
//! two main methods,
//!
//! 1. Load a model from the host context (typically by reading arguments
//!    and parsing them into the correct type), and
//! 2. Calculate the model's shape
//!
//! ```rust
//! use fj_plugins::{Model, Context};
//!
//! struct MyModel;
//!
//! impl Model for MyModel {
//!     fn from_context(ctx: &dyn Context) -> Result<Self, anyhow::Error>
//!     where
//!         Self: Sized,
//!     {
//!         todo!("Load arguments from the context and initialize the model");
//!     }
//!
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
//! # use fj_plugins::{Model, Context};
//!
//! fj_plugins::register_plugin!(|host: &mut dyn Host| {
//!     host.register_model::<MyModel>();
//!
//!     PluginMetadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
//! });
//! # struct MyModel;
//! # impl Model for MyModel {
//! #   fn from_context(ctx: &dyn Context) -> Result<Self, anyhow::Error> where Self: Sized { todo!(); }
//! #   fn shape(&self) -> fj::Shape { todo!("Calcualte the model's geometry") }
//! # }
//! ```

#![warn(
    missing_copy_implementations,
    missing_docs,
    missing_debug_implementations
)]

mod host;
mod metadata;
mod model;
mod shim;

pub use crate::{
    host::{Host, HostExt, ModelConstructor},
    metadata::PluginMetadata,
    model::{Context, ContextExt, MissingArgument, Model},
};
