use crate::builtin::alias::check_aliases;
use std::collections::HashMap;
// use std::collections::hash_map::Keys;

mod builtin;
mod commands;
mod config;
mod interface;
mod parsing;

// mod commands;
use crate::builtin::change_directory_to_home;
use crate::config::check_config_file;
use crate::config::load_history;
use crate::parsing::simple_parse;

const MAX_HISTORY_SIZE: usize = 1000;

pub struct ShellState {
    should_exit: bool, // set to true when "exit" is called
    reading_config: bool,
    exit_code: Option<i8>, // store the exit code
    working_directory: String,
    home: String,
    env_vars: HashMap<String, String>, // Dictionary for the environment variables
    aliases: HashMap<String, String>,  // Dictionary for the aliases
    functions: HashMap<String, Vec<String>>,
    history: Vec<String>,
    history_max: usize,
}

fn main() {
    let username = whoami::username().unwrap();

    let hostname = whoami::hostname().unwrap();

    let mut shell_state = ShellState {
        should_exit: false,
        reading_config: false,
        exit_code: Some(0),
        home: dirs::home_dir().unwrap().to_string_lossy().to_string(),
        working_directory: dirs::home_dir().unwrap().to_string_lossy().to_string(),
        env_vars: std::env::vars().collect(),
        aliases: HashMap::new(),
        functions: HashMap::new(),
        history: Vec::new(),
        history_max: MAX_HISTORY_SIZE,
    };

    change_directory_to_home(&mut shell_state);

    check_config_file(&mut shell_state);

    load_history(&mut shell_state);

    for env in shell_state.env_vars.iter() {
        unsafe {
            std::env::set_var(env.0.to_string(), env.1.to_string());
        }
    }

    let mut input = String::new();

    loop {
        if shell_state.should_exit {
            break;
        }

        interface::interface(
            &username,
            &hostname,
            &shell_state.working_directory,
            &shell_state.home,
        );

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.is_empty() {
            continue;
        }

        let trimmed: &str = input.trim();
        if !trimmed.is_empty() {
            // Optional: avoid duplicates with the previous command
            if shell_state.history.last() != Some(&trimmed.to_string()) {
                shell_state.history.push(trimmed.to_string());
            }
        }

        if shell_state.history.len() > shell_state.history_max {
            shell_state.history.remove(0);
        }

        input = check_aliases(&input, &mut shell_state);
        input = builtin::expand_env_vars(&input, &mut shell_state);
        let cli = simple_parse(&input);
        commands::execute_command(&cli, &mut shell_state);
    }
}
