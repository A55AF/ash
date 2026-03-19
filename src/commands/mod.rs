use crate::ShellState;
use crate::builtin::alias::{alias, unalias};
use crate::builtin::exit_shell;
use crate::builtin::print_working_directory;
use crate::builtin::{change_directory, echo, export, unset};
use crate::parsing::{Operator, ParsedCommand};
use std::process::{Child, Command};

pub struct Job {
    index: i8,
    children: Vec<Child>,
    command: String,
}

pub fn execute_full_command(commands: &Vec<(ParsedCommand, Operator)>, shell: &mut ShellState) {
    // Store the operator between the current command and the last command
    let mut last_command_operation: Operator = Operator::None;
    let mut pipeline: Vec<ParsedCommand> = Vec::new();
    for (parsed_command, operator) in commands {
        // Skip the current command if the last command failed in the AND
        // or if the last command success in the OR
        if (last_command_operation == Operator::And && shell.exit_code != Some(0))
            || (last_command_operation == Operator::Or && shell.exit_code == Some(0))
        {
            // Update the operator except if it's pipe
            // because we consider that the pipeline command is a one full command
            if operator != &Operator::Pipe {
                last_command_operation = operator.clone();
            }
            continue;
        }
        if operator != &Operator::Pipe {
            last_command_operation = operator.clone();
        }
        pipeline.push(parsed_command.clone());
        if operator == &Operator::Pipe {
            continue;
        }
        if pipeline.len() > 1 {
            run_pipeline(&pipeline, &operator, shell);
            pipeline.clear();
        } else {
            execute_command(parsed_command, &operator, shell);
        }
    }
}

pub fn execute_command(cli: &ParsedCommand, operator: &Operator, shell: &mut ShellState) {
    let is_background = if operator == &Operator::Background {
        true
    } else {
        false
    };
    match cli.command.as_str() {
        "cd" => run_builtin(cli, is_background, shell, change_directory),
        "exit" => exit_shell(cli, shell),
        "pwd" => run_builtin(cli, is_background, shell, |_, s| print_working_directory(s)),
        "echo" => run_builtin(cli, is_background, shell, echo),
        "export" => run_builtin(cli, is_background, shell, export),
        "unset" => run_builtin(cli, is_background, shell, unset),
        "alias" => run_builtin(cli, is_background, shell, alias),
        "unalias" => run_builtin(cli, is_background, shell, unalias),
        _ => run_external(cli, is_background, shell),
    }
}

fn run_builtin<F>(cli: &ParsedCommand, is_background: bool, shell: &mut ShellState, func: F)
where
    F: FnOnce(&ParsedCommand, &mut ShellState),
{
    if is_background {
        println!("    [{}] {}", shell.background_processes.len() + 1, std::process::id());
        func(cli, shell);
        println!("    [{}] + done       {}", shell.background_processes.len() + 1, cli.command);
    } else {
        func(cli, shell);
    }
}

fn run_external(cli: &ParsedCommand, is_background: bool, shell: &mut ShellState) {
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
            children: vec![child],
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
        job.children.retain_mut(|child| match child.try_wait() {
            Ok(Some(_status)) => false,
            Ok(None) => true,
            Err(e) => {
                eprintln!("Error checking process {}", e);
                false
            }
        });

        // The job is done when all it's children is done
        if job.children.is_empty() {
            println!("    [{}] + done       {}", job.index, job.command);
            false
        } else {
            true
        }
    });
}

fn run_pipeline(pipeline: &Vec<ParsedCommand>, operator: &Operator, shell: &mut ShellState) {
    let is_background = *operator == Operator::Background;
    let mut children: Vec<Child> = Vec::with_capacity(pipeline.len());
    let mut prev_stdout = None;

    // Create the full command string for display
    let mut full_command: String = String::new();
    for (i, cmd) in pipeline.iter().enumerate() {
        full_command.push_str(&cmd.command);
        if i < pipeline.len() - 1 {
            full_command.push_str(" | ");
        }
    }

    for (i, cmd) in pipeline.iter().enumerate() {
        let mut command = Command::new(&cmd.command);
        command.args(&cmd.arguments);

        // set up input for the first command
        if i == 0 {
            // in the first command we use the input we get from the terminal
            command.stdin(std::process::Stdio::inherit());
        } else {
            // we connect the pipes, the input of the current
            // command is the output of the last command
            if let Some(prev) = prev_stdout.take() {
                command.stdin(prev);
            } else {
                eprintln!("Internal error: pipe missing");
                break;
            }
        }

        // Setup Stdout
        if i == pipeline.len() - 1 {
            // in the last command, we print the output to the termainl
            command.stdout(std::process::Stdio::inherit());
        } else {
            // if not, we will create a new pipe
            // so the current command will add it's output to
            // the write end of this pipe
            command.stdout(std::process::Stdio::piped());
        }

        // for the errors we print all of it in the terminal directly
        command.stderr(std::process::Stdio::inherit());

        // After we handles the pipes of every command, we run the command
        match command.spawn() {
            Ok(mut child) => {
                // if it runs successfully, we take the output of the command
                // and save it to handle the pipes for the next command
                if i < pipeline.len() - 1 {
                    prev_stdout = child.stdout.take();
                }
                children.push(child);
            }
            Err(e) => {
                eprintln!("Failed to spawn command in pipeline: {}", e);

                for mut child in children {
                    let _ = child.kill();
                    let _ = child.wait();
                }
                shell.exit_code = Some(1);
                return;
            }
        }
    }

    // handle if the pipeline command is a background process
    if is_background {
        let job_index = (shell.background_processes.len() + 1) as i8;
        let pid = children.first().map(|c| c.id()).unwrap_or(0);
        shell.background_processes.push(Job {
            index: job_index,
            children,
            command: full_command,
        });
        println!("    [{}] {}", job_index, pid);
        shell.exit_code = Some(0)
    } else {
        let mut last_status: i32 = 0;
        for mut child in children {
            let status = child.wait().unwrap();
            last_status = status.code().unwrap_or(1);
        }
        shell.exit_code = Some(last_status as i8);
    }
}
