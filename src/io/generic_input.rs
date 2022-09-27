use std::path::PathBuf;
use std::{io, fs};

pub enum GenericInput {
    Stdin(io::Stdin),
    File(fs::File)
}

impl GenericInput {
    pub fn open(path: &Option<PathBuf>) -> Result<Self, io::Error> {
        match path {
            Some(path_buf) => Ok(Self::File(fs::File::open(path_buf)?)),
            None => Ok(Self::Stdin(io::stdin()))
        }
    }
}

impl io::Read for GenericInput {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Stdin(s) => s.read(buf),
            Self::File(f) => f.read(buf)
        }
    }
}

impl io::Seek for GenericInput {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match self {
            Self::Stdin(_) => Ok(0),
            Self::File(f) => f.seek(pos)
        }
    }
}