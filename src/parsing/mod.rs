use crate::builtin::alias;
pub struct ParsedCommand {
    pub command: String,
    pub arguments: Vec<String>,
}

impl ParsedCommand {
    pub fn new() -> Self {
        ParsedCommand {
            command: String::new(),
            arguments: Vec::new(),
        }
    }
}

pub fn simple_parse(input: &str) -> ParsedCommand {
    let mut command: Vec<&str> = input.trim().split_whitespace().collect();

    let mut result = ParsedCommand::new();
    if command.is_empty() {
        return result;
    }

    let aliased = alias::get_alias(command[0]);
    command = [aliased.split_whitespace().collect(), command[1..].to_vec()].concat();
    if command.is_empty(){
        return result;
    }

    result.command = command[0].to_string();
    for arg in &command[1..] {
        result.arguments.push(arg.to_string());
    }

    result
}
