mod builtin;
mod commands;
mod interface;
mod parsing;
// mod commands;
use crate::parsing::simple_parse;

pub struct ShellState {
    pub should_exit: bool,     // set to true when "exit" is called
    pub exit_code: Option<i8>, // store the exit code
    working_directory: String,
    home: String,
}

fn main() {
    let username = whoami::username().unwrap();

    let hostname = whoami::hostname().unwrap();

    let mut shell_state = ShellState {
        should_exit: false,
        exit_code: Some(0),
        home: dirs::home_dir().unwrap().to_string_lossy().to_string(),
        working_directory: dirs::home_dir().unwrap().to_string_lossy().to_string(),
    };

    let mut input = String::new();

    loop {
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

        let cli = simple_parse(&input);
        commands::execute_command(&cli, &mut shell_state);
    }
}
