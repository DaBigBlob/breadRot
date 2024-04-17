use std::{fs::File, io::{self, Read, Write}, path::PathBuf};
use easy_ext::ext;

type Bindata = Vec<u8>;

#[ext]
pub impl Bindata {
    fn from_file(path: PathBuf) -> io::Result<Bindata> {
        match File::open(path) {
            Ok(mut f) => {
                let mut v = Bindata::new();
                let _ = f.read_to_end(&mut v);
                Ok(v)
            },
            Err(e) => Err(e)
        }
    }
    fn to_file(self, path: PathBuf) -> io::Result<()> {
        match File::create(path) {
            Ok(mut f) => f.write_all(&self),
            Err(e) => Err(e)
        }
    }
}