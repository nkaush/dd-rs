use std::path::PathBuf;
use std::{io, fs};

// Publicly use Read so Trait Impl is automatically included
pub use std::io::{Read, Seek};

pub enum Input {
    Stdin(io::Stdin),
    File(fs::File)
}

impl Input {
    pub fn open(path: &Option<PathBuf>) -> Result<Self, io::Error> {
        match path {
            Some(path_buf) => Ok(Self::File(fs::File::open(path_buf)?)),
            None => Ok(Self::Stdin(io::stdin()))
        }
    }
}

impl Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Stdin(s) => s.read(buf),
            Self::File(f) => f.read(buf)
        }
    }
}

impl Seek for Input {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match self {
            Self::Stdin(_) => Ok(0),
            Self::File(f) => f.seek(pos)
        }
    }
}