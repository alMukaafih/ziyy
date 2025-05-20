#[derive(Debug)]
pub enum ErrorType {
    InvalidTag,
    InvalidTagName,
    InvalidTagProperty,
    InvalidTagValue,
    InvalidTagFormat,
    InvalidTagClose,
    InvalidTagOpen,
    InvalidTagSelfClose,
    InvalidTagPropertyValue,
    InvalidNumber,
    InvalidColor,
    UnexpectedToken,
    UnknownToken,
    UnexpectedEof,
}

pub struct Error {
    pub r#type: ErrorType,
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl Error {
    pub fn new(r#type: ErrorType, message: String, line: usize, column: usize) -> Self {
        Self {
            r#type,
            message,
            line,
            column,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {:?} at line {}, column {}: {}",
            self.r#type, self.line, self.column, self.message
        )
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {:?} at line {}, column {}: {}",
            self.r#type, self.line, self.column, self.message
        )
    }
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
