use std::path::PathBuf;
use clap::Parser;

// TODO: https://stackoverflow.com/questions/37223741/how-can-i-take-input-from-either-stdin-or-a-file-if-i-cannot-seek-stdin

static DEFAULT_BS: usize = 512;

/// A Rust implementation of the Linux dd utility.
#[derive(Parser, Debug, Clone)]
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
   seek: Option<u64>,

   /// Skip SKIP ibs-sized blocks at start of input
   #[clap(long, value_parser)]
   skip: Option<u64>
}

impl Arguments {
   pub fn resolve(&mut self) {
      if self.bs != DEFAULT_BS {
         self.ibs = self.bs;
         self.obs = self.bs;
      }
   }

   pub fn get_if_path(&self) -> &Option<PathBuf> {
      &self.r#if
   }

   pub fn get_of_path(&self) -> &Option<PathBuf> {
      &self.of
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

   pub fn get_seek(&self) -> Option<u64> {
      self.seek
   }

   pub fn get_skip(&self) -> Option<u64> {
      self.skip
   }
}