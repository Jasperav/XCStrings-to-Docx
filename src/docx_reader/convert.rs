use crate::docx_reader::config::Config;
use crate::{KEY_KEY, KEY_VARIATION};
use docx_rust::document::{
    BodyContent, ParagraphContent, RunContent, TableCellContent, TableRowContent,
};
use std::default::Default;

use crate::docx_reader::docx_extractor::extract;
use std::process::exit;
use swift_localizable_json_parser::types::inoutoutput::StringUnitContainer;
use swift_localizable_json_parser::types::input::{Translation, TranslationTypeContainer};
use swift_localizable_json_parser::types::output::PluralVariate;

pub fn read(config: Config) {
    if config.base_xcstrings.exists() {
        println!(
            "xcstrings file exists at path: {:#?}",
            config.base_xcstrings
        );
    } else {
        exit_with_log!(format!(
            "xcstrings file does not exists at path: {:#?}",
            config.base_xcstrings
        ));
    }

    let mut translation: Translation = match std::fs::read(&config.base_xcstrings) {
        Ok(b) => match serde_json::from_slice(&b) {
            Err(err) => {
                exit_with_log!(format!("Got converting xcstrings file to JSON: {:#?}", err));
            }
            Ok(o) => {
                println!("Converted xcstrings file correctly to JSON");

                o
            }
        },
        Err(err) => {
            exit_with_log!(format!(
                "Got error while converting xcstrings file: {:#?}",
                err
            ));
        }
    };

    let mut amount_keys_to_translate = 0;

    let extracted = extract(&config.extract_from_docx);

    for extract in extracted.extracted {
        let language = match translation.strings.get_mut(&extract.key) {
            None => {
                exit_with_log!(format!("There is no matching key for: {}", extract.key));
            }
            Some(v) => v,
        };

        macro_rules! update_string_unit_container {
            ($string_unit: expr) => {
                let translated = extract.translated.trim();

                $string_unit.string_unit.state = if translated.is_empty() {
                    "new".to_string()
                } else {
                    "translated".to_string()
                };
                $string_unit.string_unit.value = translated.to_string();

                amount_keys_to_translate += 1;
            };
        }

        let default_translation_type_container = if extract.variation.is_some() {
            TranslationTypeContainer::Variation(Default::default())
        } else {
            TranslationTypeContainer::StringUnit(Default::default())
        };
        let container_to_work_on = language
            .localizations
            .entry(extracted.language_code.clone())
            .or_insert(default_translation_type_container);

        match container_to_work_on {
            TranslationTypeContainer::StringUnit(su) => {
                if extract.variation.is_some() {
                    exit_with_log!(format!("Expected no variation for key: {}", extract.key));
                }

                update_string_unit_container!(su);
            }
            TranslationTypeContainer::Variation(v) => {
                let variation = match extract.variation {
                    None => {
                        exit_with_log!(format!("Expected variation for key: {}", extract.key));
                    }
                    Some(v) => v,
                };
                let mut container = StringUnitContainer::default();

                update_string_unit_container!(container);

                let field = match variation {
                    PluralVariate::One => &mut v.variations.plural.one,
                    PluralVariate::Two => &mut v.variations.plural.two,
                    PluralVariate::Few => &mut v.variations.plural.few,
                    PluralVariate::Many => &mut v.variations.plural.many,
                    PluralVariate::Other => &mut v.variations.plural.other,
                };

                *field = Some(container);
            }
        }
    }

    println!(
        "Successfully updated Localized file with {amount_keys_to_translate} translated keys, trying to write it back to: {:#?}",
        config.updated_xcstrings
    );

    let json = match serde_json::to_string_pretty(&translation) {
        Ok(ok) => ok,
        Err(err) => {
            exit_with_log!(format!("Error converting to JSON: {:#?}", err));
        }
    };

    match std::fs::write(config.updated_xcstrings, json) {
        Ok(_) => println!("Successfully updated xcstrings file with translations"),
        Err(err) => {
            exit_with_log!(format!(
                "Error while updating the xcstrings file: {:#?}",
                err
            ));
        }
    }
}

pub fn extract_text_from_table_row_content(table_row_content: &TableRowContent) -> String {
    let tc = match table_row_content {
        TableRowContent::TableCell(tc) => tc,
        _ => panic!(),
    };

    assert_eq!(1, tc.content.len());

    let TableCellContent::Paragraph(p) = &tc.content[0];

    let mut string = String::new();

    // For some reason, sometimes it splits up the text
    for content in &p.content {
        let run = match &content {
            ParagraphContent::Run(r) => r,
            _ => panic!(),
        };

        assert_eq!(1, run.content.len());

        match &run.content[0] {
            RunContent::Text(t) => string += t.text.as_ref(),
            _ => panic!(),
        }
    }

    // Trim, I don't think it makes sense to include spaces before/after
    string.trim().to_string()
}

#[cfg(test)]
mod test {
    use crate::docx_reader::config::Config;
    use crate::docx_reader::convert::read;

    use std::env::current_dir;

    #[test]
    fn test_read() {
        let _ = crate::docx_writer::convert::write_generated_docxs();
        let resources = current_dir().unwrap().join("resources");
        let xcstrings = resources.join("reader_test_base.xcstrings");
        let xcstrings_updated = resources.join("reader_test_updated.xcstrings");
        let xcstrings_updated_bytes = std::fs::read(&xcstrings_updated).unwrap();
        let nl = resources.join("reader_test_updated_nl.docx");
        let pl = resources.join("reader_test_updated_pl.docx");

        assert!(resources.exists(), "{:#?}", resources);

        read(Config {
            extract_from_docx: nl,
            base_xcstrings: xcstrings.clone(),
            updated_xcstrings: xcstrings_updated.clone(),
        });

        read(Config {
            extract_from_docx: pl,
            base_xcstrings: xcstrings_updated.clone(),
            updated_xcstrings: xcstrings_updated.clone(),
        });

        let updated = std::fs::read(&xcstrings_updated).unwrap();

        assert_eq!(updated, xcstrings_updated_bytes);
    }
}
