//! The Pastelito model.
#![feature(ascii_char)]
mod data;
mod model;

// These definitions should just be declared at the top-level but we need to use
// them as part of `build.rs`. Re-export them here so they appear at the
// top-level.
pub use data::{ContextSuffix, ContextWord, Feature, Model, Scores, Tag, WeightRange};
pub use model::get;
