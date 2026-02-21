use std::collections::HashMap;
use crate::parsing::ParsedCommand;

pub struct AliasManager{
    aliases: HashMap<String, String>
}
impl AliasManager {
    pub fn new() -> Self {
        Self {
            aliases: HashMap::new()
        }
    }
    pub fn set_alias(&mut self, cmd: &ParsedCommand) {
        // you should split after the first = only (still working on it)
        let pair:Vec<&str> = cmd.arguments[0].split('=').collect();
        if pair.len() != 2 {
            panic!("Invalid arguments.");
        }
        self.aliases.insert(pair[0].to_string(), pair[1].to_string());
    }
    pub fn get_alias(&self, key: &str) -> String {
        if self.aliases.contains_key(key) {
            let command = self.aliases.get(key).unwrap().to_string();
            let alias = command.split_whitespace().next().unwrap();
            // command[0] should be erased (still working on it)
            return  [self.get_alias(alias), " ".to_string(), command].concat();
        }
        key.to_string()
    }
}
