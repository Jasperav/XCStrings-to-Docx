#[macro_export]
macro_rules! exit_with_log {
    ($log: expr) => {{
        log::error!("{}", $log);

        return Err(crate::error::ConvertError::Wrapped($log.to_string()))
    }};
}

pub mod docx_metadata;
pub mod docx_reader;
pub mod docx_writer;
pub mod xcstrings_metadata;
pub mod error;

const KEY_KEY: &str = "key";
const KEY_VARIATION: &str = "variation";
