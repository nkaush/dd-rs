use std::path::PathBuf;
use std::{io, fs};

pub enum GenericOutput {
    Stdout(io::Stdout),
    File(fs::File)
}

impl GenericOutput {
    pub fn open(path: &Option<PathBuf>) -> Result<Self, io::Error> {
        match path {
            Some(path_buf) => {
                match fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(path_buf) {
                        Ok(f) => Ok(Self::File(f)),
                        Err(e) => Err(e)
                    }
            },
            None => Ok(Self::Stdout(io::stdout()))
        }
    }
}

impl io::Write for GenericOutput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::Stdout(s) => s.write(buf),
            Self::File(f) => f.write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::Stdout(s) => s.flush(),
            Self::File(f) => f.flush()
        }
    }
}

impl io::Seek for GenericOutput {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match self {
            Self::Stdout(_) => Ok(0),
            Self::File(f) => f.seek(pos)
        }
    }
}