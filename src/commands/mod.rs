use crate::ShellState;
use crate::builtin::{change_directory, echo, export, unset};
use crate::builtin::exit_shell;
use crate::builtin::print_working_directory;
use crate::builtin::alias::{alias, unalias};
use crate::parsing::{Operator, ParsedCommand};
use std::process::Command;

pub fn execute_full_command(commands: &Vec<(ParsedCommand, Operator)>, shell: &mut ShellState) {
    // Store the operator between the current command and the last command
    let mut last_command_operation: Operator = Operator::None;
    for (parsed_command, operator) in commands {
        // Skip the current command if the last command failed in the AND 
        // or if the last command success in the OR
        if (last_command_operation == Operator::And && shell.exit_code != Some(0)) 
        || (last_command_operation == Operator::Or && shell.exit_code == Some(0)) {
            // Update the operator
            last_command_operation = operator.clone();
            continue;
        }

        last_command_operation = operator.clone();
        execute_command(parsed_command, shell);
    }
}

pub fn execute_command(cli: &ParsedCommand, shell: &mut ShellState) {
    match cli.command.as_str() {
        "cd" => change_directory(cli, shell),
        "exit" => exit_shell(cli, shell),
        "pwd" => print_working_directory(shell),
        "echo" => echo(cli, shell),
        "export" => export(cli, shell),
        "unset" => unset(cli, shell),
        "alias" => alias(cli, shell),
        "unalias" => unalias(cli, shell),
        _ => run_external(cli, shell),
    }
}

fn run_external(cli: &ParsedCommand, shell: &mut ShellState) {
    let mut cmd = Command::new(&cli.command);

    let status = cmd.args(cli.arguments.clone()).status();

    if let Err(ref e) = status {
        eprintln!("Execution failed: {}", e);
    }

    shell.exit_code = Some(status.unwrap().code().unwrap().try_into().unwrap());
}