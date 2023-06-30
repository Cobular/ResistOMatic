use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum LibError {
    #[error("The voltage `{0}` is not valid. Cannot be NAN or zero.")]
    InvalidVoltage(f64),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}

impl From<LibError> for JsValue {
    fn from(val: LibError) -> Self {
        val.to_string().into()
    }
}


pub type Result<T> = std::result::Result<T, LibError>;