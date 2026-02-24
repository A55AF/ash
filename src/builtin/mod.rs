pub mod alias;

use std::env;
use crate::ShellState;
use crate::parsing::ParsedCommand;

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
