use std::fs;

use super::ExecutableCommand;

pub struct InitCommand {}

impl InitCommand {
    pub fn new(args: Vec<String>) -> Self {
        return Self { }
    }
}

impl ExecutableCommand for InitCommand {
    fn execute(&self) -> std::io::Result<()> {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory");
        Ok(())
    }
}