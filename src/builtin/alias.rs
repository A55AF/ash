use crate::parsing::ParsedCommand;
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
pub fn get_alias(key: &str, shell: &ShellState) -> String {
    if shell.aliases.contains_key(key) {
        let val = shell.aliases.get(key).unwrap().to_string();
        if val.is_empty(){
            return val;
        }
        let mut command = val.split_whitespace().next().unwrap().to_string();
        let l = command.len();
        command = get_alias(&command, shell);
        if l < val.len() {
            command = [command, val[l + 1..].to_string()].concat();
        }
        return command;
    }
    key.to_string()
}
