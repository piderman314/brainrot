use command::Command;
use program::Program;
use super::BFError;

pub fn parse(code: Vec<u8>) -> Result<Program, BFError> {
    Ok(Program::new(parse_internal(code)?))
}

fn parse_internal(code: Vec<u8>) -> Result<Vec<Command>, BFError> {
    let mut commands = Vec::new();

    for symbol in code {
        match symbol as char {
            '+' => commands.push(Command::Increment),
            _ => continue,
        }
    }

    Ok(commands)
}

#[test]
fn parse_unknown_commands() {
    let result = parse_internal(b"+qwerty".to_vec());
    assert!(result.is_ok());

    let commands = result.unwrap();
    assert_eq!(1, commands.len());
    assert!(matches!(commands[0], Command::Increment));
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