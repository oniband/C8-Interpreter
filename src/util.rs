use std::env;
use std::fmt;

type Result<String> = std::result::Result<String, ArgError>;

#[derive(Debug, Clone)]
pub struct ArgError;

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid arguments!, usage: chipp <path_to_rom>")
    }
}

pub fn validate_args() -> Result<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        Err(ArgError)
    } else {
        Ok(args[1].clone())
    }
}
