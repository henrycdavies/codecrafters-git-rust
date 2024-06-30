use crate::{commands::{Command, CommandExecutionError, ExecutableCommand}, input::ParsedInput};

pub struct Handler {}

impl Handler {
    pub fn handle_input(&self, input: ParsedInput) -> Result<i32, CommandExecutionError> {
        let command_name = input.command;
        let args = input.args.to_vec();
        let command = Command::new(command_name, args);
        if let Ok(()) = command.execute() {
            return Ok(1);
        }
        return Err(CommandExecutionError {})
    }
}