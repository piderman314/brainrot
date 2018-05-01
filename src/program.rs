use command::Command;

use std::io::{Read, Write};

pub struct Program {
    commands: Vec<Command>,
}

impl Program {
    pub fn new(commands: Vec<Command>) -> Program {
        Program { commands }
    }

    pub fn run<R: Read, W: Write>(&self, input: R, output: W) {
        //
    }
}