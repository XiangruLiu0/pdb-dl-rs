# pdb-dl-rs

Rust crate to download PDB by parsing PE file.

## Usage

You can use it as a library in your project.

```rust
use pdb_dl::{PdbDownloader};

#[tokio::main]
async fn main() {
    let pe = PathBuf::from("path/to/pe");
    let out = PathBuf::from("path/to/out");
    let downloader = PdbDownloader::new(pe, out).unwrap();
    downloader.download().await.unwrap();
}
```

Or you can use it as a binary.

```bash
cargo install https://github.com/fre3dm4n/pdb-dl-rs
pdb-dl -i path/to/pe -o path/to/out
```
