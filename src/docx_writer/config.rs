use clap::Parser;

use std::path::PathBuf;

#[derive(Clone, Debug, Parser)]
pub struct Config {
    #[clap(long)]
    pub save_in: PathBuf,
    #[clap(long)]
    pub clean_dir_before_generating: bool,
    /// If you added a new language, add it to the list so a new docx file can be generated
    #[clap(long)]
    pub new_language_codes: Vec<String>,
    #[clap(long)]
    pub path_to_xcstrings: PathBuf,
    #[clap(long)]
    pub columns_in_output: Vec<Column>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, clap::ValueEnum)]
pub enum Column {
    State,
}
