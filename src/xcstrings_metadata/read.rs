use crate::error::ConvertError;
use crate::xcstrings_metadata::config::Config;
use serde::Serialize;
use std::collections::HashSet;
use std::path::PathBuf;
use swift_localizable_json_parser::types::output::Translation;
use swift_localizable_json_parser::TRANSLATED_STATE;

#[derive(Debug, Clone, Serialize)]
pub struct ExportContainer {
    pub export: Vec<Export>,
    pub base_language: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Export {
    pub language_code: String,
    pub word_count: usize,
    pub localized_keys: usize,
    pub not_localized_keys: usize,
}

pub fn read(config: Config) -> Result<ExportContainer, ConvertError> {
    extract(&config.path_to_xcstrings)
}

pub fn extract(path_to_xcstrings: &PathBuf) -> Result<ExportContainer, ConvertError> {
    let parsed = swift_localizable_json_parser::parse_from_file(path_to_xcstrings)?;
    let loc_per_lang = parsed.localizable.localized_per_language();
    let mut export = vec![];

    // This does not take into account plurals
    let all_keys = parsed
        .translation
        .strings
        .iter()
        .map(|s| s.0.to_string())
        .collect::<HashSet<_>>();

    for (language, loc) in loc_per_lang.clone().language_localized {
        let mut localized_keys = 0;
        let mut not_localized_keys = 0;
        let mut all_keys_current_language = all_keys.clone();

        macro_rules! update_localize_stats {
            ($string_unit: expr) => {
                // There is a bug in the .xcstrings file that it doesn't update the state of the base language
                // so skip the source language, that should always be 100% up to date
                if $string_unit.state == TRANSLATED_STATE || language == parsed.localizable.source_language {
                    localized_keys += 1;
                } else {
                    not_localized_keys += 1;
                }
            };
        }

        for single_loc in &loc.translations {
            if !all_keys_current_language.remove(&single_loc.key_raw) {
                return Err(ConvertError::Wrapped(format!(
                    "No key found for: {}",
                    single_loc.key_raw
                )));
            }

            match &single_loc.translation {
                Translation::Localization(l) => {
                    update_localize_stats!(l)
                }
                Translation::PluralVariation(pv) => {
                    for pv in pv {
                        update_localize_stats!(pv.translation_value);
                    }
                }
            }
        }

        for key in all_keys_current_language {
            // The key in strings MUST exist, however, the base language isn't mandatory if the translation is inlined
            match loc_per_lang
                .language_localized
                .get(&parsed.translation.source_language)
            {
                None => not_localized_keys += 1,
                Some(loc) => match &loc
                    .translations
                    .iter()
                    .find(|t| t.key_raw == key)
                    .ok_or(ConvertError::Wrapped(format!(
                        "Key not found in base language: {}",
                        key
                    )))?
                    .translation
                {
                    Translation::Localization(_) => not_localized_keys += 1,
                    Translation::PluralVariation(pv) => not_localized_keys += pv.len(),
                },
            }
        }

        export.push(Export {
            language_code: language,
            word_count: loc.word_count,
            localized_keys,
            not_localized_keys,
        });
    }

    Ok(ExportContainer {
        export,
        base_language: parsed.translation.source_language,
    })
}

#[cfg(test)]
mod test {
    use crate::xcstrings_metadata::config::Config;
    use std::env::current_dir;

    #[test]
    fn test_read() {
        super::read(Config {
            path_to_xcstrings: current_dir()
                .unwrap()
                .join("resources")
                .join("reader_test_base.xcstrings"),
        })
        .unwrap();
    }
}
