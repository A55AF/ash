use std::io::Write;

pub(crate) fn interface(username: &str, hostname: &str, working_directory: &str, home: &str) {
    let pwd = if working_directory == home { "~" } else { working_directory };

    print!("{}@{} {}> ", username, hostname, pwd);
    std::io::stdout().flush().unwrap();
}