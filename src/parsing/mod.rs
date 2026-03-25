#[derive(Debug, Clone)]
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
#[derive(Debug)]
pub enum ParseError {
    InvalidOperator(String),
    MissBefore(String),
    MissAfter(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    And,
    Or,
    None,
    Background,
    Pipe,
}

#[derive(PartialEq)]
enum QuoteMode {
    Single,
    Double,
}

pub fn simple_parse(input: &str) -> ParsedCommand {
    let mut result = ParsedCommand::new();
    let trimmed = input.trim();

    let mut chars = trimmed.chars().peekable();
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
            '\\' => escape = true,

            '\'' => match mode {
                None => mode = Some(QuoteMode::Single),
                Some(QuoteMode::Single) => mode = None,
                Some(QuoteMode::Double) => current.push('\''),
            },

            '"' => match mode {
                None => mode = Some(QuoteMode::Double),
                Some(QuoteMode::Double) => mode = None,
                Some(QuoteMode::Single) => current.push('"'),
            },

            c if c.is_whitespace() => {
                if mode.is_none() {
                    if !current.is_empty() {
                        if first_token {
                            result.command = current;
                            first_token = false;
                        } else {
                            result.arguments.push(current);
                        }
                        current = String::new();
                    }
                } else {
                    current.push(c);
                }
            }

            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        if first_token {
            result.command = current;
        } else {
            result.arguments.push(current);
        }
    }

    result
}

pub fn split_by_operators(input: &str) -> Result<Vec<(ParsedCommand, Operator)>, ParseError> {
    let mut segments: Vec<(ParsedCommand, Operator)> = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
    let mut last_is_operator: bool = false;

    while let Some(c) = chars.next() {
        match c {
            '&' => {
                match chars.peek() {
                    Some(&'&') => {
                        chars.next();
                        if chars.peek() == Some(&'&') || last_is_operator {
                            return Err(ParseError::InvalidOperator(
                                "invalid operator &&".to_string(),
                            ));
                        }

                        let seg = current.trim().to_string();
                        if seg.is_empty() {
                            return Err(ParseError::MissBefore(
                                "'&&' has no command before it".to_string(),
                            ));
                        }

                        // check nothing after &&
                        let rest = chars.clone().collect::<String>();
                        if rest.trim().is_empty() {
                            return Err(ParseError::MissAfter(
                                "'&&' has no command after it".to_string(),
                            ));
                        }
                        if !seg.is_empty() {
                            let cmd = simple_parse(&seg);

                            segments.push((cmd, Operator::And));
                            last_is_operator = true;
                            current = String::new();
                        }
                    }
                    _ => {
                        // single & operator
                        let seg = current.trim().to_string();
                        if seg.is_empty() {
                            return Err(ParseError::InvalidOperator(
                                "invalid operator &".to_string(),
                            ));
                        } else {
                            // & after a command → Background operator
                            let cmd = simple_parse(&seg);
                            segments.push((cmd, Operator::Background));
                            current = String::new();
                            last_is_operator = true;
                        }
                    }
                }
            }

            '|' => {
                if chars.peek() == Some(&'|') {
                    chars.next();
                    if chars.peek() == Some(&'|') || last_is_operator {
                        return Err(ParseError::InvalidOperator(
                            "invalid operator ||".to_string(),
                        ));
                    }
                    let seg = current.trim().to_string();
                    if seg.is_empty() {
                        return Err(ParseError::MissBefore(
                            "'||' has no command before it".to_string(),
                        ));
                    }

                    // check nothing after ||
                    let rest = chars.clone().collect::<String>();
                    if rest.trim().is_empty() {
                        return Err(ParseError::MissAfter(
                            "'||' has no command after it".to_string(),
                        ));
                    }
                    if !seg.is_empty() {
                        let cmd = simple_parse(&seg);
                        segments.push((cmd, Operator::Or));
                        last_is_operator = true;

                        current = String::new();
                    }
                } else {
                    if last_is_operator {
                        return Err(ParseError::InvalidOperator(
                            "invalid operator | ".to_string(),
                        ));
                    }

                    let seg = current.trim().to_string();

                    if seg.is_empty() {
                        return Err(ParseError::MissBefore(
                            "'|' has no command before it".to_string(),
                        ));
                    }

                    let rest = chars.clone().collect::<String>();

                    if rest.trim().is_empty() {
                        return Err(ParseError::MissAfter(
                            "'|' has no command after it".to_string(),
                        ));
                    }

                    segments.push((simple_parse(&seg), Operator::Pipe));
                    last_is_operator = true;

                    current = String::new();
                }
            }

            _ => {
                current.push(c);
                last_is_operator = false;
            }
        }
    }

    // last segment
    let seg = current.trim().to_string();

    if !seg.is_empty() {
        let cmd = simple_parse(&seg);
        segments.push((cmd, Operator::None));
    }

    Ok(segments)
}
pub fn handle_parse(input: &str) -> Option<Vec<(ParsedCommand, Operator)>> {
    match split_by_operators(input) {
        Ok(res) => Some(res),
        Err(ParseError::InvalidOperator(msg)) => {
            eprintln!("ash: syntax error: {}", msg);
            None
        }
        Err(ParseError::MissBefore(msg)) => {
            eprintln!("ash: syntax error: {}", msg);
            None
        }
        Err(ParseError::MissAfter(msg)) => {
            eprintln!("ash: syntax error: {}", msg);
            None
        }
    }
}
