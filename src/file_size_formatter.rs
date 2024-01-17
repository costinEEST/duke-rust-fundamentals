// `cargo run "24 mb"` should return `Sizes { bytes: "24000000 bytes", kilobytes: "24000 kilobytes", megabytes: "24 megabytes", gigabytes: "0 gigabytes" }`

use std::env::args;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
struct ByteCount {
    bytes: f64,
}

impl ByteCount {
    fn new(bytes: f64) -> Self {
        Self { bytes }
    }

    fn kilobytes(&self) -> f64 {
        self.bytes as f64 / 1024.0
    }

    fn megabytes(&self) -> f64 {
        self.kilobytes() / 1024.0
    }

    fn gigabytes(&self) -> f64 {
        self.megabytes() / 1024.0
    }

    fn to_string(&self) -> String {
        format!(
            "Sizes {{ bytes: \"{} bytes\", kilobytes: \"{} kilobytes\", megabytes: \"{} megabytes\", gigabytes: \"{} gigabytes\" }}",
            self.bytes,
            self.kilobytes(),
            self.megabytes(),
            self.gigabytes()
        )
    }

    fn parse_size(&self, size_str: &str) -> Result<ByteCount, Error> {
        let parsed_bytes = match size_str.trim().to_lowercase().as_str() {
            "bytes" => self.bytes,
            "kilobytes" => self.kilobytes(),
            "megabytes" => self.megabytes(),
            "gigabytes" => self.gigabytes(),
            _ => return Err(Error::new(ErrorKind::InvalidInput, "Invalid size unit")),
        };

        Ok(ByteCount::new(parsed_bytes))
    }
}

pub fn format_file_size() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<String> = args().collect();

    if arguments.len() < 2 {
        return Err(Box::new(Error::new(
            ErrorKind::InvalidInput,
            "Missing size argument. Run `cargo run -- '24 mb'`",
        )));
    }

    let file_size = ByteCount::parse_size(&arguments[1])?;

    println!("{}", file_size.to_string());
    Ok(())
}
