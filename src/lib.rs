#[macro_export]
macro_rules! exit_with_log {
    ($log: expr) => {
        eprintln!("{}", $log);
        exit(1)
    };
}

pub mod docx_metadata;
pub mod docx_reader;
pub mod docx_writer;
pub mod xcstrings_metadata;

const KEY_KEY: &str = "key";
const KEY_VARIATION: &str = "variation";
