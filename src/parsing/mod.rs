// src/parsing/mod.rs
use std::collections::HashMap;
pub struct ParsedCommand {
    pub command: String,
    pub options:  HashMap<String, Vec<String>>,
//    pub arguments: Vec<String>,
}


impl ParsedCommand {
    pub fn new() -> Self {
        ParsedCommand {
            command: String::new(),
            options: HashMap::new(),
           // arguments: Vec::new(),
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
      let mut last_was_option = false;
        let mut last_option = String::new();  
  
  for part in &parts[1..] {
   if part.starts_with('-') {
            
           //let option_name = part.to_string();  
last_option=part.to_string();         
 last_was_option = true;
                 result.options
                .entry(last_option.clone())    
                        .or_insert_with(Vec::new);
        }
        else if last_was_option {
         
            result.options
               .entry(last_option.clone())
                .and_modify(|v| v.push(part.to_string()))
                .or_insert_with(|| vec![part.to_string()]);
                 
        }
        else {
            result.options
                .entry("ARGUMENTS".to_string())  
                .or_insert_with(Vec::new)
                .push(part.to_string());
        }
} 
        println!("Command: {}", result.command);
           for (opt, values) in &result.options {
      
            println!("  Option: {} with aruments: {:?}", opt, values);
        
    }
          result
}
