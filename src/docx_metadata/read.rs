use crate::docx_metadata::config::Config;
use crate::error::ConvertError;
use crate::xcstrings_docx_merger::docx_extractor::extract;
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize)]
pub struct Export {
    pub language_code: String,
    pub localized_keys: usize,
    pub translated_status: TranslatedStatus,
}

#[derive(Debug, Clone, Serialize)]
pub enum TranslatedStatus {
    NotYetInXcstrings,
    Translated(usize),
    NoXcstringsFile,
    // If at least one key is not in the .docx files, this is returned
    NoMatchingXcstringsKey(Vec<String>),
}

pub fn read(config: Config) -> Result<Export, ConvertError> {
    let extracted = extract(&config.extract_from_docx)?;
    let translated_status = if let Some(xcstrings) = &config.base_xcstrings {
        // Make sure all keys that are in the .docx files contains at least all keys of the uploaded .xcstrings file
        // else the merging will fail later
        let parsed = swift_localizable_json_parser::parse_from_file(xcstrings)?;
        let all_keys = parsed
            .translation
            .strings
            .iter()
            .map(|s| s.0.to_string())
            .collect::<HashSet<_>>();
        // Also create a hashset here, since plurals will be shown double since it's a vec
        // and that's not what we want, since some languages has more plural rules
        let current_language_keys = extracted
            .extracted
            .iter()
            .map(|e| e.key.to_string())
            .collect::<HashSet<_>>();
        let difference = current_language_keys
            .difference(&all_keys)
            .cloned()
            .collect::<Vec<_>>();

        if difference.is_empty() {
            super::super::xcstrings_metadata::read::extract(xcstrings)?
                .export
                .iter()
                .find(|e| e.language_code == extracted.language_code)
                .map(|export| {
                    TranslatedStatus::Translated(export.localized_keys + export.not_localized_keys)
                })
                .unwrap_or(TranslatedStatus::NotYetInXcstrings)
        } else {
            TranslatedStatus::NoMatchingXcstringsKey(difference)
        }
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

#[cfg(test)]
mod test {
    use crate::docx_metadata::config::Config;
    use crate::docx_metadata::read::TranslatedStatus;
    use std::env::current_dir;

    #[test]
    fn test() {
        let base = current_dir().unwrap().join("resources");
        let export = super::read(Config {
            extract_from_docx: base.join("reader_test_updated_nl.docx"),
            base_xcstrings: Some(base.join("reader_test_base.xcstrings")),
        })
        .unwrap();

        match export.translated_status {
            TranslatedStatus::Translated(_) => {}
            _ => panic!("{:#?}", export),
        }
    }
}
