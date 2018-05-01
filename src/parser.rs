use super::BFError;
use program::Command;
use program::Program;

pub fn parse(code: Vec<u8>) -> Result<Program, BFError> {
    Ok(Program::new(parse_internal(code)?))
}

fn parse_internal(code: Vec<u8>) -> Result<Vec<Command>, BFError> {
    let mut commands = Vec::new();

    for symbol in code {
        match symbol as char {
            '+' => commands.push(Command::Increment),
            '-' => commands.push(Command::Decrement),
            ',' => commands.push(Command::Input),
            '.' => commands.push(Command::Output),
            '<' => commands.push(Command::Left),
            '>' => commands.push(Command::Right),
            _ => continue,
        }
    }

    Ok(commands)
}

#[cfg(test)]
mod test {
    use super::*;
    use program::Command;

    #[test]
    fn parse_unknown_commands() {
        parse_code(b"+qwerty.".to_vec(), vec![Command::Increment, Command::Output]);
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
}
