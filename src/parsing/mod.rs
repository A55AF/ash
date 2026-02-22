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
    let mut result = ParsedCommand::new();
    let mut chars = input.trim().chars().peekable();
    let mut current_arg = String::new();
    let mut in_quotes = false;
    let mut in_quote = false;
    let mut escaped = false;
    let mut first_token = true;

    while let Some(c) = chars.next() {
        if escaped {
            // Escaped character: add it literally
            current_arg.push(c);
            escaped = false;
            continue;
        }

        match c {
            '\\' => {
                escaped = true;
                // Don't add the backslash yet; next char will be added literally
            }
            '"' => {
                in_quotes = !in_quotes;
                // Quote character itself is not added to the argument
            }
            '\'' => {
                in_quote = !in_quote;
                // Quote character itself is not added to the argument
            }
            c if c.is_whitespace() && !in_quotes && !in_quote => {
                // Whitespace outside quotes ends the current argument
                if !current_arg.is_empty() {
                    if first_token {
                        result.command = current_arg;
                        first_token = false;
                    } else {
                        result.arguments.push(current_arg);
                    }
                    current_arg = String::new();
                }
                // Skip this whitespace
            }
            _ => {
                // Normal character: add to current argument
                current_arg.push(c);
            }
        }
    }

    // Handle last argument
    if !current_arg.is_empty() {
        if first_token {
            result.command = current_arg;
        } else {
            result.arguments.push(current_arg);
        }
    }

    result
}
