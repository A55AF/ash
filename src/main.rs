use crate::builtin::alias::check_aliases;
use crate::parsing::ParseError;
use crate::parsing::handle_parse;
use std::collections::HashMap;
mod builtin;
mod commands;
mod interface;
mod parsing;
use crate::builtin::change_directory_to_home;
use crate::commands::{Job, execute_full_command, handle_background_processes};
use crate::parsing::split_by_operators;
pub struct ShellState {
    should_exit: bool,     // set to true when "exit" is called
    exit_code: Option<i8>, // store the exit code
    working_directory: String,
    home: String,
    env_vars: HashMap<String, String>, // Dictionary for the environment variables
    aliases: HashMap<String, String>,  // Dictionary for the aliases\
    background_processes: Vec<Job>,
}

fn main() {
    let username = whoami::username().unwrap();

    let hostname = whoami::hostname().unwrap();

    let mut shell_state = ShellState {
        should_exit: false,
        exit_code: Some(0),
        home: dirs::home_dir().unwrap().to_string_lossy().to_string(),
        working_directory: dirs::home_dir().unwrap().to_string_lossy().to_string(),
        env_vars: HashMap::new(),
        aliases: HashMap::new(),
        background_processes: Vec::new(),
    };

    change_directory_to_home(&mut shell_state);

    let mut input = String::new();

    loop {
        if shell_state.should_exit {
            break;
        }

        // TODO: Handle it with a monitor/timer to check for
        // the running processes every 1 second
        handle_background_processes(&mut shell_state);

        interface::interface(
            &username,
            &hostname,
            &shell_state.working_directory,
            &shell_state.home,
        );
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            continue;
        }

        input = check_aliases(&input, &mut shell_state);
        let cli = handle_parse(&input);
        execute_full_command(&cli.unwrap(), &mut shell_state);
    }
}
