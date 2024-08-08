use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: ConfigContainer,
}

fn main() {
    let cli = Cli::parse();

    macro_rules! handle_result {
        ($result: expr) => {{
            match serde_json::to_string(&$result.map_err(|e| e.to_string())) {
                Ok(ok) => println!("{ok}"),
                Err(e) => {
                    eprint!("This is bad: {:#?}", e);

                    exit(1)
                }
            }
        }};
    }

    match cli.command {
        ConfigContainer::Read(c) => handle_result!(xcstringsdocx::docx_reader::convert::read(c)),
        ConfigContainer::Write(c) => {
            handle_result!(xcstringsdocx::docx_writer::convert::convert_from_path(c))
        }
        ConfigContainer::XCStringsMetadata(c) => {
            handle_result!(xcstringsdocx::xcstrings_metadata::read::read(c))
        }
        ConfigContainer::DocxMetadata(c) => {
            handle_result!(xcstringsdocx::docx_metadata::read::read(c))
        }
    };
}

#[derive(Subcommand, Clone, Debug)]
enum ConfigContainer {
    DocxMetadata(xcstringsdocx::docx_metadata::config::Config),
    XCStringsMetadata(xcstringsdocx::xcstrings_metadata::config::Config),
    Read(xcstringsdocx::docx_reader::config::Config),
    Write(xcstringsdocx::docx_writer::config::Config),
}
