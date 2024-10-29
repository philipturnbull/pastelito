//! The default model for the Pastelito tagger.
use std::sync::OnceLock;

use crate::data::Model;
use speedy::Readable as _;

static MODEL_BIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/model.bin"));

static MODEL: OnceLock<Model> = OnceLock::new();

/// Get the default model.
pub fn get() -> &'static Model {
    MODEL.get_or_init(|| Model::read_from_buffer(MODEL_BIN).unwrap())
}
