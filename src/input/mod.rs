use crate::{ShellState, builtin::alias::check_aliases, parsing::{Operator, ParsedCommand, split_by_operators}};

pub fn read_input(shell_state: &ShellState, multpile_lines: bool) -> Option<Vec<(ParsedCommand, Operator)>>{
    if multpile_lines {
        print!("> ");
    }

    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();

    if input.is_empty() {
        return None;
    }

    input = check_aliases(&input, shell_state);
    let commands: Vec<(ParsedCommand, Operator)> = split_by_operators(&input);

    Some(commands)
}
