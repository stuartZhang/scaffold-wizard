use ::std::{error::Error as StdError, fmt::{Debug, Display, Formatter, Result as FmtResult}, io::Error as IoError};
#[derive(Debug)]
pub enum OriginalError {
    Io(IoError)
}
#[derive(Debug)]
pub struct Error {
    message: String,
    original_error: Option<OriginalError>
}
impl Error {
    pub fn new<T>(message: T, original_error: Option<OriginalError>) -> Error
    where T: AsRef<str> {
        Error {
            message: message.as_ref().to_string(),
            original_error
        }
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        if let Some(error) = self.original_error.as_ref() {
            let OriginalError::Io(error) = error;
            return Some(error);
        }
        Error::source(&self)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message)
    }
}
impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::new(format!("{:?}", error), Some(OriginalError::Io(error)))
    }
}
