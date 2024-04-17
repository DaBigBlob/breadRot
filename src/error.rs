use std::io;


#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum Error {
    #[error("File could not be read: `{0}`")]
    FileNotReadable(String, io::Error)
}

impl Error {
    pub fn say(self) {
        eprintln!("{}", self)
    }
}