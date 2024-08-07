use crate::docx_metadata::config::Config;
use crate::docx_reader::docx_extractor::extract;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Export {
    pub language_code: String,
    pub localized_keys: usize,
    pub total_keys_to_translate: Option<i32>,
}

pub fn read(config: Config) {
    let extracted = extract(&config.extract_from_docx);
    let total_keys_to_translate = if let Some(xcstrings) = &config.base_xcstrings {
        // This should always exist though
        super::super::xcstrings_metadata::read::extract(xcstrings)
            .iter()
            .find(|e| e.language_code == extracted.language_code).map(|export| export.localized_keys + export.not_localized_keys)
    } else {
        None
    };
    let export = Export {
        language_code: extracted.language_code,
        localized_keys: extracted
            .extracted
            .iter()
            .filter(|e| !e.translated.is_empty())
            .count(),
        total_keys_to_translate,
    };

    let transformed = serde_json::to_string(&export).unwrap();

    println!("Exported result: {transformed}");
}
