use crate::docx_writer::{config, convert};

pub fn convert_from_path(config: config::Config) -> anyhow::Result<()> {
    convert_from_raw(&std::fs::read(&config.path_to_xcstrings)?, config)
}

pub fn convert_from_raw(xcstrings: &[u8], config: config::Config) -> anyhow::Result<()> {
    convert(
        swift_localizable_json_parser::parse_from_bytes(xcstrings),
        config,
    )
}

#[cfg(debug_assertions)]
pub fn write_generated_docxs() -> std::path::PathBuf {
    let base = std::env::current_dir().unwrap();

    assert_eq!("xcstringsdocx", base.file_name().unwrap().to_str().unwrap());

    let base = base.join("generated");
    let raw = include_bytes!("../../resources/reader_test_base.xcstrings");

    convert_from_raw(
        raw,
        crate::docx_writer::config::Config {
            save_in: base.clone(),
            clean_dir_before_generating: true,
            new_language_codes: vec!["pl".to_string()],
            columns_in_output: vec![crate::docx_writer::config::Column::State],
            path_to_xcstrings: Default::default(),
        },
    )
    .unwrap();

    base
}

#[cfg(test)]
mod test {
    use crate::docx_writer::convert::write_generated_docxs;

    #[test]
    fn test_convert() {
        write_generated_docxs();
    }
}
