use command::Command;

use std::io::{Read, Write};

pub struct Program {
    commands: Vec<Command>,
}

impl Program {
    pub fn new(commands: Vec<Command>) -> Program {
        Program { commands }
    }

    pub fn run<R: Read, W: Write>(&mut self, input: R, output: &mut W) {
        let mut array = Array::new();

        for command in &self.commands {
            match command {
                &Command::Increment => {
                    let value = array.get_value();
                    array.set_value(value + 1);
                }
                &Command::Output => {
                    output.write(&[array.get_value(); 1]).expect("Failed to write");
                }
            }
        }
    }
}

struct Array {
    data: Vec<u8>,
    data_pointer: usize,
}

impl Array {
    fn new() -> Array {
        let mut array = Array {
            data: Vec::with_capacity(30_000),
            data_pointer: 0,
        };

        // set element 0 to 0
        array.data.push(0);

        array
    }

    fn right(&mut self) {
        if self.data_pointer == self.data.len() {
            self.data.push(0);
        }

        self.data_pointer += 1;
    }

    fn left(&mut self) {
        if self.data_pointer != 0 {
            self.data_pointer -= 1;
        }
    }

    fn get_value(&self) -> u8 {
        self.data[self.data_pointer]
    }

    fn set_value(&mut self, value: u8) {
        self.data[self.data_pointer] = value;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_increment() {
        let commands = vec![Command::Increment, Command::Increment, Command::Output];
        let mut program = Program::new(commands);

        let mut output = Vec::new();
        program.run("".as_bytes(), &mut output);

        assert_eq!(1, output.len());
        assert_eq!(2, output[0]);
    }
}