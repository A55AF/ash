use std::env;
use std::path::Path;

mod parsing;
use parsing::ParsedCommand;

fn execute_builtin_function(command: ParsedCommand) {
    match command.commnad {
        "cd" => break,
        "exit" => break,
        "pwd" => break,
        "echo" => break,
        "export" => break,
        "unset" => break,
    }
}

fn change_directory(args: &[String]) -> Result<(), String> {}
