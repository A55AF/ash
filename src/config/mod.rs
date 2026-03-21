use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::ShellState;
use crate::builtin::alias::check_aliases;
use crate::commands::execute_command;
use crate::parsing::ParsedCommand;
use crate::simple_parse;

const CONFIG_FILE_NAME: &str = ".ashrc";
const HISTORY_FILE_NAME: &str = ".ash_history";
const DEFAULT_CONFIG_FILE_CONTENT: &str = "# ~/.ashrc

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
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut line: String = String::new();
    let mut process: String;
    let mut cli: ParsedCommand;
    let mut in_function: bool = false;
    let mut current_function: String = String::new();
    let mut function_body: Vec<String> = Vec::new();
    let mut brace_count: usize = 0;

    shell.reading_config = true;

    loop {
        let bytes_read = match reader.read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(n) => n,
            Err(e) => panic!("Failed to create .ashrc: {}", e),
        };

        if line.is_empty() || line.starts_with("#") || bytes_read == 1 {
            line.clear();
            continue;
        }

        if !in_function {
            if let Some(name) = is_function_definition(&line) {
                in_function = true;
                current_function = name;
                function_body.clear();
                brace_count = line.chars().filter(|&c| c == '{').count();
                // If the opening brace is on the same line, we might already have some body content after it.
                // For simplicity, we'll assume the brace is at the end and the body starts on next lines.
                // You can enhance this to handle "name() { cmd; }" on one line.
                line.clear();
                continue;
            } else {
                process = check_aliases(&line, shell);
                cli = simple_parse(&process);
                execute_command(&cli, &crate::parsing::Operator::Background, shell);
            }
        } else {
            // We are inside a function body
            brace_count += line.chars().filter(|&c| c == '{').count();
            brace_count -= line.chars().filter(|&c| c == '}').count();

            // Add the line to the function body (store the raw line)
            if line.chars().all(|c| c != '{' && c != '}') {
                let trimmed: &str = line.trim();
                function_body.push(trimmed.to_string());
            }

            if brace_count == 0 {
                // Function definition finished
                shell
                    .functions
                    .insert(current_function.clone(), function_body.clone());
                in_function = false;
                current_function.clear();
                function_body.clear();
            }
        }

        line.clear();
    }

    shell.reading_config = false;
}

pub fn add_config(cli: &ParsedCommand, shell: &mut ShellState) {
    let mut command: String = String::from(&cli.command);

    for arg in cli.arguments.iter() {
        command.push(' ');

        // Check if the argument contains whitespace or is empty.
        // If so, wrap it in double quotes to ensure it parses correctly later.
        if arg.contains(' ') || arg.is_empty() {
            // Escape any existing double quotes inside the argument
            let escaped = arg.replace("\"", "\\\"");
            command.push_str(&format!("\"{}\"", escaped));
        } else {
            command.push_str(arg);
        }
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

fn is_function_definition(line: &str) -> Option<String> {
    // Pattern: "name() {" or "function name {"
    let line = line.trim();

    // Case 1: "name() {"
    if line.ends_with("){") || line.ends_with(") {") {
        if let Some(open_paren) = line.find('(') {
            let name = line[..open_paren].trim();
            if is_valid_name(name) {
                return Some(name.to_string());
            }
        }
    }

    // Case 2: "function name {"
    if line.starts_with("function ") && line.contains('{') {
        let after_function = &line["function".len()..].trim_start();
        if let Some(brace_pos) = after_function.find('{') {
            let name = after_function[..brace_pos].trim();
            if is_valid_name(name) {
                return Some(name.to_string());
            }
        }
    }

    None
}

fn is_valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub fn execute_conf_function(fn_name: &str, shell: &mut ShellState) -> bool {
    // First, check if it's a function
    if let Some(body) = shell.functions.get(fn_name).cloned() {
        // Execute the function body
        // For now, we ignore arguments; later we can pass $1, $2 etc.
        for line in body {
            // Expand aliases and parse each line
            let expanded = check_aliases(&line, shell);
            let sub_cli = simple_parse(&expanded);
            execute_command(&sub_cli, &crate::parsing::Operator::Background, shell);
        }
        shell.exit_code = Some(0);
        return true;
    }

    // shell.exit_code = Some(1);
    return false;
}

fn history_file_path(shell: &ShellState) -> PathBuf {
    Path::new(&shell.home).join(HISTORY_FILE_NAME)
}

pub fn load_history(shell: &mut ShellState) {
    let path = history_file_path(shell);
    if !path.exists() {
        return; // nothing to load
    }

    let file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Warning: could not open history file: {}", e);
            shell.exit_code = Some(1);
            return;
        }
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(cmd) => {
                // Optionally trim and skip empty lines
                let cmd = cmd.trim().to_string();
                if !cmd.is_empty() {
                    shell.history.push(cmd);
                }
            }
            Err(e) => {
                eprintln!("Warning: error reading history line: {}", e);
                shell.exit_code = Some(1);
            }
        }
    }

    // Keep only the last `history_max` entries (if you set a limit)
    if shell.history.len() > shell.history_max {
        let start = shell.history.len() - shell.history_max;
        shell.history = shell.history.drain(start..).collect();
    }

    shell.exit_code = Some(0);
}

pub fn save_history(shell: &mut ShellState) {
    let path: PathBuf = history_file_path(shell);
    // Open in write mode, create if not exists, truncate (overwrite)
    let mut file: File = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Warning: could not write history file: {}", e);
            shell.exit_code = Some(1);
            return;
        }
    };

    for cmd in &shell.history {
        if let Err(e) = writeln!(file, "{}", cmd) {
            eprintln!("Warning: error writing history: {}", e);
            shell.exit_code = Some(1);
        }
    }

    shell.exit_code = Some(0);
}
