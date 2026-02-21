use crate::builtin::alias::AliasManager;
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
    let aliases = AliasManager::new();
    let alias = aliases.get_alias(command[0]);
    command = [alias.split_whitespace().collect(), command].concat();

    result.command = command[0].to_string();
    for arg in &command[1..] {
        result.arguments.push(arg.to_string());
    }

    result
}
