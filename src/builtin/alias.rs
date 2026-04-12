use std::collections::HashSet;

use crate::parsing::{Operator, ParsedCommand};
use crate::ShellState;

pub fn unquote(string: &mut String) {
    if string.len() >= 2 && string.starts_with('"') && string.ends_with('"') {
        string.remove(0);
        string.pop();
    }
}
pub fn alias(cmd: &ParsedCommand, shell: &mut ShellState) {
    if !cmd.arguments[0].contains('='){
        println!("{} is not a valid alias", cmd.arguments[0]);
        return;
    }
    let mut key = cmd.arguments[0].split('=').next().unwrap().to_string();
    let mut val = cmd.arguments[0][key.len()+1..].to_string();
    unquote(&mut key);
    unquote(&mut val);
    shell.aliases.insert(key, val);
}
pub fn unalias(cmd: &ParsedCommand, shell: &mut ShellState){
    let key = &cmd.arguments[0];
    if key == "-a" {
        shell.aliases.clear();
    }
    else if shell.aliases.contains_key(key) {
        shell.aliases.remove(key);
    }
    else {
        println!("{} alias not found", key);
    }
}
pub fn get_alias(key: &str, shell: &ShellState, visited: &mut HashSet<String>) -> String {
    if !visited.insert(key.to_string()) {
        return key.to_string();
    }
    if shell.aliases.contains_key(key) {
        let val = shell.aliases.get(key).unwrap().to_string();
        if val.is_empty() || val == key {
            return val;
        }

        if val != key {
            return check_alias(&val, shell, visited);
        } 
    }
    key.to_string()
}
pub fn check_alias(cmd: &str, shell: &ShellState, visited: &mut HashSet<String>) -> String {
    let mut command = cmd.split_whitespace().next().unwrap().to_string();
    let l = command.len();
    command = get_alias(&command, shell, visited);
    if l < cmd.len() {
        command = [command, " ".to_string(), cmd[l + 1..].to_string()].concat();
    }
    command
}

pub fn check_aliases(cmd: Vec<(ParsedCommand, Operator)>, shell: &ShellState) -> Vec<(ParsedCommand, Operator)> {
    let mut expanded_commands = cmd;
    for (parsed_command, _) in expanded_commands.iter_mut() {
        let mut visited: HashSet<String> = HashSet::new();
        let command = parsed_command.command.clone();
        let alias_value = check_alias(&command, shell, &mut visited);
        let alias = alias_value.split_whitespace().collect::<Vec<_>>();
        parsed_command.command = alias[0].to_string();
        if alias.len() > 1 {
            let mut new_args: Vec<String> = alias.into_iter().skip(1).map(|s| s.to_string()).collect();
            new_args.append(&mut parsed_command.arguments);
            parsed_command.arguments = new_args;
        }
    }

    expanded_commands
}
