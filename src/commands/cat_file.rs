use super::ExecutableCommand;

pub struct CatFileCommand {
    pub args: Vec<String>,
}

impl CatFileCommand {
    pub fn new(args: Vec<String>) -> Self {
        return CatFileCommand  { args }
    }
}

impl ExecutableCommand for CatFileCommand {
    fn execute(&self) -> std::io::Result<()> {
        unimplemented!()
    }
}