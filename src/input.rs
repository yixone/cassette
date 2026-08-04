use std::path::{Path, PathBuf};

use tokio::{fs::File, io::AsyncRead};

#[derive(Debug)]
pub struct MediaInput {
    pub(crate) path: PathBuf,
}

impl MediaInput {
    pub fn try_new<P>(path: P) -> std::io::Result<Self>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();

        if !path.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "File not found",
            ));
        }

        Ok(MediaInput { path })
    }
}

/// Media file reader
pub struct CassetteInput {
    pub(crate) reader: Box<dyn AsyncRead + Send + Unpin>,
}

impl CassetteInput {
    /// Opens a reader from the file at the specified path
    pub async fn open_file<P>(path: P) -> Result<CassetteInput, std::io::Error>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path).await?;
        Ok(CassetteInput {
            reader: Box::new(file),
        })
    }

    /// Uses the provided reader as the reader for the media file
    pub fn open_reader<R>(reader: R) -> CassetteInput
    where
        R: AsyncRead + Send + Unpin + 'static,
    {
        CassetteInput {
            reader: Box::new(reader),
        }
    }
}
