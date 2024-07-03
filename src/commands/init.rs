use std::{fs, io::Result};

use clap::Args;

#[derive(Args, Debug)]
pub struct InitFileArgs {}

pub fn init(_args: &InitFileArgs) -> Result<()> {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory");
    Ok(())
}