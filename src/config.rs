use std::path::PathBuf;
use docx_rust::Docx;

pub struct Config {
    /// Would normally be 'en'
    pub base_language_code: String,
    pub save_in: PathBuf,
    pub clean_dir_before_generating: bool,
}

pub struct Converted<'a> {
    pub docx: Docx<'a>,
    pub language: String,
}