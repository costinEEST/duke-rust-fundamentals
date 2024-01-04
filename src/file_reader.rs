use std::env::args;
use std::io::{self, ErrorKind};

// cargo run -- file_path
pub fn read_from_file() -> Result<String, std::io::Error> {
    let arguments: Vec<String> = args().collect();

    if arguments.len() < 2 {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "No file path provided",
        ));
    }

    std::fs::read_to_string(&arguments[1])
}
