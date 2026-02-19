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
    let target = cli
        .options
        .get("ARGUMENTS")
        .and_then(|args| args.get(0))
        .map(|s| s.as_str())
        .unwrap_or(&shell.home);

    if let Err(e) = env::set_current_dir(target) {
        eprintln!("cd error: {}", e);
    } else {
        shell.working_directory =
            env::current_dir().unwrap().to_string_lossy().to_string();
    }
}

fn run_external(cli: &ParsedCommand, shell: &ShellState) {
    let mut cmd = Command::new(&cli.command);

    if let Some(args) = cli.options.get("ARGUMENTS") {
        cmd.args(args);
    }

    let status = cmd
        .current_dir(&shell.working_directory)
        .status();

    if let Err(e) = status {
        eprintln!("Execution failed: {}", e);
    }
}
