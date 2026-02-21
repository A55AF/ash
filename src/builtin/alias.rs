use std::collections::HashMap;
use crate::parsing::ParsedCommand;
use std::sync::{OnceLock, Mutex, MutexGuard};

static DICTIONARY: OnceLock<Mutex<HashMap<String, String>>> = OnceLock::new();
fn get_aliases() -> &'static Mutex<HashMap<String, String>> {
    DICTIONARY.get_or_init(|| Mutex::new(HashMap::new()))
}
pub fn set_alias(cmd: &ParsedCommand) {
    let mut aliases = get_aliases().lock().unwrap();
    let key = cmd.arguments[0].split('=').next().unwrap().to_string();
    let val = cmd.arguments[0][key.len()+1..].to_string();
    aliases.insert(key, val);
}
pub fn get_alias(key: &str) -> String {
    let mut aliases = get_aliases().lock().unwrap();
    if aliases.contains_key(key) {
        let input: Vec<&str> = aliases.get(key).unwrap().split_whitespace().collect();
        if input.is_empty(){
            return "".to_string();
        }
        let mut command = get_alias(input[0]);
        for s in &input[1..] {
            command.push_str(" ");
            command.push_str(s);
        }
        return command;
    }
    key.to_string()
}
