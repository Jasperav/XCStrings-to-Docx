use docx_rust::DocxError;
use std::error::Error;
use std::fmt::{Display, Formatter};
use swift_localizable_json_parser::types::output::ParsedError;

#[derive(Debug, Clone)]
pub enum ConvertError {
    Wrapped(String),
}

impl ConvertError {
    pub fn corrupted_docx_file() -> Self {
        ConvertError::from("Corrupted .docx file")
    }
}

impl From<String> for ConvertError {
    fn from(value: String) -> Self {
        debug_assert!(false, "{value}");

        ConvertError::Wrapped(value)
    }
}

impl From<&str> for ConvertError {
    fn from(value: &str) -> Self {
        ConvertError::from(value.to_string())
    }
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // Replace all newlines so it's always on 1 line
            ConvertError::Wrapped(error) => {
                write!(f, "Error occurred: {}", error.replace('\n', " "))
            }
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
        debug_assert!(false, "{:#?}", value);

        ConvertError::Wrapped(value.to_string())
    }
}

impl From<std::io::Error> for ConvertError {
    fn from(value: std::io::Error) -> Self {
        debug_assert!(false);

        ConvertError::Wrapped(value.to_string())
    }
}

impl From<serde_json::Error> for ConvertError {
    fn from(value: serde_json::Error) -> Self {
        debug_assert!(false);

        ConvertError::Wrapped(value.to_string())
    }
}

impl From<DocxError> for ConvertError {
    fn from(value: DocxError) -> Self {
        debug_assert!(false);

        ConvertError::Wrapped(format!("{:#?}", value))
    }
}
