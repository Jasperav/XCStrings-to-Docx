use clap::Parser;
use std::path::PathBuf;

#[derive(Clone, Debug, Parser)]
pub struct Config {
    /// Path to the docx file to read from
    #[clap(long)]
    pub extract_from_docx: PathBuf,
    /// The path to an existing .xcstrings file to merge the localizations from
    #[clap(long)]
    pub base_xcstrings: PathBuf,
    /// The path to an existing or non-existing .xcstrings file to merge the localizations to
    #[clap(long)]
    pub updated_xcstrings: PathBuf,
}
