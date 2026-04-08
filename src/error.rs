pub type Result<T> = core::result::Result<T, Error>;

pub struct Error {
    inner: Box<ErrorKind>,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            inner: Box::new(kind),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::new(ErrorKind::StdIoError(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::new(ErrorKind::SerdeJsonError(e))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::new(ErrorKind::PraseIntError(e))
    }
}
pub enum ErrorKind {
    OsynicPadError(String),
    PraseIntError(std::num::ParseIntError),
    SerdeJsonError(serde_json::Error),
    StdIoError(std::io::Error),
}

impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::OsynicPadError(e) => write!(f, "OsynicPadError: {}", e),
            ErrorKind::PraseIntError(e) => write!(f, "PraseIntError: {}", e),
            ErrorKind::SerdeJsonError(e) => write!(f, "SerdeJsonError: {}", e),
            ErrorKind::StdIoError(e) => write!(f, "StdIoError: {}", e),
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::OsynicPadError(e) => write!(f, "OsynicPadError: {}", e),
            ErrorKind::PraseIntError(e) => write!(f, "PraseIntError: {}", e),
            ErrorKind::SerdeJsonError(e) => write!(f, "SerdeJsonError: {}", e),
            ErrorKind::StdIoError(e) => write!(f, "StdIoError: {}", e),
        }
    }
}
