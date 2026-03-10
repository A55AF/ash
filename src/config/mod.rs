use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

use crate::ShellState;
use crate::builtin::alias::check_aliases;
use crate::commands::execute_command;
use crate::parsing::ParsedCommand;
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

pub fn read_config_file(file: File, shell: &mut ShellState) {
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    shell.reading_config = true;

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

    shell.reading_config = false;
}

pub fn add_config(cli: &ParsedCommand, shell: &mut ShellState) {
    let mut command: String = String::from(&cli.command);

    for arg in cli.arguments.iter() {
        command.push_str(" ");
        command.push_str(arg);
    }

    command.push('\n');

    let mut config_file: File = match OpenOptions::new()
        .create(true)
        .append(true)
        .open(CONFIG_FILE_NAME)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("add_config: failed to open config file: {}", e);
            shell.exit_code = Some(1);
            return;
        }
    };

    if let Err(e) = config_file.write_all(command.as_bytes()) {
        eprintln!("add_config: failed to write to config file: {}", e);
        shell.exit_code = Some(1);
        return;
    }

    shell.exit_code = Some(0);
}

pub fn remove_var_from_config(cli: &ParsedCommand, shell: &mut ShellState) {
    let path = Path::new(CONFIG_FILE_NAME);
    if !path.exists() {
        shell.exit_code = Some(1);
        return; // nothing to do
    }

    // Read all lines
    let file: File = File::open(path).expect("Can't open config file");
    let reader: BufReader<File> = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to read config file");

    let mut removed: u8 = 0;
    let mut new_lines: Vec<String> = Vec::new();

    // Patterns to match: "export VARNAME=..." and optionally "VARNAME=..."
    // We'll use starts_with for simplicity.
    let export_pattern: String = format!("export {}=", cli.arguments[0].as_str());
    let bare_pattern: String = format!("{}=", cli.arguments[0].as_str());

    for line in lines {
        let trimmed: &str = line.trim_start();
        if trimmed.starts_with(&export_pattern) || trimmed.starts_with(&bare_pattern) {
            removed += 1;
            // Skip the line (do not push it) – effectively removing it
        } else {
            new_lines.push(line);
        }
    }

    if removed > 0 {
        // Write back the modified content
        let mut file: File = File::create(path).expect("Can't open config file");
        for line in new_lines {
            writeln!(file, "{}", line).expect("Failed to write on config file");
        }
    }

    shell.exit_code = Some(0);
}
