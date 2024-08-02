use crate::docx_writer::config;
use crate::error::ConvertError;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Export {
    pub amount_keys_to_translate: i32,
    pub language_code: String,
    pub file_name: String,
}

pub fn convert_from_path(config: config::Config) -> Result<Vec<Export>, ConvertError> {
    let read = std::fs::read(&config.path_to_xcstrings)?;

    convert_from_raw(&read, config)
}

pub fn convert_from_raw(
    xcstrings: &[u8],
    config: config::Config,
) -> Result<Vec<Export>, ConvertError> {
    let converted = convert(
        swift_localizable_json_parser::parse_from_bytes(xcstrings)?,
        config,
    )?;

    Ok(converted)
}

use docx_rust::document::{Paragraph, RunContent, Table, TableCell, TableRow, Text, TextSpace};
use docx_rust::formatting::{
    Bold, BoldComplex, BorderStyle, BottomBorder, CharacterProperty, InsideHorizonBorder,
    InsideVerticalBorder, LeftBorder, OnOffOnlyType, ParagraphProperty, RightBorder, TableBorders,
    TableProperty, TableRowProperty, TopBorder,
};

use docx_rust::Docx;

use docx_rust::document::ParagraphContent::Run;
use std::collections::HashSet;
use swift_localizable_json_parser::NEW_STATE;

use crate::docx_writer::config::{Column, Config};
use crate::{KEY_KEY, KEY_VARIATION, KEY_COMMENT};
use swift_localizable_json_parser::types::output::{Parsed, Translation};

pub fn convert(
    localizable: Parsed,
    config: Config,
) -> std::result::Result<Vec<Export>, ConvertError> {
    let localized_per_language = localizable.localizable.localized_per_language();

    if config.clean_dir_before_generating {
        // Dir does not have to exist
        let _ = std::fs::remove_dir_all(&config.save_in);
    }

    // Ignore any error
    let _ = std::fs::create_dir(&config.save_in);

    let base_language = &localizable.translation.source_language;
    let base_translation = localized_per_language
        .language_localized
        .get(base_language)
        .unwrap()
        .translations
        .clone();
    let mut languages_to_write_docx_files = localized_per_language
        .language_localized
        .keys()
        .map(|k| k.to_string())
        .collect::<HashSet<_>>();

    languages_to_write_docx_files.extend(config.new_language_codes);

    assert!(languages_to_write_docx_files.remove(base_language));

    let mut exports = vec![];

    for language_to_write in languages_to_write_docx_files {
        log::debug!("Writing language: {language_to_write}");

        let mut amount_keys_to_translate = 0;
        let table_border_style = BorderStyle::Single;
        let table_border_size = 4isize;
        let mut table_borders = TableBorders::default()
            .top(
                TopBorder::default()
                    .style(table_border_style.clone())
                    .size(table_border_size),
            )
            .bottom(
                BottomBorder::default()
                    .style(table_border_style.clone())
                    .size(table_border_size),
            );

        table_borders.left = Some(
            LeftBorder::default()
                .style(table_border_style.clone())
                .size(table_border_size),
        );
        table_borders.right = Some(
            RightBorder::default()
                .style(table_border_style.clone())
                .size(table_border_size),
        );
        table_borders.inside_horizon = Some(
            InsideHorizonBorder::default()
                .style(table_border_style.clone())
                .size(table_border_size),
        );
        table_borders.inside_vertical = Some(
            InsideVerticalBorder::default()
                .style(table_border_style.clone())
                .size(table_border_size),
        );

        let mut docx = Docx::default();
        let mut character_property = CharacterProperty::default().bold(Bold::default());

        character_property.bold_complex = Some(BoldComplex::default());

        let paragraph_property = ParagraphProperty {
            r_pr: vec![character_property.clone()],
            ..Default::default()
        };

        let mut header_paragraph = Paragraph::default().property(paragraph_property.clone());

        header_paragraph.content.push(Run(
            docx_rust::document::Run::default().property(character_property.clone())
        ));

        macro_rules! add_header {
            ($text: expr) => {{
                let mut cloned = header_paragraph.clone();

                cloned.content.push(Run(docx_rust::document::Run {
                    content: vec![RunContent::Text($text.into())],
                    property: Some(character_property.clone()),
                    ..Default::default()
                }));

                cloned
            }};
        }

        let mut base_table_row = TableRow::default()
            .property(TableRowProperty::default().table_header(OnOffOnlyType::On))
            .push_cell(add_header!(KEY_KEY))
            .push_cell(add_header!(KEY_COMMENT))
            .push_cell(add_header!(KEY_VARIATION));

        if config.columns_in_output.contains(&Column::State) {
            base_table_row = base_table_row.push_cell(add_header!("State"));
        }

        base_table_row = base_table_row
            .push_cell(add_header!(base_language.to_string()))
            .push_cell(add_header!(language_to_write.clone()));

        let mut table = Table::default()
            .property(TableProperty::default().borders(table_borders))
            .push_row(base_table_row);

        macro_rules! write_translation {
            ($table_row: expr, $translation_value: expr, $base_translation_value: expr) => {{
                let state = $translation_value
                    .clone()
                    .map(|t| t.state)
                    .unwrap_or(NEW_STATE.to_string());
                let value = $translation_value
                    .map(|t| t.value)
                    .unwrap_or("".to_string());

                let mut new_table_row = $table_row;

                if config.columns_in_output.contains(&Column::State) {
                    new_table_row = new_table_row.push_cell(Paragraph::default().push_text(state));
                }

                new_table_row = new_table_row
                    .push_cell(create_table_cell($base_translation_value))
                    .push_cell(create_table_cell(&value));

                table = table.push_row(new_table_row);

                amount_keys_to_translate += 1;
            }};
        }

        let localization_language_to = localized_per_language
            .language_localized
            .get(&language_to_write)
            .cloned()
            .unwrap_or_default();
        for localized_base_translation in &base_translation {
            let existing_translation = localization_language_to
                .translations
                .iter()
                .find(|t| t.key_raw == localized_base_translation.key_raw)
                .map(|t| t.translation.clone());

            let table_row = TableRow::default()
                .push_cell(create_table_cell(&localized_base_translation.key_raw))
                .push_cell(create_table_cell(&localized_base_translation.comment));

            match &localized_base_translation.translation {
                Translation::Localization(base) => {
                    let table_row = table_row
                        .clone()
                        .push_cell(Paragraph::default().push_text("N/A"));
                    let existing = existing_translation.map(|e| e.expect_localization());

                    write_translation!(table_row, existing, &base.value)
                }
                Translation::PluralVariation(base) => {
                    // First handle the default plural variations from the base language
                    for pv in base {
                        let table_row = table_row
                            .clone()
                            .push_cell(Paragraph::default().push_text(pv.variate.android_key()));

                        let existing = existing_translation.clone().and_then(|et| {
                            et.expect_plural_variation()
                                .iter()
                                .find(|existing| existing.variate == pv.variate)
                                .map(|pv| pv.translation_value.clone())
                        });

                        write_translation!(table_row, existing, &pv.translation_value.value)
                    }

                    // It could also be that the language already has other plural variations than the base language, check that
                    if let Some(e) = existing_translation {
                        for pv in e.expect_plural_variation() {
                            if base.iter().any(|p| p.variate == pv.variate) {
                                // Already a variation for it
                                continue;
                            }

                            let mut table_row = table_row.clone();

                            table_row = table_row.push_cell(
                                Paragraph::default().push_text(pv.variate.android_key()),
                            );

                            write_translation!(table_row, Some(pv.translation_value.clone()), &"");
                        }
                    }
                }
            }
        }

        docx.document.push(table);

        let file_name = format!("{}.docx", language_to_write);
        let write_to = config.save_in.join(&file_name);
        let _ = std::fs::remove_file(&write_to);

        docx.write_file(write_to)?;

        log::debug!(
            "Exported {amount_keys_to_translate} translations for language: {language_to_write}"
        );

        exports.push(Export {
            amount_keys_to_translate,
            language_code: language_to_write,
            file_name,
        })
    }

    Ok(exports)
}

fn create_table_cell<T: ToString>(text: T) -> TableCell<'static> {
    let mut table_cell = TableCell::default();
    let text = text.to_string();
    let split = text.split('\n');

    for split in split {
        let text = Text {
            space: if split.trim() == split {
                None
            } else {
                Some(TextSpace::Preserve)
            },
            text: split.to_string().into(),
        };

        table_cell
            .content
            .push(Paragraph::default().push_text(text).into());
    }

    table_cell
}

#[cfg(debug_assertions)]
pub fn write_generated_docxs() -> std::path::PathBuf {
    let base = std::env::current_dir().unwrap();

    assert_eq!(
        "XCStrings-to-Docx",
        base.file_name().unwrap().to_str().unwrap()
    );

    let base = base.join("generated");
    let raw = include_bytes!("../../resources/reader_test_base.xcstrings");
    let expect_nl = include_bytes!("../../resources/reader_test_base_nl.docx");
    let expect_pl = include_bytes!("../../resources/reader_test_base_pl.docx");

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

    macro_rules! read {
        ($lang: expr, $lang_expect: expr) => {
            let read = std::fs::read(base.join(format!("{}.docx", $lang))).unwrap();

            assert_eq!($lang_expect, read.as_slice());
        };
    }

    read!("nl", expect_nl);
    read!("pl", expect_pl);

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
