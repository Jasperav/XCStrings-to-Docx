use clap::Parser;
use std::path::PathBuf;

#[derive(Clone, Debug, Parser)]
pub struct Config {
    pub extract_from_docx: PathBuf,
    pub base_xcstrings: Option<PathBuf>,
}
