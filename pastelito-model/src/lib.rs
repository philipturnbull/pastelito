//! The default model for the Pastelito tagger.
use lazy_static::lazy_static;
use pastelito_data::Model;
use speedy::Readable as _;

static MODEL_BIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/model.bin"));

lazy_static! {
    static ref MODEL: Model = Model::read_from_buffer(MODEL_BIN).unwrap();
}

/// Get the default model.
pub fn get() -> &'static Model {
    &MODEL
}
