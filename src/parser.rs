use super::BFError;
use program::Command;
use program::Program;

pub fn parse(code: Vec<u8>) -> Result<Program, BFError> {
    Ok(Program::new(parse_internal(code)?))
}

fn parse_internal(code: Vec<u8>) -> Result<Vec<Command>, BFError> {
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
    }

    Ok(commands)
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

    fn parse_code(input: Vec<u8>, output: Vec<Command>) {
        let result = parse_internal(input);
        assert!(result.is_ok());

        let commands = result.unwrap();
        assert_eq!(commands, output);
    }

    #[test]
    fn parse_unexpected_end_loop() {
        let result = parse_internal(b"+++]".to_vec());
        assert!(result.is_err());

        let error = result.err().unwrap();
        assert_eq!("Unexpected ]", error.description());
    }
}
