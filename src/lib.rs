pub mod config;
mod docx_writer;

use std::path::{Path, PathBuf};
use anyhow::Result;

use crate::config::{Config};
use crate::docx_writer::convert;

pub fn convert_from_path<'a>(path_to_xcstrings: PathBuf, config: Config) -> Result<()> {
    convert_from_raw(&std::fs::read(path_to_xcstrings)?, config)
}

pub fn convert_from_raw<'a>(xcstrings: &[u8], config: Config) -> Result<()> {
    convert(swift_localizable_json_parser::parse_from_bytes(xcstrings).localized_per_language(), config)
}

#[test]
fn test_convert() {
    let base = std::env::current_dir().unwrap();

    assert_eq!("xcstringsdocx", base.file_name().unwrap().to_str().unwrap());

    let base = base.join("generated");
    let raw = include_bytes!("../resources/Localizable.xcstrings");

    convert_from_raw(raw, Config {
        base_language_code: "en".to_string(),
        save_in: base,
        clean_dir_before_generating: true,
    }).unwrap();
}