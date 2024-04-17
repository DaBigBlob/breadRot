use std::io;


#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum Error {
    #[error("File `{0}` could not be read: {1}")]
    FileNotReadable(String, io::Error)
}

impl Error {
    pub fn say(self) {
        eprintln!("{}", self)
    }
}