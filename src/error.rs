use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum PdbDownloadError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("http error: {0}")]
    Http(u16),
    #[error("not a PE file: {0}")]
    NotPe(PathBuf),
    #[error("load PE error: {0}")]
    LoadPe(#[from] goblin::error::Error),
    #[error("no PDB info found")]
    NoPdbInfo,
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type PdbDownloadResult<T> = Result<T, PdbDownloadError>;
