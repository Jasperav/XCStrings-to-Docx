use std::error::Error;
use std::fmt::{Display, Formatter};
use docx_rust::DocxError;
use swift_localizable_json_parser::types::output::ParsedError;

#[derive(Debug, Clone)]
pub enum ConvertError {
    Wrapped(String),
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // Replace all newlines so it's always on 1 line
            ConvertError::Wrapped(error) => write!(f, "Error occurred: {}", error.replace("\n", " ")),
        }
    }
}

impl Error for ConvertError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<ParsedError> for ConvertError {
    fn from(value: ParsedError) -> Self {
        ConvertError::Wrapped(value.to_string())
    }
}

impl From<std::io::Error> for ConvertError {
    fn from(value: std::io::Error) -> Self {
        ConvertError::Wrapped(value.to_string())
    }
}

impl From<DocxError> for ConvertError {
    fn from(value: DocxError) -> Self {
        ConvertError::Wrapped(format!("{:#?}", value))
    }
}