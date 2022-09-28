use std::{env, fmt, error::Error};

#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseErrorKind {
    UnknownOperand,
    NoValueSpecified,
    InvalidNumericValue
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParseError {
    kind: ParseErrorKind,
    description: String
}

use ParseErrorKind::*;

impl ParseError {
    pub(in crate::parser) fn new(kind: ParseErrorKind, key: &str) -> Self {
        let reason = match kind {
            UnknownOperand => format!("unknown operand {}", key),
            NoValueSpecified => format!("no value specified for {}", key),
            InvalidNumericValue => format!("{}: invalid numeric value", key)
        };

        let program_name = env::args().nth(0).unwrap();
        let description = format!("{}: {}", program_name, reason);

        Self {
            kind,
            description
        }
    } 
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.description
    }
}