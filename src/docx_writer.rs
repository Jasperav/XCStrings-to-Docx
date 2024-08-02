use docx_rust::document::{Paragraph, Table, TableRow};
use docx_rust::Docx;
use swift_localizable_json_parser::types::output::{LocalizedPerLanguage, Translation};
use crate::config::{Config, Converted};
use docx_rust::document::TableRowContent::TableCell;
use anyhow::Result;
use docx_rust::formatting::{OnOffOnlyType, TableHeader, TableRowProperty};
use swift_localizable_json_parser::types::inoutoutput::TranslationValue;

pub(crate) fn convert<'a>(
    localized_per_language: LocalizedPerLanguage,
    config: Config
) -> Result<()> {
    if config.clean_dir_before_generating {
        // Dir does not have to exist
        let _ = std::fs::remove_dir_all(&config.save_in);
    }

    std::fs::create_dir(&config.save_in)?;

    for (language, localization) in localized_per_language.language_localized.clone() {
        if language == config.base_language_code {
            continue;
        }

        let mut docx = Docx::default();
        let mut table = Table::default()
            .push_row(
                TableRow::default()
                    .property(TableRowProperty::default().table_header(OnOffOnlyType::On))
                    .push_cell(Paragraph::default().push_text("key"))
                    .push_cell(Paragraph::default().push_text("variation"))
                    .push_cell(Paragraph::default().push_text("state"))
                    .push_cell(Paragraph::default().push_text(config.base_language_code.clone()))
                    .push_cell(Paragraph::default().push_text(language.clone()))
            );

        for localize in &localization.translations {
            let base_translation = &localized_per_language
                .language_localized
                .get(&config.base_language_code)
                .unwrap()
                .translations
                .iter()
                .find(|l| l.key == localize.key)
                .unwrap()
                .translation;
            let mut table_row = TableRow::default()
                .push_cell(Paragraph::default().push_text(localize.key.clone()));

            match (&localize.translation, base_translation) {
                (Translation::Localization(translation), Translation::Localization(base)) => {
                    table_row = table_row.push_cell(Paragraph::default().push_text("N/A"));

                    table = write_translation(table, table_row, translation, base.value.clone())
                }
                (Translation::PluralVariation(translation), Translation::PluralVariation(base)) => {
                    for pv in translation {
                        let default_base_value = base.iter().map(|b| format!("Variation: {}, translation: {}", b.variate.android_key(), b.translation_value.value)).collect::<Vec<_>>().join("\n");
                        let base_value = base.iter().find(|b| b.variate == pv.variate).map(|pv| pv.translation_value.value.to_string()).unwrap_or(default_base_value);
                        let mut table_row = table_row.clone();

                        table_row = table_row.push_cell(Paragraph::default().push_text(pv.variate.android_key()));

                        table = write_translation(table, table_row, &pv.translation_value, base_value)
                    }
                }
                _ => panic!("Mismatch, this should never happen")
            }
        }

        docx.document.push(table);
        docx.write_file(config.save_in.join(format!("{}.docx", language))).map_err(|f| anyhow::Error::msg( format!("{:#?}", f)))?;
    }

    Ok(())
}

fn write_translation<'a>(
    table: Table<'a>,
    table_row: TableRow<'a>,
    translation_value: &'a TranslationValue,
    base_translation_value: String,
) -> Table<'a> {
    table.push_row(
    table_row
        .push_cell(Paragraph::default().push_text(translation_value.state.as_str()))
        .push_cell(Paragraph::default().push_text(base_translation_value))
        .push_cell(Paragraph::default().push_text(translation_value.value.as_str()).property)
    )
}