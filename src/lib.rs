mod parser;
mod program;

use std::io::{Read, Write};

#[derive(Debug, Clone)]
pub struct BFError;

pub fn run<R: Read, W:Write>(code: Vec<u8>, input: &mut R, output: &mut W) {
    let mut program = parser::parse(code).unwrap();

    program.run(input, output);
}
