use crate::ShellState;
use crate::builtin::change_directory;
use crate::builtin::exit_shell;
use crate::parsing::ParsedCommand;
// use std::env;
use std::process::Command;

pub fn execute_command(cli: &ParsedCommand, shell: &mut ShellState) {
    match cli.command.as_str() {
        "cd" => change_directory(cli, shell),
        "exit" => exit_shell(cli, shell),
        _ => run_external(cli, shell),
    }
}

fn run_external(cli: &ParsedCommand, shell: &ShellState) {
    let mut cmd = Command::new(&cli.command);

    if cli.command.is_empty() {
        return;
    }

    cmd.args(cli.arguments.clone());

    let status = cmd.current_dir(&shell.working_directory).status();

    if let Err(e) = status {
        eprintln!("Execution failed: {}", e);
    }
}
