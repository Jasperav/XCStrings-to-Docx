use clap::{Parser, Subcommand};
use std::process::exit;
use xcstringsdocx::exit_with_log;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: ConfigContainer,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        ConfigContainer::Read(r) => xcstringsdocx::docx_reader::convert::read(r),
        ConfigContainer::Write(w) => {
            match xcstringsdocx::docx_writer::convert::convert_from_path(w) {
                Ok(_) => {
                    println!("Successfully wrote the docx file")
                }
                Err(err) => {
                    exit_with_log!(format!("Error while writing docx file: {:#?}", err));
                }
            }
        }
        ConfigContainer::XCStringsMetadata(m) => {
            match xcstringsdocx::xcstrings_metadata::read::read(m) {
                Ok(_) => {
                    // Don't do anything
                }
                Err(err) => {
                    exit_with_log!(format!("Error while writing docx file: {:#?}", err));
                }
            }
        }
        ConfigContainer::DocxMetadata(d) => xcstringsdocx::docx_metadata::read::read(d),
    }
}

#[derive(Subcommand, Clone, Debug)]
enum ConfigContainer {
    DocxMetadata(xcstringsdocx::docx_metadata::config::Config),
    XCStringsMetadata(xcstringsdocx::xcstrings_metadata::config::Config),
    Read(xcstringsdocx::docx_reader::config::Config),
    Write(xcstringsdocx::docx_writer::config::Config),
}
