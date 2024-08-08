use crate::xcstrings_metadata::config::Config;
use serde::Serialize;
use std::path::PathBuf;
use swift_localizable_json_parser::types::output::Translation;
use crate::error::ConvertError;

#[derive(Debug, Clone, Serialize)]
pub struct Export {
    pub language_code: String,
    pub word_count: usize,
    pub localized_keys: i32,
    pub not_localized_keys: i32,
}

pub fn read(config: Config) -> Result<Vec<Export>, ConvertError> {
    extract(&config.path_to_xcstrings)
}

pub fn extract(path_to_xcstrings: &PathBuf) -> Result<Vec<Export>, ConvertError> {
    let loc_per_lang = swift_localizable_json_parser::parse_from_dir(path_to_xcstrings)?
        .localizable
        .localized_per_language();
    let mut export = vec![];

    for (language, loc) in loc_per_lang.language_localized {
        let mut localized_keys = 0;
        let mut not_localized_keys = 0;

        macro_rules! update_localize_stats {
            ($string_unit: expr) => {
                if $string_unit.state == "translated" {
                    localized_keys += 1;
                } else {
                    not_localized_keys += 1;
                }
            };
        }

        for single_loc in &loc.translations {
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

        export.push(Export {
            language_code: language,
            word_count: loc.word_count,
            localized_keys,
            not_localized_keys,
        });
    }

    Ok(export)
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
