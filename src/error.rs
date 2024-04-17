use std::io;


#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum Error {
    #[error("File `{0}` could not be read: {1}")]
    FileNotReadable(String, io::Error)
}

#[allow(dead_code)]
impl Error {
    pub fn to_string(self) -> String {
        format!("{}", self)
    }
    pub fn to_stderr(self) {
        eprintln!("{}", self.to_string())
    }
    pub fn to_err<T>(self) -> Result<T, Error> {
        Err(self)
    }
}