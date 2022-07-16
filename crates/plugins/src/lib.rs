//! Abstractions for defining your own Fornjot plugins.
//!
//! The typical workflow is to first define a [`Model`] type.

#![warn(
    missing_copy_implementations,
    missing_docs,
    missing_debug_implementations
)]

mod host;
mod model;
mod shim;

pub use crate::{
    host::{Host, HostExt, ModelConstructor},
    model::{Context, ContextExt, MissingArgument, Model},
};
