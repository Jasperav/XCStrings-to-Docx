use clap::{Parser, Subcommand};
use std::process::exit;
use xcstringsdocx::{
    android_xml_writer, docx_metadata, docx_writer, xcstrings_docx_merger, xcstrings_metadata, extension_determiner
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: ConfigContainer,
}

fn main() {
    env_logger::init();

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
        ConfigContainer::XCStringsDocxMerger(c) => {
            handle_result!(xcstrings_docx_merger::convert::merge(c))
        }
        ConfigContainer::DocxFromXCStrings(c) => {
            handle_result!(docx_writer::convert::convert_from_path(c))
        }
        ConfigContainer::XCStringsMetadata(c) => {
            handle_result!(xcstrings_metadata::read::read(c))
        }
        ConfigContainer::DocxMetadata(c) => {
            handle_result!(docx_metadata::read::read(c))
        }
        ConfigContainer::AndroidXmlWriter(c) => {
            handle_result!(android_xml_writer::convert::write_xmls(c))
        }
        ConfigContainer::ExtensionDeterminer(c) => {
            handle_result!(extension_determiner::extension_determiner(c))
        }
    };
}

#[derive(Subcommand, Clone, Debug)]
enum ConfigContainer {
    DocxMetadata(docx_metadata::config::Config),
    XCStringsMetadata(xcstrings_metadata::config::Config),
    XCStringsDocxMerger(xcstrings_docx_merger::config::Config),
    DocxFromXCStrings(docx_writer::config::Config),
    AndroidXmlWriter(android_xml_writer::config::Config),
    ExtensionDeterminer(extension_determiner::Config),
}
