#[derive(Debug)]
pub enum CassetteError {
    FfmpegNotInstalled,
    ParseOutputError,
    UnsupportedFormat,
    FileAlreadyExists,
    Io(std::io::Error),
}

impl From<std::io::Error> for CassetteError {
    fn from(e: std::io::Error) -> Self {
        CassetteError::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CassetteError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        CassetteError::ParseOutputError
    }
}
