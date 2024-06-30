#[derive(Debug)]
pub struct InputParseError {}

pub struct ParsedInput<'a> {
    pub command: &'a String,
    pub args: &'a [String],
}