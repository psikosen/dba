use std::fmt;
use std::io;

/// Errors that can occur during save operations
#[derive(Debug)]
pub enum SaveError {
    /// Failed to serialize save data to JSON
    SerializationFailed(String),
    /// Failed to create saves directory
    DirectoryCreationFailed(io::Error),
    /// Failed to write save file to disk
    WriteFailed(io::Error),
    /// Save data is invalid or corrupted
    InvalidData(String),
}

impl fmt::Display for SaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SaveError::SerializationFailed(msg) => {
                write!(f, "Failed to serialize save data: {}", msg)
            }
            SaveError::DirectoryCreationFailed(e) => {
                write!(f, "Failed to create saves directory: {}", e)
            }
            SaveError::WriteFailed(e) => {
                write!(f, "Failed to write save file: {}", e)
            }
            SaveError::InvalidData(msg) => {
                write!(f, "Invalid save data: {}", msg)
            }
        }
    }
}

impl std::error::Error for SaveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SaveError::DirectoryCreationFailed(e) | SaveError::WriteFailed(e) => Some(e),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for SaveError {
    fn from(err: serde_json::Error) -> Self {
        SaveError::SerializationFailed(err.to_string())
    }
}

/// Errors that can occur during load operations
#[derive(Debug)]
pub enum LoadError {
    /// Failed to read save file from disk
    ReadFailed(io::Error),
    /// Failed to deserialize JSON data
    DeserializationFailed(String),
    /// Save file not found
    FileNotFound(String),
    /// Save file format is incompatible or corrupted
    IncompatibleFormat(String),
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::ReadFailed(e) => {
                write!(f, "Failed to read save file: {}", e)
            }
            LoadError::DeserializationFailed(msg) => {
                write!(f, "Failed to deserialize save data: {}", msg)
            }
            LoadError::FileNotFound(path) => {
                write!(f, "Save file not found: {}", path)
            }
            LoadError::IncompatibleFormat(msg) => {
                write!(f, "Save file format is incompatible: {}", msg)
            }
        }
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoadError::ReadFailed(e) => Some(e),
            _ => None,
        }
    }
}

impl From<serde_json::Error> for LoadError {
    fn from(err: serde_json::Error) -> Self {
        LoadError::DeserializationFailed(err.to_string())
    }
}
