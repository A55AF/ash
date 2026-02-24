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

#[derive(PartialEq)]
enum QuoteMode {
    Single,
    Double,
}

pub fn simple_parse(input: &str) -> ParsedCommand {
    let mut result = ParsedCommand::new();
    let mut chars = input.trim().chars();
    let mut current = String::new();
    let mut mode: Option<QuoteMode> = None;
    let mut escape = false;
    let mut first_token = true;

    while let Some(c) = chars.next() {
        if escape {
            current.push(c);
            escape = false;
            continue;
        }

        match c {
            '\\' => {
                escape = true; // backslash escapes next character
            }
            '\'' => {
                match mode {
                    None => mode = Some(QuoteMode::Single), // enter single quotes
                    Some(QuoteMode::Single) => mode = None, // exit single quotes
                    Some(QuoteMode::Double) => current.push('\''), // literal inside double quotes
                }
            }
            '"' => {
                match mode {
                    None => mode = Some(QuoteMode::Double), // enter double quotes
                    Some(QuoteMode::Double) => mode = None, // exit double quotes
                    Some(QuoteMode::Single) => current.push('"'), // literal inside single quotes
                }
            }
            c if c.is_whitespace() => {
                if mode.is_none() {
                    // whitespace outside quotes ends the current argument
                    if !current.is_empty() {
                        if first_token {
                            result.command = current;
                            first_token = false;
                        } else {
                            result.arguments.push(current);
                        }
                        current = String::new();
                    }
                    // skip this whitespace
                } else {
                    // whitespace inside quotes is part of the argument
                    current.push(c);
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    // Handle any remaining characters after the loop
    if !current.is_empty() {
        if first_token {
            result.command = current;
        } else {
            result.arguments.push(current);
        }
    }

    result
}
