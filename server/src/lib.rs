use anyhow::anyhow;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
pub struct Painting {
    pub artist: String,
    pub title: String,
    pub description: String,
}

impl Painting {
    pub fn load_info(id: u16) -> Result<Painting, Error> {
        let path = Path::new("data").join("infos").join(id.to_string());
        let contents = fs::read_to_string(path).map_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => Error::InfoNotFound(id),
            _ => Error::Other(err.into()),
        })?;

        let parts = contents.splitn(3, '\n').collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(anyhow!("failed to parse file {}. It doesn't split to 3 parts").into());
        }

        Ok(Painting {
            artist: parts[0].trim().to_string(),
            title: parts[1].trim().to_string(),
            description: parts[2].trim().to_string(),
        })
    }

    pub fn image_path(id: u16) -> Result<PathBuf, Error> {
        let path = Path::new("data").join("images").join(format!("{}.jpg", id));
        match fs::metadata(&path) {
            Ok(_) => Ok(path),
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => Err(Error::ImageNotFound(id)),
                _ => Err(Error::Other(err.into())),
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("info {0} is not found")]
    InfoNotFound(u16),

    #[error("failed to parse file {0}")]
    FailedToParseInfo(PathBuf),

    #[error("image {0} is not found")]
    ImageNotFound(u16),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
