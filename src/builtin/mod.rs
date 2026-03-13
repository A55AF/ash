pub mod alias;

use crate::config::add_config;
use crate::config::remove_var_from_config;
use crate::config::save_history;
use crate::parsing::ParsedCommand;
use crate::{ShellState, config};
use std::{iter::Peekable, str::Chars};

use std::env;
use std::fs::File;
// use std::process::Command;

pub fn change_directory_to_home(shell: &mut ShellState) {
    let target: String = shell.home.clone();

    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}: {}", target, e);
        shell.exit_code = Some(1);
        return;
    }
}

// Eyad made this function
pub fn change_directory(cli: &ParsedCommand, shell: &mut ShellState) {
    // Determine the target path
    let target: String = if cli.arguments.is_empty() {
        // No arguments: go to home
        shell.home.clone()
    } else {
        let first_arg: &String = &cli.arguments[0];
        if first_arg.starts_with('~') {
            // Expand ~ to home directory
            if first_arg == "~" {
                shell.home.clone()
            } else {
                // Replace the leading '~' with the home path
                format!("{}{}", shell.home, &first_arg[1..])
            }
        } else {
            // Normal path
            first_arg.clone()
        }
    };

    // Attempt to change directory
    if let Err(e) = env::set_current_dir(&target) {
        eprintln!("cd: {}: {}", target, e);
        shell.exit_code = Some(1);
        return;
    }

    // Update the stored working directory
    match env::current_dir() {
        Ok(path) => {
            shell.working_directory = path.to_string_lossy().to_string();
            shell.exit_code = Some(0);
        }
        Err(e) => {
            eprintln!("cd: unable to get current directory after change: {}", e);
            shell.exit_code = Some(1);
            // Keep old working_directory; it may be inaccurate, but better than panicking.
        }
    }
}

pub fn exit_shell(cli: &ParsedCommand, shell: &mut ShellState) {
    let command_arg: Option<&String> = cli.arguments.get(0);
    let mut code: Option<i8> = command_arg.and_then(|s| s.parse().ok());

    if code == None {
        code = Some(0);
    }

    save_history(shell);

    shell.exit_code = code;
    shell.should_exit = true;
}

pub fn print_working_directory(shell: &mut ShellState) {
    println!("{}", shell.working_directory);

    shell.exit_code = Some(0);
}

pub fn echo(cli: &ParsedCommand, shell: &mut ShellState) {
    let mut output: Vec<String> = Vec::new();

    for arg in &cli.arguments {
        if arg.starts_with('$') {
            let var_name: &str = &arg[1..]; // strip the leading '$'
            match shell.env_vars.get(var_name) {
                Some(value) => output.push(value.clone()),
                None => output.push(String::new()), // undefined variable -> empty string
            }
        } else {
            output.push(arg.clone());
        }
    }

    println!("{}", output.join(" "));

    shell.exit_code = Some(0);
}

// NOTE: for now this function is temporary, until we make the conf file
pub fn export(cli: &ParsedCommand, shell: &mut ShellState) {
    for arg in cli.arguments.iter() {
        if let Some((key, value)) = arg.split_once('=') {
            let expanded_value: String = check_env_vars(value, &shell);
            shell.env_vars.insert(key.to_string(), expanded_value);
        } else {
            shell
                .env_vars
                .entry(arg.clone())
                .or_insert_with(String::new);
        }
    }

    if !shell.reading_config {
        add_config(cli, shell);
    }

    shell.exit_code = Some(0);
}

pub fn unset(cli: &ParsedCommand, shell: &mut ShellState) {
    for arg in cli.arguments.iter() {
        shell.env_vars.remove(arg);
        remove_var_from_config(cli, shell);
    }

    shell.exit_code = Some(0);
}

pub fn source(cli: &ParsedCommand, shell: &mut ShellState) {
    let config_file: File = match File::open(cli.arguments[0].to_string()) {
        Ok(f) => f,
        Err(e) => panic!("Failed to create .ashrc: {}", e),
    };

    config::read_config_file(config_file, shell);
}

pub fn show_history(shell: &mut ShellState) {
    for (i, cmd) in shell.history.iter().enumerate() {
        println!("{:5}  {}", i + 1, cmd);
    }
    shell.exit_code = Some(0);
}

pub fn check_env_vars(input: &str, shell: &ShellState) -> String {
    let mut result: String = String::new();
    let mut chars: Peekable<Chars<'_>> = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            // Check for valid variable name characters
            let mut var_name: String = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphanumeric() || next_c == '_' {
                    var_name.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }

            if !var_name.is_empty() {
                // Replace variable with value, or empty string if not found
                let val: String = shell.env_vars.get(&var_name).cloned().unwrap_or_default();
                result.push_str(&val);
            } else {
                // Handle lone '$' at end of string or followed by non-var char
                result.push('$');
            }
        } else {
            result.push(c);
        }
    }
    result
}
