use std::fs::OpenOptions;
use std::os::unix::io::AsRawFd;
pub fn redirect(a : bool, file_name : &str, cmd : &str){
    let a = true;
    let stdout;
    unsafe {
        stdout = libc::dup(libc::STDOUT_FILENO);
        let file = OpenOptions::new().write(true).create(true).append(a).open(file_name).unwrap();
        libc::dup2(file.as_raw_fd(), libc::STDOUT_FILENO);
    }
    // execute command
    unsafe{
        libc::dup2(stdout, libc::STDOUT_FILENO);
        libc::close(stdout);
    }
}