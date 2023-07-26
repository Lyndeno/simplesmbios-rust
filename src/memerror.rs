use std::{env::VarError, error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum SMBiosError {
    Io(io::Error),
    OsStr,
    Var(VarError),
    String(String),
}

impl Display for SMBiosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SMBiosError::Io(..) => write!(f, "IO error"),
            SMBiosError::OsStr => write!(f, "OsStr parsing error"),
            SMBiosError::Var(..) => write!(f, "Error parsing environment variable"),
            SMBiosError::String(ref s) => write!(f, "Error: {}", *s),
        }
    }
}

impl Error for SMBiosError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            SMBiosError::Io(ref e) => Some(e),
            SMBiosError::OsStr => None,
            SMBiosError::Var(ref e) => Some(e),
            SMBiosError::String(..) => None,
        }
    }
}

impl From<io::Error> for SMBiosError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<VarError> for SMBiosError {
    fn from(err: VarError) -> Self {
        Self::Var(err)
    }
}

impl From<String> for SMBiosError {
    fn from(err: String) -> Self {
        Self::String(err)
    }
}
