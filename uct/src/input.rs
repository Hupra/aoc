use std::fs;
use std::io::{ self, Read };

pub enum InputSource {
    File(String),
    Direct(String),
    Stdin,
}

pub fn read_input(source: InputSource) -> Result<String, std::io::Error> {
    match source {
        InputSource::File(path) => fs::read_to_string(&path),
        InputSource::Direct(content) => Ok(content),
        InputSource::Stdin => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            Ok(input)
        }
    }
}
