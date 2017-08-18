
#[macro_use]
extern crate procedurals;

use std::{fmt, io};

#[derive(Debug, EnumError)]
pub enum Error {
    IO(io::Error),
    Fmt(fmt::Error),
}

#[test]
fn test_all() {
    merged_error().unwrap();
}

fn io_error() -> Result<(), io::Error> {
    Ok(())
}

fn fmt_error() -> Result<(), fmt::Error> {
    Ok(())
}

fn merged_error() -> Result<(), Error> {
    io_error()?;
    fmt_error()?;
    Ok(())
}
