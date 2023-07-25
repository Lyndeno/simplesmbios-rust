use std::{env::VarError, error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum MemError {
    Io(io::Error),
    OsStr,
    Var(VarError),
    String(String),
}

impl Display for MemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MemError::Io(..) => write!(f, "IO error"),
            MemError::OsStr => write!(f, "OsStr parsing error"),
            MemError::Var(..) => write!(f, "Error parsing environment variable"),
            MemError::String(ref s) => write!(f, "Error: {}", *s),
        }
    }
}

impl Error for MemError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            MemError::Io(ref e) => Some(e),
            MemError::OsStr => None,
            MemError::Var(ref e) => Some(e),
            MemError::String(..) => None,
        }
    }
}

impl From<io::Error> for MemError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<VarError> for MemError {
    fn from(err: VarError) -> Self {
        Self::Var(err)
    }
}

impl From<String> for MemError {
    fn from(err: String) -> Self {
        Self::String(err)
    }
}
