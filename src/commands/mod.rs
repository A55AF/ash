use crate::ShellState;
use crate::builtin::alias::{alias, unalias};
use crate::builtin::exit_shell;
use crate::builtin::print_working_directory;
use crate::builtin::source;
use crate::builtin::{change_directory, echo, export, show_history, unset};
use crate::config::execute_conf_function;
use crate::parsing::ParsedCommand;

use std::process::Command;

pub fn execute_command(cli: &ParsedCommand, shell: &mut ShellState) {
    if !execute_conf_function(&cli.command, shell) {
        match cli.command.as_str() {
            "cd" => change_directory(cli, shell),
            "exit" => exit_shell(cli, shell),
            "pwd" => print_working_directory(shell),
            "echo" => echo(cli, shell),
            "export" => export(cli, shell),
            "unset" => unset(cli, shell),
            "alias" => alias(cli, shell),
            "unalias" => unalias(cli, shell),
            "source" => source(cli, shell),
            "history" => show_history(shell),
            _ => run_external(cli, shell),
        }
    }
}

fn run_external(cli: &ParsedCommand, shell: &mut ShellState) {
    if cli.command.is_empty() {
        return;
    }

    let mut cmd = Command::new(&cli.command);

    // let status;

    cmd.args(&cli.arguments)
        .current_dir(&shell.working_directory)
        .envs(&shell.env_vars);

    match cmd.status() {
        Ok(status) => {
            shell.exit_code = status.code().map(|c| c as i8);
        }
        Err(e) => {
            eprintln!("{}: {}", cli.command, e);
            shell.exit_code = Some(127);
        }
    }

    // if let Err(e) = status {
    //     eprintln!("Execution failed: {}", e);
    // }
}
