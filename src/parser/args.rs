use super::error::{ParseError, ParseErrorKind::*};
use std::path::PathBuf;
use std::str::FromStr;
use std::env;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Arguments {
    /// Read from IF instead of stdin
    r#if: Option<PathBuf>,
    /// Write to OF instead of stdout
    of: Option<PathBuf>,
    /// Read and write up to BS bytes at a time; overrides ibs and obs
    bs: usize,
    /// Read up to IBS bytes at a time
    ibs: usize,
    /// Write OBS bytes at a time
    obs: usize,
    /// Copy only COUNT input blocks
    count: Option<usize>,
    /// Skip SEEK obs-sized blocks at start of output
    seek: Option<usize>,
    /// Skip SKIP ibs-sized blocks at start of input
    skip: Option<usize>
}

#[derive(Default)]
struct ArgsBuilder {
    r#if: Option<PathBuf>,
    of: Option<PathBuf>,
    bs: Option<usize>,
    ibs: Option<usize>,
    obs: Option<usize>,
    count: Option<usize>,
    seek: Option<usize>,
    skip: Option<usize>
}

impl ArgsBuilder {
    fn parse_kvp(&mut self, key: &str, value: &str) -> Result<(), ParseError> {
        let as_int_res = try_to_int(key, value);
        match key {
            "if"    => self.r#if  = Some(value.into()),
            "of"    => self.of    = Some(value.into()),
            "bs"    => self.bs    = Some(as_int_res?),
            "ibs"   => self.ibs   = Some(as_int_res?),
            "obs"   => self.obs   = Some(as_int_res?),
            "seek"  => self.seek  = Some(as_int_res?),
            "skip"  => self.skip  = Some(as_int_res?),
            "count" => self.count = Some(as_int_res?),
            _ => return Err(ParseError::new(UnknownOperand, key)),
        }

        Ok(())
    }

    #[allow(dead_code, unused_variables)]
    fn parse_flag(&mut self, flag: &str) {
        todo!();
    }

    fn collect(mut self) -> Arguments {
        if let Some(bytes) = self.bs {
            self.ibs = Some(bytes);
            self.obs = Some(bytes);
        }

        Arguments {
            r#if: self.r#if,
            of: self.of,
            bs: self.bs.unwrap_or(Arguments::DEFAULT_BS),
            ibs: self.ibs.unwrap_or(Arguments::DEFAULT_BS),
            obs: self.obs.unwrap_or(Arguments::DEFAULT_BS),
            count: self.count,
            seek: self.seek,
            skip: self.skip
        }
    }
}

fn try_to_int<T: FromStr>(k: &str, v: &str) -> Result<T, ParseError> {
    match v.parse() {
        Ok(r) => Ok(r),
        Err(_) => Err(ParseError::new(InvalidNumericValue, k))
    }
}

impl Arguments {
    const ARG_DELIMITER: char = '=';
    const DEFAULT_BS: usize = 512;

    pub fn parse() -> Result<Self, ParseError> {
        let mut builder = ArgsBuilder::default();

        for operand in env::args().skip(1) {
            let split: Vec<&str> = operand
                .splitn(2, Self::ARG_DELIMITER)
                .collect();

            let (key, value): (&str, &str) = match split[..] {
                [k] => return Err(ParseError::new(UnknownOperand, k)),
                [k, ""] => return Err(ParseError::new(NoValueSpecified, k)),
                [k, v] => (k, v),
                _ => return Err(ParseError::new(UnknownOperand, ""))
            };

            builder.parse_kvp(key, value)?;
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
}