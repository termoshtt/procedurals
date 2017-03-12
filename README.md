enum-error-derive
==================
[![Crate](http://meritbadge.herokuapp.com/enum-error-derive)](https://crates.io/crates/enum-error-derive)
[![docs.rs](https://docs.rs/enum-error-derive/badge.svg)](https://docs.rs/enum-error-derive)
[![Build Status](https://travis-ci.org/termoshtt/enum-error-derive.svg?branch=master)](https://travis-ci.org/termoshtt/enum-error-derive)

Derive Error traits for Enum Error struct

Example
--------

```rust
#[macro_use]
extern crate enum_error_derive;

use std::{io, fmt};

#[derive(Debug, EnumError)] // EnumError derives From<*>, fmt::Display and error::Error
pub enum Error {
    IO(io::Error),
    Fmt(fmt::Error),
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
```
