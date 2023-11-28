use std::fs::metadata;
use std::process::Command;

pub fn run_command(name: &str, args: Vec<&str>) {
    // spawn child process
    let mut child = Command::new(name).args(args).spawn().unwrap();

    // wait for child to finish
    child.wait().unwrap();
}

pub fn exists(path: &str) -> bool {
    match metadata(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
