use std::process::Command;
use std::env;
use crate::parsing::ParsedCommand;
use crate::ShellState;

pub fn execute_command(cli: &ParsedCommand, shell: &mut ShellState) {
    match cli.command.as_str() {
        "cd" => change_directory(cli, shell),
        _ => run_external(cli, shell),
    }
}

fn change_directory(cli: &ParsedCommand, shell: &mut ShellState) {
    let target = if cli.arguments.is_empty() {&shell.home.as_str()} else {cli.arguments[0].as_str()};

    if let Err(e) = env::set_current_dir(target) {
        eprintln!("cd error: {}", e);
    } else {
        shell.working_directory =
            env::current_dir().unwrap().to_string_lossy().to_string();
    }
}

fn run_external(cli: &ParsedCommand, shell: &ShellState) {
    let mut cmd = Command::new(&cli.command);

    if cli.command.is_empty() {
        return;
    }

    cmd.args(cli.arguments.clone());

    let status = cmd
        .current_dir(&shell.working_directory)
        .status();

    if let Err(e) = status {
        eprintln!("Execution failed: {}", e);
    }
}
