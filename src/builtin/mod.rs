use std::env;
// use std::path::Path;

use crate::ShellState;
use crate::parsing::ParsedCommand;

// Eyad made this function
pub fn change_directory(cli: &ParsedCommand, shell: &mut ShellState) {
    let target = if cli.arguments.is_empty() {
        &shell.home.as_str()
    } else {
        cli.arguments[0].as_str()
    };

    if let Err(e) = env::set_current_dir(target) {
        eprintln!("cd error: {}", e);
    } else {
        shell.working_directory = env::current_dir().unwrap().to_string_lossy().to_string();
    }
}

pub fn exit_shell(cli: &ParsedCommand, shell: &mut ShellState) {
    let command_arg: Option<&String> = cli.arguments.get(0);
    let mut code: Option<i8> = command_arg.and_then(|s| s.parse().ok());

    if code == None {
        code = Some(0);
        println!("Not a valid argument");
    }

    shell.exit_code = code;
    shell.should_exit = true;
}
