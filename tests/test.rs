
#[macro_use]
extern crate enum_error_derive;

use std::{io, fs};

#[derive(Debug, EnumError)]
pub enum Error {
    IO(io::Error),
}

#[test]
fn test_all() {
    check_from_converter().unwrap();
}

fn check_from_converter() -> Result<(), Error> {
    let _ = fs::File::create("test_text.txt")?;
    Ok(())
}
