#[macro_export]
macro_rules! exit_with_log {
    ($log: expr) => {{
        debug_assert!(false, "{}", $log.to_string());

        return Err($crate::error::ConvertError::Wrapped($log.to_string()));
    }};
}

pub mod android_xml_writer;
pub mod docx_metadata;
pub mod docx_writer;
pub mod error;
pub mod xcstrings_docx_merger;
pub mod xcstrings_metadata;
pub mod extension_determiner;

const KEY_KEY: &str = "Key";
const KEY_VARIATION: &str = "Variation";
const KEY_COMMENT: &str = "Comment";
