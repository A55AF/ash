mod interface;

fn main() {
    let username = whoami::username().unwrap();
    let hostname = whoami::hostname().unwrap();
    let home = dirs::home_dir().unwrap().to_string_lossy().to_string();
    let working_directory = home.clone();
    let mut input = String::new();
    loop {
        interface::interface(&username, &hostname, &working_directory, &home);
        std::io::stdin().read_line(&mut input).unwrap();
    }
}
