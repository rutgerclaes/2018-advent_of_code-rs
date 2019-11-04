use std::fs;
use std::io::Error;

pub fn load_file(path: &str) -> Result<String, Error> {
  fs::read_to_string(path)
}
