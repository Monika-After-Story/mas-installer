
use std::io::Error as IOError;
use std::error::Error as StdError;
use std::fmt;

use zip::result::ZipError;
use reqwest::{
    Error as ReqError,
    StatusCode
};
use serde_json::Error as SerdeError;
use rodio::{
    PlayError,
    StreamError,
    DevicesError,
    decoder::DecoderError
};


/// Error type repesenting an error occured during downloading
#[derive(Debug)]
pub enum DownloadError {
    /// Got invalid response/failed to send request
    RequestError(ReqError),
    /// Server failed to provide (valid anyway) content length
    InvalidContentLen,
    /// Server returned invalid status code
    /// while downloading the assets
    InvalidStatusCode(StatusCode),
    /// General IO failure, couldn't write/read
    IOError(IOError)
}

impl From<ReqError> for DownloadError {
    fn from(err: ReqError) -> Self {
        return Self::RequestError(err);
    }
}
impl From<IOError> for DownloadError {
    fn from(err: IOError) -> Self {
        return Self::IOError(err);
    }
}

impl StdError for DownloadError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        return match self {
            Self::RequestError(og_err) => Some(og_err),
            Self::IOError(og_err) => Some(og_err),
            _ => None
        };
    }
}

impl fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::RequestError(err) => {
                write!(f, "failed to request data: {}", err)
            },
            Self::InvalidContentLen => {
                write!(f, "GitHub failed to provide content length")
            }
            Self::InvalidStatusCode(code) => {
                write!(f, "GitHub returned invalid status code: {}", code)
            },
            Self::IOError(err) => {
                write!(f, "failed to read/write data: {}", err)
            }
        };
    }
}


/// Error type repesenting an error occured during extraction
#[derive(Debug)]
pub enum ExtractionError {
    /// An issue with the archive data
    ArchiveError(ZipError),
    /// Unsafe file path in the archive, possible attack?
    UnsafeFilepath(String),
    /// I/O error
    IOError(IOError)
}

impl From<ZipError> for ExtractionError {
    fn from(err: ZipError) -> Self {
        return Self::ArchiveError(err);
    }
}
impl From<IOError> for ExtractionError {
    fn from(err: IOError) -> Self {
        return Self::IOError(err);
    }
}

impl StdError for ExtractionError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        return match self {
            Self::ArchiveError(og_err) => Some(og_err),
            Self::IOError(og_err) => Some(og_err),
            _ => None
        };
    }
}

impl fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::ArchiveError(err) => {
                write!(f, "archive issue: {}", err)
            },
            Self::UnsafeFilepath(_) => {
                write!(f, "found unsafe filepath in archive")
            },
            Self::IOError(err) => {
                write!(f, "failed to read/write data: {}", err)
            }
        };
    }
}


/// The "main" error type that can occur,
/// represents an error occured during installation
#[derive(Debug)]
pub enum InstallerError {
    /// Error occured during downloading
    DownloadError(DownloadError),
    /// JSON is corrupted
    CorruptedJSON(&'static str),
    /// JSON is missing some fields
    InvalidJson(SerdeError),
    /// Got invalid response/failed to send request
    RequestError(ReqError),
    /// General IO failure, couldn't write/read
    IOError(IOError),
    /// Error occured during extraction
    ExtractionError(ExtractionError)
}

impl From<SerdeError> for InstallerError {
    fn from(err: SerdeError) -> Self {
        return Self::InvalidJson(err);
    }
}
impl From<ReqError> for InstallerError {
    fn from(err: ReqError) -> Self {
        return Self::RequestError(err);
    }
}
impl From<IOError> for InstallerError {
    fn from(err: IOError) -> Self {
        return Self::IOError(err);
    }
}
impl From<DownloadError> for InstallerError {
    fn from(err: DownloadError) -> Self {
        return Self::DownloadError(err);
    }
}
impl From<ExtractionError> for InstallerError{
    fn from(err: ExtractionError) -> Self {
        return Self::ExtractionError(err);
    }
}

impl StdError for InstallerError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        return match self {
            Self::DownloadError(og_err) => Some(og_err),
            Self::InvalidJson(og_err) => Some(og_err),
            Self::RequestError(og_err) => Some(og_err),
            Self::IOError(og_err) => Some(og_err),
            Self::ExtractionError(og_err) => Some(og_err),
            _ => None
        };
    }
}

impl fmt::Display for InstallerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::DownloadError(err) => {
                write!(f, "download failed: {}", err)
            },
            Self::CorruptedJSON(info) => {
                write!(f, "failed to parse JSON: {}", info)
            },
            Self::InvalidJson(err) => {
                write!(f, "recieved invalid JSON data from GitHub: {}", err)
            },
            Self::RequestError(err) => {
                write!(f, "failed to request data: {}", err)
            },
            Self::IOError(err) => {
                write!(f, "I/O failure: {}", err)
            },
            Self::ExtractionError(err) => {
                write!(f, "extraction failed: {}", err)
            }
        };
    }
}


/// Error enum for audio related errors
#[derive(Debug)]
pub enum AudioError {
    PlayError(PlayError),
    StreamError(StreamError),
    DevicesError(DevicesError),
    DecoderError(DecoderError)
}

impl From<PlayError> for AudioError {
    fn from(err: PlayError) -> Self {
        return Self::PlayError(err);
    }
}

impl From<StreamError> for AudioError {
    fn from(err: StreamError) -> Self {
        return Self::StreamError(err);
    }
}

impl From<DevicesError> for AudioError {
    fn from(err: DevicesError) -> Self {
        return Self::DevicesError(err);
    }
}

impl From<DecoderError> for AudioError {
    fn from(err: DecoderError) -> Self {
        return Self::DecoderError(err);
    }
}

impl StdError for AudioError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        return match self {
            Self::PlayError(og_err) => Some(og_err),
            Self::StreamError(og_err) => Some(og_err),
            Self::DevicesError(og_err) => Some(og_err),
            Self::DecoderError(og_err) => Some(og_err)
        };
    }
}

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::PlayError(err) => {
                write!(f, "{err}")
            },
            Self::StreamError(err) => {
                write!(f, "{err}")
            },
            Self::DevicesError(err) => {
                write!(f, "{err}")
            },
            Self::DecoderError(err) => {
                write!(f, "{err}")
            }
        };
    }
}
