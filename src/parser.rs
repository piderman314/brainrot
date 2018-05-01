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
        let result = parse_internal(b"+qwerty.".to_vec());
        assert!(result.is_ok());

        let commands = result.unwrap();
        assert_eq!(2, commands.len());
        assert!(matches!(commands[0], Command::Increment));
        assert!(matches!(commands[1], Command::Output));
    }

    #[test]
    fn parse_plus() {
        let result = parse_internal(b"++".to_vec());
        assert!(result.is_ok());

        let commands = result.unwrap();
        assert_eq!(2, commands.len());
        assert!(matches!(commands[0], Command::Increment));
        assert!(matches!(commands[1], Command::Increment));
    }

    #[test]
    fn parse_minus() {
        let result = parse_internal(b"--".to_vec());
        assert!(result.is_ok());

        let commands = result.unwrap();
        assert_eq!(2, commands.len());
        assert!(matches!(commands[0], Command::Decrement));
        assert!(matches!(commands[1], Command::Decrement));
    }


    #[test]
    fn parse_comma() {
        let result = parse_internal(b",,".to_vec());
        assert!(result.is_ok());

        let commands = result.unwrap();
        assert_eq!(2, commands.len());
        assert!(matches!(commands[0], Command::Input));
        assert!(matches!(commands[1], Command::Input));
    }

    #[test]
    fn parse_period() {
        let result = parse_internal(b"..".to_vec());
        assert!(result.is_ok());

        let commands = result.unwrap();
        assert_eq!(2, commands.len());
        assert!(matches!(commands[0], Command::Output));
        assert!(matches!(commands[1], Command::Output));
    }
}
