mod cat_file;
mod init;
mod error;

use std::io::Result;

pub use init::InitCommand;
pub use error::CommandExecutionError;
pub use cat_file::CatFileCommand;

pub trait ExecutableCommand {
    fn execute(&self) -> Result<()>;
}

pub enum Command {
    Init(InitCommand),
    CatFile(CatFileCommand)
}

impl Command {
    pub fn new(command_name: &String, args: Vec<String>) -> Self {
        match command_name.as_str() {
            "init" => Command::Init(InitCommand::new(args.to_vec())),
            "cat-file" => Command::CatFile(CatFileCommand::new(args.to_vec())),
            _ => Command::Init(InitCommand::new(args.to_vec())),
        }
    }
}

impl ExecutableCommand for Command {
    fn execute(self: &Self) -> Result<()> {
        self.execute()
    }
}