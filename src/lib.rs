mod parser;
mod program;

use std::io::{Read, Write};
use std::error::Error;
use std::fmt;

pub fn run<R: Read, W:Write>(code: Vec<u8>, input: &mut R, output: &mut W) -> Result<(), BFError> {
    let program = parser::parse(code)?;

    program.run(input, output);

    Ok(())
}

#[derive(Debug, Clone)]
pub struct BFError {
    message: String,
}

impl Error for BFError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl fmt::Display for BFError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BFError: {}", self.message)
    }
}