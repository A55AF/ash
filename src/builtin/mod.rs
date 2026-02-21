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

pub fn print_working_directory(shell: &mut ShellState) {
    println!("{}", shell.working_directory);

    shell.exit_code = Some(0);
}

pub fn echo(cli: &ParsedCommand, shell: &mut ShellState) {
    println!("{}", cli.arguments.join(" "));

    shell.exit_code = Some(0);
}

// NOTE: for now this function is temporary, until we make the conf file
pub fn export(cli: &ParsedCommand, shell: &mut ShellState) {
    for arg in cli.arguments.iter() {
        if let Some((key, value)) = arg.split_once('=') {
            shell.env_vars.insert(key.to_string(), value.to_string());
        } else {
            shell
                .env_vars
                .entry(arg.clone())
                .or_insert_with(String::new);
        }
    }

    shell.exit_code = Some(0);
}

pub fn unset(cli: &ParsedCommand, shell: &mut ShellState) {
    for arg in cli.arguments.iter() {
        shell.env_vars.remove(arg);
    }

    shell.exit_code = Some(0);
}
