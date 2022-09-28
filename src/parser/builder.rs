use crate::parser::{*, ParseErrorKind::*};

use std::path::PathBuf;
use std::str::FromStr;

#[derive(Default)]
pub (in crate::parser) struct ArgumentsBuilder {
    r#if: Option<PathBuf>,
    of: Option<PathBuf>,
    bs: Option<usize>,
    ibs: Option<usize>,
    obs: Option<usize>,
    count: Option<usize>,
    seek: Option<usize>,
    skip: Option<usize>,
    print_help: bool,
    print_version: bool
}

fn set_flag(flag: &mut bool) {
    *flag = true;
}

fn try_to_int<T: FromStr>(k: &str, v: &str) -> Result<T, ParseError> {
    match v.parse() {
        Ok(r) => Ok(r),
        Err(_) => Err(ParseError::new(InvalidNumericValue, k))
    }
}

impl ArgumentsBuilder {
    pub (in crate::parser) fn parse_kvp(&mut self, key: &str, value: &str) -> Result<(), ParseError> {
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
            _       => return Err(ParseError::new(UnknownOperand, key)),
        }

        Ok(())
    }

    pub (in crate::parser) fn parse_flag(&mut self, flag: &str) -> Result<(), ParseError> {
        match flag {
            "--help" | "-h"    => set_flag(&mut self.print_help),
            "--version" | "-V" => set_flag(&mut self.print_version),
            "excl"             => todo!(),
            "nocreat"          => todo!(),
            "notrunc"          => todo!(),
            "count_bytes"      => todo!(),
            "skip_bytes"       => todo!(),
            "seek_bytes"       => todo!(),
            _                  => return Err(ParseError::new(UnknownOperand, flag))
        }

        Ok(())
    }

    pub (in crate::parser) fn collect(mut self) -> Arguments {
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
            skip: self.skip,
            print_help: self.print_help,
            print_version: self.print_version
        }
    }
}
