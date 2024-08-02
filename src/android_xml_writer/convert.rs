use crate::android_xml_writer::config::Config;
use crate::error::ConvertError;
use serde::Serialize;
use swift_localizable_json_parser::types::output::{
    AndroidLocalizeConfig, AndroidWriteConfig, WrittenXml,
};

#[derive(Clone, Debug, Serialize)]
pub struct Export {
    pub written_xmls: Vec<WrittenXml>,
}

pub fn write_xmls(config: Config) -> Result<Export, ConvertError> {
    let result = swift_localizable_json_parser::parse_from_file(&config.base_xcstrings)?
        .localizable
        .localized_per_language()
        .localized_for_android(AndroidLocalizeConfig {
            app_name: config.app_name_for_android,
            write_config: Some(AndroidWriteConfig {
                write_in: config.write_in,
                only_write_language_code: None,
            }),
        })?;

    Ok(Export {
        written_xmls: result.written_xmls,
    })
}
