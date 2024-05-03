use std::io;

const READ_ERROR_MESSAGE: &str = "Failed to read line";
const PARSE_ERROR_MESSAGE: &str = "Please type a number!";

/// Reads a line from stdin and attempts to parse it into a usize.
pub fn read_usize_from_user() -> Result<usize, Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| READ_ERROR_MESSAGE)?;
    input
        .trim()
        .parse::<usize>()
        .map_err(|_| PARSE_ERROR_MESSAGE.into())
}
