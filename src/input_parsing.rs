// src/parsing/mod.rs
pub struct ParsedCommand {
    pub command: String,
    pub options: Vec<String>,
    pub arguments: Vec<String>,}


impl ParsedCommand {
    pub fn new() -> Self {
        ParsedCommand {
            command: String::new(),
            options: Vec::new(),
            arguments: Vec::new(),
        }
    }
}

pub fn simple_parse(input: &str) -> ParsedCommand {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
    let mut result = ParsedCommand::new();
    if parts.is_empty() {
        return result;
    }

  result.command = parts[0].to_string();
  
    
    for part in &parts[1..] {
        if part.starts_with('-') {
           result.options.push(part.to_string());
        } else {
            result.arguments.push(part.to_string());
        }
    }
    result
}
