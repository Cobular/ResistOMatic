use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum LibError {
    #[error("The voltage `{0}` is not valid. Cannot be NAN or zero.")]
    InvalidVoltage(f64),
    #[error("Failed to serialize")]
    SerdeWasmBindgenError(#[from] serde_wasm_bindgen::Error),
    #[error("unknown data store error")]
    Unknown,
}

impl From<LibError> for JsValue {
    fn from(_: LibError) -> Self {
        "encountered some error".into()
    }
}

pub type Result<T> = std::result::Result<T, LibError>;
