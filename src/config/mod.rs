use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

use crate::ShellState;
use crate::builtin::alias::check_aliases;
use crate::commands::execute_command;
use crate::simple_parse;

const CONFIG_FILE_NAME: &str = ".ashrc";
const DEFAULT_CONFIG_FILE_CONTENT: &str = "# ~/.bashrc

# Aliases
alias ll='ls -alF'
alias la='ls -A'
alias ..='cd ..'

# PATH
export PATH=\"$HOME/bin:$PATH\"
";

pub fn check_config_file(shell: &mut ShellState) {
    let path = Path::new(CONFIG_FILE_NAME);

    if !path.exists() {
        let mut config_file: File = match File::create(CONFIG_FILE_NAME) {
            Ok(f) => f,
            Err(e) => panic!("Failed to create .ashrc: {}", e),
        };

        config_file
            .write_all(DEFAULT_CONFIG_FILE_CONTENT.as_bytes())
            .expect("Failed to write to .ashrc");
    } else {
        let config_file: File = match File::open(CONFIG_FILE_NAME) {
            Ok(f) => f,
            Err(e) => panic!("Failed to create .ashrc: {}", e),
        };

        read_config_file(config_file, shell);
    }
}

fn read_config_file(file: File, shell: &mut ShellState) {
    let mut reader = io::BufReader::new(file);
    let mut line = String::new();

    loop {
        let bytes_read = match reader.read_line(&mut line) {
            Ok(n) => n,
            Err(e) => panic!("Failed to create .ashrc: {}", e),
        };

        if bytes_read == 0 {
            break; // EOF
        }

        if line.is_empty() || bytes_read == 1 || line.starts_with("#") {
            line.clear();
            continue;
        }

        line = check_aliases(&line, shell);
        let cli = simple_parse(&line);
        execute_command(&cli, shell);

        line.clear();
    }
}
