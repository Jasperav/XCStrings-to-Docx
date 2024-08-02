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

    match cli.command {
        ConfigContainer::Read(r) => xcstringsdocx::docx_reader::convert::read(r),
        ConfigContainer::Write(w) => {
            match xcstringsdocx::docx_writer::convert::convert_from_path(w) {
                Ok(_) => {
                    println!("Successfully wrote the docx file")
                }
                Err(err) => {
                    eprintln!("Error while writing docx file: {:#?}", err);

                    exit(1)
                }
            }
        }
    }
}

#[derive(Subcommand, Clone, Debug)]
enum ConfigContainer {
    Read(xcstringsdocx::docx_reader::config::Config),
    Write(xcstringsdocx::docx_writer::config::Config),
}
