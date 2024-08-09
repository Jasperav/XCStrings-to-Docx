use crate::docx_metadata::config::Config;
use crate::docx_reader::docx_extractor::extract;
use crate::error::ConvertError;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Export {
    pub language_code: String,
    pub localized_keys: usize,
    pub translated_status: TranslatedStatus,
}

#[derive(Debug, Clone, Serialize)]
pub enum TranslatedStatus {
    NotYetInXcstrings,
    Translated(i32),
    NoXcstringsFile,
}

pub fn read(config: Config) -> Result<Export, ConvertError> {
    let extracted = extract(&config.extract_from_docx)?;
    let translated_status = if let Some(xcstrings) = &config.base_xcstrings {
        // This should always exist though
        super::super::xcstrings_metadata::read::extract(xcstrings)?
            .iter()
            .find(|e| e.language_code == extracted.language_code)
            .map(|export| {
                TranslatedStatus::Translated(export.localized_keys + export.not_localized_keys)
            })
            .unwrap_or(TranslatedStatus::NotYetInXcstrings)
    } else {
        TranslatedStatus::NoXcstringsFile
    };

    Ok(Export {
        language_code: extracted.language_code,
        localized_keys: extracted
            .extracted
            .iter()
            .filter(|e| !e.translated.is_empty())
            .count(),
        translated_status,
    })
}
