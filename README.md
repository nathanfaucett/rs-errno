errno [![Build Status](https://travis-ci.org/nathanfaucett/rs-errno.svg?branch=master)](https://travis-ci.org/nathanfaucett/rs-errno)
=====

macros for creating number based errors

```rust
#[macro_use]
extern crate errno;


#[derive(Clone, Copy)]
pub enum ErrorCode {
    SomeErrorCode = 0,
}


pub static ERROR: [&'static str; 1] = ["Some Error Message"];


create_errno!(Error, ErrorCode, ERROR);


fn example() {
    let error = Error::new(ErrorCode::SomeErrorCode);
    error.get_message(); // "Some Error Message"
}
```
