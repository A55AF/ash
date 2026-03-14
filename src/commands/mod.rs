use crate::ShellState;
use crate::builtin::{change_directory, echo, export, unset};
use crate::builtin::exit_shell;
use crate::builtin::print_working_directory;
use crate::builtin::alias::{alias, unalias};
use crate::parsing::{Operator, ParsedCommand};
use std::process::{Child, Command};

pub struct Job {
    index: i8,
    child: Child,
    command: String,
}

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
        execute_command(parsed_command, &operator, shell);
    }
}

pub fn execute_command(cli: &ParsedCommand, operator: &Operator ,shell: &mut ShellState) {
    match cli.command.as_str() {
        "cd" => change_directory(cli, shell),
        "exit" => exit_shell(cli, shell),
        "pwd" => print_working_directory(shell),
        "echo" => echo(cli, shell),
        "export" => export(cli, shell),
        "unset" => unset(cli, shell),
        "alias" => alias(cli, shell),
        "unalias" => unalias(cli, shell),
        _ => {
            if operator == &Operator::Background {
                run_external(cli, true, shell);
            } else {
                run_external(cli, false, shell);
            }
        },
    }
}

fn run_external(cli: &ParsedCommand,is_background: bool ,shell: &mut ShellState) {
    let mut cmd = Command::new(&cli.command);

    // Run the process and handle if it's failed
    let running_process = cmd.args(&cli.arguments).spawn();
    if let Err(ref e) = running_process {
        eprintln!("Execution failed: {}", e)
    }

    // Get the processs id and the child itself to handle
    // if it's a background process
    let mut child = running_process.unwrap();
    let pid = child.id();
    let mut status = Some(0);

    if is_background {
        // Get the last index inthe background processes 
        // and concatenate the full command to print
        let job_index = (shell.background_processes.len() + 1) as i8;

        shell.background_processes.push(Job {
            index: job_index,
            child,
            command: cli.command.clone(),
        });
        println!("    [{}] {}", job_index, pid);
    } else {
        status = child.wait().unwrap().code();
    }

    shell.exit_code = Some(status.unwrap() as i8);
}

pub fn handle_background_processes(shell: &mut ShellState) {
    shell.background_processes.retain_mut(|job| {
        match job.child.try_wait() {
            Ok(Some(_status)) => {
                println!("    [{}] + done       {}", job.index, job.command);
                false
            },
            Ok(None) => true,
            Err(e) => {
                eprintln!("Error checking process {}", e);
                false
            },
        }
    });
}