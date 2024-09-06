mod data;
mod download;
mod error;

pub use self::{download::PdbDownloader, error::*};

#[cfg(test)]
#[allow(unused)]
pub(crate) fn init_tracing() {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
