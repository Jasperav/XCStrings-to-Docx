use docx_rust::document::{ParagraphContent, RunContent, TableCellContent, TableRowContent};
use std::default::Default;

use crate::error::ConvertError;
use crate::xcstrings_docx_merger::config::Config;
use crate::xcstrings_docx_merger::docx_extractor::extract;
use serde::Serialize;
use serde_json_fmt::JsonFormat;
use swift_localizable_json_parser::types::inoutoutput::StringUnitContainer;
use swift_localizable_json_parser::types::input::TranslationTypeContainer;
use swift_localizable_json_parser::types::output::PluralVariate;
use swift_localizable_json_parser::{NEW_STATE, TRANSLATED_STATE};

#[derive(Clone, Debug, Serialize)]
pub struct Export {
    pub keys_translated: i32,
    pub keys_to_translate: i32,
}

pub fn merge(config: Config) -> Result<Export, ConvertError> {
    if config.base_xcstrings.exists() {
        log::debug!(
            "xcstrings file exists at path: {:#?}",
            config.base_xcstrings
        );
    } else {
        exit_with_log!(format!(
            "xcstrings file does not exists at path: {:#?}",
            config.base_xcstrings
        ));
    }

    let mut parsed = swift_localizable_json_parser::parse_from_file(&config.base_xcstrings)?;
    let mut keys_translated = 0;
    let mut keys_to_translate = 0;
    let extracted = extract(&config.extract_from_docx)?;

    for extract in extracted.extracted {
        let language = match parsed.translation.strings.get_mut(&extract.key) {
            None => {
                exit_with_log!(format!("There is no matching key for: {}", extract.key));
            }
            Some(v) => v,
        };

        macro_rules! update_string_unit_container {
            ($string_unit: expr) => {
                let translated = extract.translated.trim();

                $string_unit.string_unit.state = if translated.is_empty() {
                    keys_to_translate += 1;

                    NEW_STATE.to_string()
                } else {
                    keys_translated += 1;

                    TRANSLATED_STATE.to_string()
                };
                $string_unit.string_unit.value = translated.to_string();
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
                    PluralVariate::Zero => &mut v.variations.plural.zero,
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

    log::debug!(
        "Successfully updated Localized file with {keys_translated} translated keys and {keys_to_translate} keys to translate, trying to write it back to: {:#?}",
        config.updated_xcstrings
    );

    // Apple for some reason adds a space before the colon, this is the reason we use another crate
    let json = JsonFormat::pretty()
        .colon(" : ")
        .unwrap() // This is fine, should always work
        .format_to_string(&parsed.translation)?; // This as well, but who knows...

    std::fs::write(&config.updated_xcstrings, json)?;

    Ok(Export {
        keys_translated,
        keys_to_translate,
    })
}

pub fn extract_text_from_table_row_content(
    table_row_content: &TableRowContent,
) -> Result<String, ConvertError> {
    let tc = match table_row_content {
        TableRowContent::TableCell(tc) => tc,
        _ => return Err(ConvertError::corrupted_docx_file()),
    };

    let mut string = vec![];

    for content in &tc.content {
        let mut inner_string = String::new();
        let TableCellContent::Paragraph(p) = content;

        // For some reason, sometimes it splits up the text
        for content in &p.content {
            let run = match &content {
                ParagraphContent::Run(r) => r,
                ParagraphContent::CommentRangeStart(_)
                | ParagraphContent::CommentRangeEnd(_)
                | ParagraphContent::BookmarkStart(_)
                | ParagraphContent::BookmarkEnd(_) => continue,
                ParagraphContent::Link(l) => &l.content,
            };

            if run.content.is_empty() {
                inner_string += "\n";
            } else {
                for content in &run.content {
                    match &content {
                        RunContent::Text(t) => inner_string += t.text.as_ref(),
                        _ => continue, // Ignore, this could be anything word added
                    }
                }
            }
        }

        string.push(inner_string)
    }

    // If no trim is added, for some reason another space is added, at least when reading out the headers
    let read = string.join("\n").trim().to_string();

    Ok(read)
}

#[cfg(test)]
mod test {
    use crate::xcstrings_docx_merger::config::Config;
    use crate::xcstrings_docx_merger::convert::merge;
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

        merge(Config {
            extract_from_docx: nl,
            base_xcstrings: xcstrings.clone(),
            updated_xcstrings: xcstrings_updated.clone(),
        })
        .unwrap();

        merge(Config {
            extract_from_docx: pl,
            base_xcstrings: xcstrings_updated.clone(),
            updated_xcstrings: xcstrings_updated.clone(),
        })
        .unwrap();

        super::super::super::android_xml_writer::convert::write_xmls(
            super::super::super::android_xml_writer::config::Config {
                base_xcstrings: xcstrings_updated.clone(),
                write_in: xcstrings_updated.parent().unwrap().to_path_buf(),
                app_name_for_android: "test_app".to_string(),
            },
        )
        .unwrap();

        let updated = std::fs::read(&xcstrings_updated).unwrap();

        assert_eq!(
            String::from_utf8(updated).unwrap(),
            String::from_utf8(xcstrings_updated_bytes).unwrap()
        );
    }
}
