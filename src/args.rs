use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::io;
use clap::Parser;

static DEFAULT_BS: usize = 512;

/// A Rust implementation of the Linux dd utility.
#[derive(Parser, Debug)]
#[clap(name = "dd")]
#[clap(version, author)]
#[clap(about = "A Rust implementation of the Linux dd utility.")]
pub struct Arguments {
   /// Read from IF instead of stdin
   #[clap(long, parse(from_os_str))]
   r#if: Option<PathBuf>,

   /// Write to OF instead of stdout
   #[clap(long, parse(from_os_str))]
   of: Option<PathBuf>,

   /// Read and write up to BS bytes at a time; overrides ibs and obs
   #[clap(long, value_parser, default_value_t = DEFAULT_BS)]
   bs: usize,

   /// Read up to IBS bytes at a time
   #[clap(long, value_parser, default_value_t = DEFAULT_BS)]
   ibs: usize,

   /// Write OBS bytes at a time
   #[clap(long, value_parser, default_value_t = DEFAULT_BS)]
   obs: usize,

   /// Copy only COUNT input blocks
   #[clap(long, value_parser)]
   count: Option<usize>,

   /// Skip SEEK obs-sized blocks at start of output
   #[clap(long, value_parser)]
   seek: Option<usize>,

   /// Skip SKIP ibs-sized blocks at start of input
   #[clap(long, value_parser)]
   skip: Option<usize>
}

type DDInput = dyn io::Read + 'static;
type DDOutput = dyn io::Write + 'static;

impl Arguments {
   pub fn resolve(&mut self) {
      if self.bs != DEFAULT_BS {
         self.ibs = self.bs;
         self.obs = self.bs;
      }
   }

   pub fn open_if(&self) -> Result<Box<DDInput>, io::Error> {
      match self.r#if.as_ref() {
         Some(path_buf) => Ok(Box::new(File::open(path_buf)?)),
         None => return Ok(Box::new(io::stdin()))
      }
   }

   pub fn open_of(&self) -> Result<Box<DDOutput>, io::Error> {
      match self.of.as_ref() {
         Some(path_buf) => {
            let f = OpenOptions::new()
               .read(true)
               .write(true)
               .create(true)
               .open(path_buf)?;
            Ok(Box::new(f))
         },
         None => return Ok(Box::new(io::stdout()))
      }
   }

   pub fn get_ibs(&self) -> usize {
      self.ibs
   }

   pub fn get_obs(&self) -> usize {
      self.obs
   }

   pub fn get_count(&self) -> &Option<usize> {
      &self.count
   }
}