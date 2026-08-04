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
    fn from(_: std::string::FromUtf8Error) -> Self {
        CassetteError::ParseOutputError
    }
}

impl std::fmt::Display for CassetteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for CassetteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CassetteError::Io(error) => Some(error),
            _ => None,
        }
    }
}
