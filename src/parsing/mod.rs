
// ─── Data Structures ───────────────────────────────────────────────────────

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

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    And,        
    Or,         
None,
Background,}

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


pub fn split_by_operators(input: &str) -> Vec<(ParsedCommand, Operator)> {
    let mut segments: Vec<(ParsedCommand, Operator)> = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();
  
    while let Some(c) = chars.next() {
   

        match c {
          
            '&' => {
                match chars.peek() {
                    Some(&'&') => {
                        chars.next();
                        let seg = current.trim().to_string();
                        if !seg.is_empty() {
                        let cmd = simple_parse(&seg); // ← fix: &seg
                            segments.push((cmd,Operator::And)); 
                            current = String::new();
                        }
                    }
                    _ => {
                        // single & operator
                        let seg = current.trim().to_string();
                        if seg.is_empty() {
                            // & is at start → it's a background prefix for next cmd
                            current.push('&');
                        } else {
                            // & after a command → Background operator
                                  let cmd = simple_parse(&seg);
                           segments.push((cmd,Operator::Background)); 
                            current = String::new();
                        }
                    }
                }
            }

            '|' => {
                if chars.peek() == Some(&'|') {
                    // || operator → flush current with Or
                    chars.next();
                    let seg = current.trim().to_string();
                    if !seg.is_empty() {
                     let cmd = simple_parse(&seg); 
                       segments.push((cmd, Operator::Or)); 
                        current = String::new();
                    }
                } 
                else {
                    current.push(c);
                }
            }

            _ => current.push(c),
        }
    }

    // last segment
    let seg = current.trim().to_string();
    
    if !seg.is_empty() {
   let cmd = simple_parse(&seg); 
        segments.push((cmd, Operator::None)); 
    }

    segments
}





