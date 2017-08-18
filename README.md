Procedurals
===============
[![Crate](http://meritbadge.herokuapp.com/procedurals)](https://crates.io/crates/procedurals)
[![docs.rs](https://docs.rs/procedurals/badge.svg)](https://docs.rs/procedurals)
[![Build Status](https://travis-ci.org/termoshtt/procedurals.svg?branch=master)](https://travis-ci.org/termoshtt/procedurals)

Collection of basic proc-macros

EnumError
----------

```rust
#[macro_use]
extern crate procedurals;

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
