use crate::parsing::ParsedCommand;
use crate::ShellState;
use std::env;
use std::path::Path;
use std::fs;
struct Command{
    name: &'static str,
    description: &'static str,
    limit_arg:bool,
    num_args: usize,
    exec: fn(&ParsedCommand, &mut ShellState)
}

static COMMANDS: &[Command] = &[
    //Argument = 0
    Command{
        name: "help",
        description: "Display this list of commands",
        limit_arg: true,
        exec: command_help,
        num_args: 0,
    },
    Command {
        name: "ls",
        description: "List the files and folders",
        limit_arg: true,
        exec: command_ls,
        num_args: 0,
    },

    //Argument = 1
    Command {
        name: "cd",
        description: "Change current directory",
        limit_arg: true,
        exec: command_cd,
        num_args: 1,
    }
];

pub fn execute(cli: &ParsedCommand, curr_state: &mut ShellState){
    if let Some(cmd) = COMMANDS.iter().find(|c| c.name==cli.command){
        if cmd.limit_arg==true {
            let arg_count:usize = cli
                                        .options
                                        .get("ARGUMENTS")
                                        .map(|v| v.len())
                                        .unwrap_or(0);
            if arg_count != cmd.num_args {
                println!("{} expects {} argument(s), got {}",cmd.name, cmd.num_args, arg_count);
                return;
            } //Hehehe, I made a refactor on my own :3

            (cmd.exec)(cli, curr_state);
        }
    }
}

pub fn command_help(cli: &ParsedCommand,curr_state:&mut ShellState){
    let mut i:u8=1;
    for cmd in COMMANDS {
        println!("{}- {}: {}",i,cmd.name,cmd.description);
        i = i+1;
    }
}

pub fn command_cd(cli: &ParsedCommand,curr_state:&mut ShellState){
    let target = cli
        .options
        .get("ARGUMENTS")
        .and_then(|v| v.first())
        .map(|s| s.as_str())
        .unwrap();

    let path = if target == "~" {
        Path::new(&curr_state.home)
    }else if target == ".." {
        let temp =Path::new(&curr_state.working_directory);
        temp.parent().unwrap()
    }
     else {
        Path::new(target)
    };

    match env::set_current_dir(path) {
        Ok(_) => {
            curr_state.working_directory = env::current_dir()
                .unwrap()
                .to_string_lossy()
                .to_string();
        }
        Err(e) => println!("cd: {}", e),
    }
}

pub fn command_ls(cli: &ParsedCommand,curr_state:&mut ShellState){
    let paths = fs::read_dir(&curr_state.working_directory).unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}

