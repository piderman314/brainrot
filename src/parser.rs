use super::BFError;
use program::Command;
use program::Program;

use std::vec::IntoIter;

pub fn parse(code: Vec<u8>) -> Result<Program, BFError> {
    let mut commands = Vec::new();

    let mut iter = code.into_iter();

    loop {
        let symbol = iter.next();

        if symbol.is_none() {
            break;
        }

        if let Some(command) = map_common_chars(symbol) {
            commands.push(command);
            continue;
        }

        if symbol.unwrap() as char == ']' {
            return Err(BFError {
                message: String::from("Unexpected ]"),
            });
        }

        if symbol.unwrap() as char == '[' {
            commands.push(parse_loop(&mut iter)?);
        }
    }

    Ok(Program::new(commands))
}

fn parse_loop(iter: &mut IntoIter<u8>) -> Result<Command, BFError> {
    let mut commands = Vec::new();

    loop {
        let symbol = iter.next();

        if symbol.is_none() {
            return Err(BFError {
                message: String::from("Unexpected end of input while parsing loop"),
            })
        }

        if let Some(command) = map_common_chars(symbol) {
            commands.push(command);
            continue;
        }

        if symbol.unwrap() as char == ']' {
            break;
        }

        if symbol.unwrap() as char == '[' {
            commands.push(parse_loop(iter)?);
        }
    }

    Ok(Command::Loop(Program::new(commands)))
}

fn map_common_chars(symbol: Option<u8>) -> Option<Command> {
    if let Some(symbol) = symbol {
        match symbol as char {
            '+' => Some(Command::Increment),
            '-' => Some(Command::Decrement),
            ',' => Some(Command::Input),
            '.' => Some(Command::Output),
            '<' => Some(Command::Left),
            '>' => Some(Command::Right),
            _ => None,
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use program::Command;
    use std::error::Error;

    #[test]
    fn parse_unknown_commands() {
        parse_code(
            b"+qwerty.".to_vec(),
            vec![Command::Increment, Command::Output],
        );
    }

    #[test]
    fn parse_plus() {
        parse_code(b"++".to_vec(), vec![Command::Increment, Command::Increment]);
    }

    #[test]
    fn parse_minus() {
        parse_code(b"--".to_vec(), vec![Command::Decrement, Command::Decrement]);
    }

    #[test]
    fn parse_comma() {
        parse_code(b",,".to_vec(), vec![Command::Input, Command::Input]);
    }

    #[test]
    fn parse_period() {
        parse_code(b"..".to_vec(), vec![Command::Output, Command::Output]);
    }

    #[test]
    fn parse_loop() {
        let loop_commands = vec![Command::Decrement, Command::Right, Command::Increment, Command::Left];
        let loopp = Command::Loop(Program::new(loop_commands));

        parse_code(b"++>++<[->+<]>.".to_vec(), vec![Command::Increment, Command::Increment, Command::Right, Command::Increment, Command::Increment, Command::Left, loopp, Command::Right, Command::Output]);
    }

    #[test]
    fn parse_nested_loop() {
        let loop_commands = vec![Command::Decrement, Command::Right, Command::Increment, Command::Left];
        let loopp = vec![Command::Loop(Program::new(loop_commands))];
        let loop2 = Command::Loop(Program::new(loopp));

        parse_code(b"++>++<[[->+<]]>.".to_vec(), vec![Command::Increment, Command::Increment, Command::Right, Command::Increment, Command::Increment, Command::Left, loop2, Command::Right, Command::Output]);
    }

    fn parse_code(input: Vec<u8>, output: Vec<Command>) {
        let result = parse(input);
        assert!(result.is_ok());

        let program = result.unwrap();
        assert_eq!(program, Program::new(output));
    }

    #[test]
    fn parse_unexpected_end_loop() {
        let result = parse(b"+++]".to_vec());
        assert!(result.is_err());

        let error = result.err().unwrap();
        assert_eq!("Unexpected ]", error.description());
    }

    #[test]
    fn parse_unexpected_end_of_input() {
        let result = parse(b"+[++".to_vec());
        assert!(result.is_err());

        let error = result.err().unwrap();
        assert_eq!("Unexpected end of input while parsing loop", error.description());
    }
}
