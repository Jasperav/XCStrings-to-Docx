use clap::Parser;
use std::path::PathBuf;

#[derive(Clone, Debug, Parser)]
pub struct Config {
    /// The path to an existing .xcstrings file to merge the localizations from
    #[clap(long)]
    pub base_xcstrings: PathBuf,
    #[clap(long)]
    pub write_in: PathBuf,
    #[clap(long)]
    pub app_name_for_android: String,
}
