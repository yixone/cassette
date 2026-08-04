use std::path::PathBuf;

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
