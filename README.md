Procedurals
===============
[![Crate](http://meritbadge.herokuapp.com/procedurals)](https://crates.io/crates/procedurals)
[![docs.rs](https://docs.rs/procedurals/badge.svg)](https://docs.rs/procedurals)
[![Build Status](https://travis-ci.org/termoshtt/procedurals.svg?branch=master)](https://travis-ci.org/termoshtt/procedurals)

Collection of basic proc-macros


IntoEnum
---------

```rust
#[macro_use]
extern crate procedurals;

struct A {}
struct B {}

#[derive(IntoEnum)] // derives From<A> and From<B> for E
enum E {
    A(A),
    B(B),
}
```

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
```

NewType
---------

```rust
#[macro_use]
extern crate procedurals;

struct B {}

#[derive(NewType)] // NewType derives From<B>, Deref, and DerefMut
struct A(B);
```
