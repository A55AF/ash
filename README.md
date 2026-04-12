# ASH - A Simple Shell

A lightweight, feature-rich Linux shell written in Rust. ASH is designed to be portable across any Linux distribution providing essential shell functionality and modern command execution features.

[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GitHub Repository](https://img.shields.io/badge/GitHub-A55AF/ash-black.svg)](https://github.com/A55AF/ash)

## Overview

ASH is a functional Unix shell implementation written entirely in Rust. It successfully handles native command execution, advanced parsing with support for pipes and operators, command history management, and simple configuration through shell scripts.

## Features

### Core Shell Capabilities

- **Command Execution**: Execute any external command with full argument passing
- **Pipeline Support**: Chain commands with pipes (`|`) for data stream processing
- **Conditional Execution**: 
  - AND operator (`&&`) - Execute next command only if previous succeeds
  - OR operator (`||`) - Execute next command only if previous fails
- **Background Processes**: Run commands asynchronously with `&` operator
- **Command History**: Automatic history tracking with configurable size limits
- **Interactive Prompt**: Clean `username@hostname:pwd$` format with home directory abbreviation

### Built-in Commands

| Command | Description |
|---------|-------------|
| `cd [path]` | Change directory with `~` home expansion support |
| `pwd` | Print the current working directory |
| `echo [text]` | Display text with environment variable expansion |
| `export [VAR=value]` | Set environment variables persistently |
| `unset [VAR]` | Remove environment variables |
| `alias [name=command]` | Create command aliases for shortcuts |
| `unalias [-a\|name]` | Remove aliases (use `-a` to clear all) |
| `exit [code]` | Exit the shell with optional exit code |
| `source [file]` | Execute commands from a configuration file |
| `history` | Display the command history |

### Configuration System

- **RC File Support** (`.ashrc`): Auto-created configuration file in the home directory
- **Environment Variables**: Persistent variable storage and expansion
- **Custom Aliases**: Define shortcuts for frequently used commands
  ```bash
  alias ll='ls -alF'
  alias la='ls -A'
  alias ..='cd ..'
  ```
- **Shell Functions**: Define and execute custom functions in configuration
- **PATH Management**: Full environment variable support for system path configuration

### Shell History

- **Automatic Tracking**: All commands automatically saved to history
- **Persistent Storage**: History saved in `.ash_history` file
- **Duplicate Prevention**: Consecutive duplicate commands not recorded
- **Configurable Limits**: Default 1000-command history with customizable size
- **History Retrieval**: Access previous commands with `history` command

### Advanced Features

- **Variable Expansion**: Automatic substitution of `$VARIABLE` references in commands
- **Background Process Management**: Monitor and track background jobs
- **Exit Code Tracking**: Capture and utilize exit codes for conditional execution
- **Quote Handling**: Support for single and double quotes in command parsing
- **Escape Sequences**: Handle escaped characters in command arguments
- **Multi-command Processing**: Parse and execute complex command chains

## Getting Started

### Prerequisites

- Rust 1.70+ or the 2024 edition
- Linux operating system
- Standard library dependencies: `dirs`, `whoami`

### Installation
#### Download Binary (Recommended)
- current : [ash v0.1.0-beta](https://github.com/A55AF/ash/releases/tag/v0.1.0-beta)
```bash
chmod +x ash_v0.1.0-beta_linux-x86_64
sudo mv ash_v0.1.0-beta_linux-x86_64 /usr/local/bin/ash
```
#### Build From Source
```bash
# Clone the repository
git clone https://github.com/A55AF/ash.git
cd ash

# Build the project
cargo build --release

# Run the shell
./target/release/ash
```
## Configuration

The shell automatically creates a `.ashrc` configuration file in your home directory on first run with sensible defaults including common aliases.

### Example `.ashrc` Configuration

```bash
# ~/.ashrc - ASH Shell Configuration

# Aliases for common commands
alias ll='ls -alF'
alias la='ls -A'
alias ..='cd ..'
alias gs='git status'
alias gp='git pull'

# PATH configuration
export PATH="$HOME/.local/bin:$PATH"

# Custom environment variables
export EDITOR=vim
export TERM=xterm-256color

# Custom shell functions
greet() {
    echo "Hello, Tux! Welcome to ASH."
}
```
## Operator Support

| Operator | Behavior | Example |
|----------|----------|---------|
| `\|` | Pipe stdout to next command | `ls \| grep .rs` |
| `&&` | Run next if previous succeeds | `cd dir && ls` |
| `\|\|` | Run next if previous fails | `test -f file \|\| echo "not found"` |
| `&` | Run in background | `sleep 100 &` |

## Known Limitations

- No interactive line editing or completion (future enhancement)
- Functions are limited to basic implementations
- No script scheduling or cron-like features
- Background process monitoring uses polling (marked for event-based refactor)
- No implementation for redirection (future enhancement)
- Limited Shell Scripting and configurations

## Contributing

Contributions are welcome! This project is actively maintained and designed to be:
- A solid foundation for shell features
- Suitable for educational use

## License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.
