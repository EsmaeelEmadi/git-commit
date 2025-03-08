use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ReadFileError {
    CurrentDirError(std::io::Error),
    FileOpenError {
        path: PathBuf,
        source: std::io::Error,
    },
    DeserializeError(serde_yml::Error),
}

impl fmt::Display for ReadFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadFileError::CurrentDirError(e) => {
                write!(f, "Failed to get current directory: {}", e)
            }
            ReadFileError::FileOpenError { path, source } => {
                write!(f, "Failed to open file {:?}: {}", path, source)
            }
            ReadFileError::DeserializeError(e) => {
                write!(f, "Failed to deserialize config: {}", e)
            }
        }
    }
}

impl std::error::Error for ReadFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ReadFileError::CurrentDirError(e) => Some(e),
            ReadFileError::FileOpenError { source, .. } => Some(source),
            ReadFileError::DeserializeError(e) => Some(e),
        }
    }
}
