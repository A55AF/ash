use std::collections::HashMap;
use crate::builtin::alias::check_aliases;

mod builtin;
mod commands;
mod interface;
mod parsing;

// mod commands;
use crate::builtin::change_directory_to_home;
use crate::parsing::simple_parse;

pub struct ShellState {
    should_exit: bool,     // set to true when "exit" is called
    exit_code: Option<i8>, // store the exit code
    working_directory: String,
    home: String,
    env_vars: HashMap<String, String>, // Dictionary for the environment variables
    aliases: HashMap<String, String>, // Dictionary for the aliases
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
    };

    change_directory_to_home(&mut shell_state);

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

        if input.trim() == "exit" {
            return;
        }

        input = check_aliases(&input, &mut shell_state);
        println!("{}", input);
        let cli = simple_parse(&input);
        commands::execute_command(&cli, &mut shell_state);
    }
}
