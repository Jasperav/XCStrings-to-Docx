use clap::Parser;
use std::path::PathBuf;

#[derive(Clone, Debug, Parser)]
pub struct Config {
    pub path_to_xcstrings: PathBuf,
}
