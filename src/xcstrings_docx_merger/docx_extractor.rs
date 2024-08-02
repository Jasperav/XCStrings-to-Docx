use crate::{KEY_KEY, KEY_VARIATION};
use docx_rust::document::BodyContent;
use std::path::PathBuf;

use crate::error::ConvertError;
use crate::xcstrings_docx_merger::convert::extract_text_from_table_row_content;
use swift_localizable_json_parser::types::output::PluralVariate;

pub struct ExtractContainer {
    pub(crate) language_code: String,
    pub(crate) extracted: Vec<Extract>,
}

pub(crate) struct Extract {
    pub(crate) key: String,
    pub(crate) variation: Option<PluralVariate>,
    pub(crate) translated: String,
}

pub fn extract(extract_from_docx: &PathBuf) -> Result<ExtractContainer, ConvertError> {
    if extract_from_docx.exists() {
        log::debug!("docx file exists...");
    } else {
        exit_with_log!("docx file does not exists");
    }

    let docxfile = match docx_rust::DocxFile::from_file(extract_from_docx) {
        Ok(ok) => {
            log::debug!("Read docx file successfully");

            ok
        }
        Err(err) => {
            exit_with_log!(format!("Got error while reading docx file: {:#?}", err));
        }
    };
    let parsed = match docxfile.parse() {
        Ok(ok) => {
            log::debug!("Parsed docx file successfully");

            ok
        }
        Err(err) => {
            exit_with_log!(format!("Got error while parsing docx file: {:#?}", err));
        }
    };

    let tables = parsed
        .document
        .body
        .content
        .iter()
        .filter_map(|c| match c {
            BodyContent::Table(t) => Some(t),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(1, tables.len());

    let table = tables[0];
    let mut index_key = None;
    let mut index_variation = None;
    let mut language_code = None;
    let index_translated = table.rows[0].cells.len() - 1;

    for (index, header) in table.rows[0].cells.iter().enumerate() {
        let text = extract_text_from_table_row_content(header)?;

        // For some reason, matching does not work
        if text.as_str() == KEY_KEY {
            index_key = Some(index);
        } else if text.as_str() == KEY_VARIATION {
            index_variation = Some(index);
        }

        if index == index_translated {
            language_code = Some(text.to_string());
        }
    }

    let index_key = match index_key {
        None => {
            exit_with_log!("There is no key column");
        }
        Some(index) => index,
    };
    let index_variation = match index_variation {
        None => {
            exit_with_log!("There is no variation column");
        }
        Some(index) => index,
    };
    let language_code = match language_code {
        None => {
            exit_with_log!(
                "There is no language code to translate from, this should be the last column"
            );
        }
        Some(language_code) => language_code,
    };

    let mut extracted = vec![];

    for table_row in table.rows.iter().skip(1) {
        let key = extract_text_from_table_row_content(&table_row.cells[index_key])?;
        let variation_raw = extract_text_from_table_row_content(&table_row.cells[index_variation])?;
        let translated = extract_text_from_table_row_content(&table_row.cells[index_translated])?;

        if key.is_empty() {
            return Err(ConvertError::from(format!(
                "Found empty key, variation: {:#?}, translated value: {:#?}",
                variation_raw, translated
            )));
        }

        if variation_raw.is_empty() {
            return Err(ConvertError::from(format!(
                "Found empty variation, key: {:#?}, translated value: {:#?}",
                key, translated
            )));
        }

        extracted.push(Extract {
            key,
            variation: PluralVariate::from_android_key(&variation_raw),
            translated,
        });
    }

    Ok(ExtractContainer {
        language_code,
        extracted,
    })
}

#[cfg(test)]
mod test {
    use crate::xcstrings_docx_merger::docx_extractor::extract;

    #[test]
    fn test_extract() {
        extract(
            &std::env::current_dir()
                .unwrap()
                .join("generated")
                .join("nl.docx"),
        )
        .unwrap();
    }
}
