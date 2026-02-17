mod interface;
mod input_parsing;
use input_parsing::{simple_parse};
fn main() {
    let username = whoami::username().unwrap();
    let hostname = whoami::hostname().unwrap();
    let home = dirs::home_dir().unwrap().to_string_lossy().to_string();
    let working_directory = home.clone();
    let mut input = String::new();
    loop {
        interface::interface(&username, &hostname, &working_directory, &home);
input.clear();        
std::io::stdin().read_line(&mut input).unwrap();
        if input.is_empty() {
            continue;
        }
        
        if input.trim() == "exit" {
            return;
        }
 
  let cli=simple_parse(&input);
        println!("Command: {} ", cli.command);
        println!("Options: {:?} ", cli.options);
        println!("Arguments: {:?}", cli.arguments);
        println!("---");

}

}
