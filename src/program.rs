use std::io::{Read, Write};

#[derive(Debug, PartialEq)] // to be able to test somewhat sensibly
pub enum Command {
    Increment,
    Decrement,
    Input,
    Output,
    Left,
    Right,
    Loop(Program),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    commands: Vec<Command>,
}

impl Program {
    pub fn new(commands: Vec<Command>) -> Program {
        Program { commands }
    }

    pub fn run<R: Read, W: Write>(&mut self, input: &mut R, output: &mut W) {
        let mut array = Array::new();

        self.run_internal(&mut array, input, output);
    }

    fn run_internal<R: Read, W: Write>(&mut self, array: &mut Array, input: &mut R, output: &mut W) {
        for command in &mut self.commands {
            match *command {
                Command::Increment => {
                    let value = array.get_value();
                    array.set_value(value + 1);
                }
                Command::Decrement => {
                    let value = array.get_value();
                    array.set_value(value - 1);
                }
                Command::Output => {
                    output
                        .write(&[array.get_value(); 1])
                        .expect("Failed to write");
                }
                Command::Input => {
                    let mut i = [0; 1];
                    input.read(&mut i).unwrap();
                    array.set_value(i[0]);
                }
                Command::Left => {
                    array.left();
                }
                Command::Right => {
                    array.right();
                }
                Command::Loop(ref mut program) => {
                   program.loop_internal(array, input, output);
                }
            }
        }
    }

    fn loop_internal<R: Read, W: Write>(&mut self, array: &mut Array, input: &mut R, output: &mut W) {
         if array.get_value() == 0 {
            return;
        }

        loop {
            self.run_internal(array, input, output);

            if array.get_value() == 0 {
                break;
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

        array.data.push(0);

        array
    }

    fn right(&mut self) {
        if self.data_pointer == (self.data.len() - 1) {
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

        let mut input = "".as_bytes();
        let mut output = Vec::new();
        program.run(&mut input, &mut output);

        assert_eq!(1, output.len());
        assert_eq!(2, output[0]);
    }

    #[test]
    fn test_decrement() {
        let commands = vec![Command::Increment, Command::Decrement, Command::Output];
        let mut program = Program::new(commands);

        let mut input = "".as_bytes();
        let mut output = Vec::new();
        program.run(&mut input, &mut output);

        assert_eq!(1, output.len());
        assert_eq!(0, output[0]);
    }

    #[test]
    fn test_input() {
        let commands = vec![Command::Input, Command::Output];
        let mut program = Program::new(commands);

        let mut input = "q".as_bytes();
        let mut output = Vec::new();
        program.run(&mut input, &mut output);

        assert_eq!(1, output.len());
        assert_eq!('q', output[0] as char);
    }

    #[test]
    fn test_left_and_right() {
        let commands = vec![
            Command::Right,
            Command::Increment,
            Command::Right,
            Command::Increment,
            Command::Output,
            Command::Left,
            Command::Increment,
            Command::Output,
            Command::Left,
            Command::Left,
            Command::Output,
        ];
        let mut program = Program::new(commands);

        let mut input = "q".as_bytes();
        let mut output = Vec::new();
        program.run(&mut input, &mut output);

        assert_eq!(3, output.len());
        assert_eq!(1, output[0]);
        assert_eq!(2, output[1]);
        assert_eq!(0, output[2]);
    }

    #[test]
    fn test_skip_loop() {
        let loop_commands = vec![Command::Increment];
        let loopp = Command::Loop(Program::new(loop_commands));

        let commands = vec![loopp, Command::Output];
        let mut program = Program::new(commands);

        let mut input = "".as_bytes();
        let mut output = Vec::new();
        program.run(&mut input, &mut output);

        assert_eq!(1, output.len());
        assert_eq!(0, output[0]);
    }

    #[test]
    fn test_loop() {
        let loop_commands = vec![Command::Decrement, Command::Right, Command::Increment, Command::Left];
        let loopp = Command::Loop(Program::new(loop_commands));

        // ++>++<[->+<]>.
        let commands = vec![Command::Increment, Command::Increment, Command::Right, Command::Increment, Command::Increment, Command::Left, loopp, Command::Right, Command::Output];
        let mut program = Program::new(commands);

        let mut input = "".as_bytes();
        let mut output = Vec::new();
        program.run(&mut input, &mut output);

        assert_eq!(1, output.len());
        assert_eq!(4, output[0]);
    }
}
