use crate::parser::error::{ParseError, ParseErrorKind::*};
use crate::parser::builder::ArgumentsBuilder;
use std::path::PathBuf;
use std::env;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Arguments {
    /// Read from IF instead of stdin
    pub (in crate::parser) r#if: Option<PathBuf>,
    /// Write to OF instead of stdout
    pub (in crate::parser) of: Option<PathBuf>,
    /// Read and write up to BS bytes at a time; overrides ibs and obs
    pub (in crate::parser) bs: usize,
    /// Read up to IBS bytes at a time
    pub (in crate::parser) ibs: usize,
    /// Write OBS bytes at a time
    pub (in crate::parser) obs: usize,
    /// Copy only COUNT input blocks
    pub (in crate::parser) count: Option<usize>,
    /// Skip SEEK obs-sized blocks at start of output
    pub (in crate::parser) seek: Option<usize>,
    /// Skip SKIP ibs-sized blocks at start of input
    pub (in crate::parser) skip: Option<usize>,
    pub (in crate::parser) print_help: bool,
    pub (in crate::parser) print_version: bool
}

impl Arguments {
    const ARG_DELIMITER: char = '=';
    pub const DEFAULT_BS: usize = 512;

    pub fn parse() -> Result<Self, ParseError> {
        let mut builder = ArgumentsBuilder::default();

        for operand in env::args().skip(1) {
            let split: Vec<&str> = operand
                .splitn(2, Self::ARG_DELIMITER)
                .collect();

            match split[..] {
                [k] => builder.parse_flag(k)?,
                [k, ""] => return Err(ParseError::new(NoValueSpecified, k)),
                [k, v] => builder.parse_kvp(k, v)?,
                _ => unreachable!("Should ONLY find arrays of size at most 2")
            };
        }

        Ok(builder.collect())
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

    pub fn get_count(&self) -> Option<usize> {
        self.count
    }

    pub fn get_seek(&self) -> Option<usize> {
        self.seek
    }

    pub fn get_skip(&self) -> Option<usize> {
        self.skip
    }

    pub fn help_requested(&self) -> bool {
        self.print_help
    }

    pub fn version_requested(&self) -> bool {
        self.print_version
    }
}