use crate::ShellState;
use crate::builtin::{change_directory, echo, export, unset};
use crate::builtin::exit_shell;
use crate::builtin::print_working_directory;
use crate::builtin::alias::set_alias;
use crate::parsing::ParsedCommand;
// use std::env;
use std::process::Command;

pub fn execute_command(cli: &ParsedCommand, shell: &mut ShellState) {
    match cli.command.as_str() {
        "cd" => change_directory(cli, shell),
        "exit" => exit_shell(cli, shell),
        "pwd" => print_working_directory(shell),
        "echo" => echo(cli, shell),
        "export" => export(cli, shell),
        "unset" => unset(cli, shell),
        "alias" => set_alias(cli, shell),
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
