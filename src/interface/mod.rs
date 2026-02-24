use std::io::Write;

pub(crate) fn interface(username: &str, hostname: &str, working_directory: &str, home: &str) {
    let pwd = if working_directory.starts_with(home) {
        if working_directory.len() == home.len() {
            // Exactly the home directory
            "~".to_string()
        } else {
            // Check that the next character is a path separator (to avoid false positives like /home/user2)
            let after_home = &working_directory[home.len()..];
            if after_home.starts_with('/') {
                format!("~{}", after_home) // e.g., ~/projects
            } else {
                // Not a true subdirectory (e.g., /home/user2 when home is /home/user)
                working_directory.to_string()
            }
        }
    } else {
        // Outside home – show full path
        working_directory.to_string()
    };

    print!("{}@{}:{}$ ", username, hostname, pwd);
    std::io::stdout().flush().unwrap();
}
