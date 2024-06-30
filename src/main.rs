#[allow(unused_imports)]
use std::env;

use handler::Handler;
use input::{InputParseError, ParsedInput};

mod commands;
mod handler;
mod input;

fn parse_args<'a>(args: &'a[String]) -> Result<ParsedInput<'a>, InputParseError> {
    let subcommand_and_args = &args[1..];
    let split_args = subcommand_and_args.split_first();
    if let Some((command_arg, add_args)) = split_args {
        return Ok(ParsedInput { command: command_arg, args: add_args })
    }
    Err(InputParseError{})
}

fn main() {
    let handler = Handler{};
    let args: Vec<String> = env::args().collect();
    let parse_input_result = parse_args(args.as_slice());
    if let Ok(inp) = parse_input_result {
        let res = handler.handle_input(inp);
        if let Ok(code) = res {
            std::process::exit(code)
        }
    }
    println!("Invalid empty input.");
    std::process::exit(1);
}
