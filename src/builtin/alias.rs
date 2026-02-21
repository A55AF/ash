use crate::parsing::ParsedCommand;
use crate::ShellState;

pub fn set_alias(cmd: &ParsedCommand, shell: &mut ShellState) {
    let key = cmd.arguments[0].split('=').next().unwrap().to_string();
    let val = cmd.arguments[0][key.len()+1..].to_string();
    shell.aliases.insert(key, val);
}
pub fn get_alias(key: &str, shell: &ShellState) -> String {
    if shell.aliases.contains_key(key) {
        let input: Vec<&str> = shell.aliases.get(key).unwrap().split_whitespace().collect();
        if input.is_empty(){
            return "".to_string();
        }
        let mut command = get_alias(input[0], shell);
        for s in &input[1..] {
            command.push_str(" ");
            command.push_str(s);
        }
        return command;
    }
    key.to_string()
}
