use goblin::Object;
use reqwest::{Client, Url};
use std::{
    ffi::CStr,
    path::{Path, PathBuf},
};
use tokio::io::AsyncWriteExt;
use tracing::{error, info};

use crate::{data::*, error::*};

const MICROSOFT_SYMBOL_STORE_URL: &str = "https://msdl.microsoft.com/download/symbols/";

pub struct PdbDownloader {
    guid: PdbGuid,
    filename: String,
    age: u32,
    out_path: PathBuf,
}

impl PdbDownloader {
    pub fn new<P: AsRef<Path>, O: AsRef<Path>>(pe_path: P, out_path: O) -> PdbDownloadResult<Self> {
        let path = pe_path.as_ref();
        let buffer = std::fs::read(path).inspect_err(|e| {
            error!(?e, path = ?path.display(), "failed to read PE file");
        })?;
        let obj = Object::parse(&buffer).inspect_err(|e| {
            error!(?e, path = ?path.display(), "failed to parse PE file");
        })?;
        let pe = if let Object::PE(pe) = obj {
            pe
        } else {
            return Err(PdbDownloadError::NotPe(path.to_path_buf()));
        };
        info!("PE loaded, trying to extract PDB info");
        let pdb = pe
            .debug_data
            .ok_or(PdbDownloadError::NoPdbInfo)?
            .codeview_pdb70_debug_info
            .ok_or(PdbDownloadError::NoPdbInfo)?;
        let guid = PdbGuid::from(pdb.signature);
        let filename = CStr::from_bytes_until_nul(pdb.filename)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        let age = pdb.age;
        Ok(PdbDownloader {
            guid,
            filename,
            age,
            out_path: out_path.as_ref().to_owned(),
        })
    }

    pub async fn download(&self) -> PdbDownloadResult<()> {
        let Self {
            guid,
            filename,
            age,
            out_path,
        } = self;
        let client = Client::new();

        let url = Url::parse(MICROSOFT_SYMBOL_STORE_URL).unwrap();
        let url = url
            .join(&format!("{filename}/{guid}{age}/{filename}"))
            .unwrap();
        info!(url = url.to_string(), "Trying to download PDB");
        let mut response = client.get(url).send().await.inspect_err(|e| {
            error!(?e, "Failed to download PDB");
        })?;
        if !response.status().is_success() {
            return Err(PdbDownloadError::Http(response.status().as_u16()));
        }

        let mut file = tokio::fs::File::create(out_path).await.inspect_err(|e| {
            error!(?e, path = ?out_path.display(), "Failed to create PDB file");
        })?;
        while let Some(chunk) = response.chunk().await.inspect_err(|e| {
            error!(?e, "Failed to read bytes during download");
        })? {
            file.write_all(&chunk).await.inspect_err(|e| {
                error!(?e, path = ?out_path.display(), "Failed to write PDB bytes");
            })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let pe = root.join("tests/ntoskrnl.exe");
        let out_path = root.join("tests/ntoskrnl.pdb");
        let downloader = PdbDownloader::new(pe, &out_path).unwrap();
        downloader.download().await.unwrap();
        // delete the file after the test
        std::fs::remove_file(out_path).unwrap();
    }
}
