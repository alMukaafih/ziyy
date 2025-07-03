use crate::Span;

/// Represents the various types of errors that can occur.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorType {
    /// Indicates an invalid tag was encountered.
    InvalidTag,
    /// Indicates an invalid tag name was encountered.
    InvalidTagName,
    /// Indicates an invalid value for a tag property was encountered.
    InvalidTagPropertyValue,
    /// Indicates an invalid number was encountered.
    InvalidNumber,
    /// Indicates an invalid color was encountered.
    InvalidColor,
    /// Indicates an unexpected token was encountered.
    UnexpectedToken,
    /// Indicates the end of input was reached unexpectedly.
    UnexpectedEof,
}

/// Represents an error with additional context such as its type, message, and location.
pub struct Error {
    /// The type of the error.
    pub r#type: ErrorType,
    /// A descriptive message providing more details about the error.
    pub message: String,
    /// The span in the source where the error occurred.
    pub span: Span,
}

impl Error {
    /// Creates a new `Error` instance.
    ///
    /// # Arguments
    ///
    /// * `r#type` - The type of the error.
    /// * `message` - A descriptive message providing more details about the error.
    /// * `span` - The span in the source where the error occurred.
    ///
    /// # Returns
    ///
    /// A new `Error` instance.
    pub fn new(r#type: ErrorType, message: String, span: Span) -> Self {
        Self {
            r#type,
            message,
            span,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error: {:?} at span {:?}: {}",
            self.r#type, self.span, self.message
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
            "Error: {:?} at line {:?}: {}",
            self.r#type, self.span, self.message
        )
    }
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// A type alias for results that return an `Error` on failure.
pub type Result<T> = std::result::Result<T, Error>;
