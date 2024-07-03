mod cat_file;
mod init;
mod error;

use std::io::Result;

use cat_file::{cat_file, CatFileArgs};
use clap::{Parser, Subcommand};
pub use error::CommandExecutionError;
use init::{init, InitFileArgs};

pub trait ExecutableCommand {
    fn execute(&self) -> Result<()>;
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init(InitFileArgs),
    CatFile(CatFileArgs),
}

impl Commands {
    pub fn new() -> Self {
        let args = Cli::parse();
        args.cmd
    }
}

impl ExecutableCommand for Commands {
    fn execute(&self) -> Result<()> {
        match self {
            Commands::Init(args) => init(args),
            Commands::CatFile(args) => cat_file(args),
        }
    }
}