pub mod u8bitfield;
use self::u8bitfield::U8idx;

use std::{fs::File, io::{self, Read, Write}, path::PathBuf};
use easy_ext::ext;

pub type U8bits = u8;
#[ext(U8bitsTrait)]
impl U8bits {
    pub fn get(self, idx: U8idx) -> bool {
        u8bitfield::get(self, idx)
    }
    pub fn set(&mut self, idx: U8idx) {
        u8bitfield::set(self, idx)
    }
}


pub type Bindata = Vec<U8bits>;

#[ext(BindataTrait)]
impl Bindata {
    pub fn from_file(path: &PathBuf) -> io::Result<Bindata> {
        match File::open(path) {
            Ok(mut f) => {
                let mut v = Bindata::new();
                let _ = f.read_to_end(&mut v);
                Ok(v)
            },
            Err(e) => Err(e)
        }
    }
    pub fn to_file(self, path: &PathBuf) -> io::Result<()> {
        match File::create(path) {
            Ok(mut f) => f.write_all(&self),
            Err(e) => Err(e)
        }
    }
}