use std::path::PathBuf;
use clap::Parser;
use serde::Serialize;
use swift_localizable_json_parser::types::input::Translation;
use crate::error::ConvertError;
use crate::xcstrings_docx_merger::docx_extractor::extract;

#[derive(Clone, Debug, Parser)]
pub struct Config {
    #[clap(long)]
    pub path_to_file: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
pub enum Export {
    XCStrings,
    Docx,
    Other,
}

pub fn extension_determiner(config: Config) -> Result<Export, ConvertError> {
    match extract(&config.path_to_file) {
        Ok(_) => return Ok(Export::Docx),
        Err(_) => {
            // Nothing to do, no valid docx
        }
    }

    let to_str = std::fs::read_to_string(&config.path_to_file)?;
    let translation: serde_json::Result<Translation> = serde_json::from_str(&to_str);

    if translation.is_ok() {
        Ok(Export::XCStrings)
    } else {
        Ok(Export::Other)
    }
}