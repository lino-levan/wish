use std::fs::metadata;
use std::io::{stderr, stdout, Write};
use std::process::Command;

pub fn run_command(name: &str, args: Vec<&str>) {
    let output = Command::new(name).args(args).output().unwrap();
    stdout().write_all(output.stdout.as_slice()).unwrap();
    stderr().write_all(output.stderr.as_slice()).unwrap();
}

pub fn exists(path: &str) -> bool {
    match metadata(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
