#![feature(collections)]
#![no_std]


#[macro_use(format)]
extern crate collections;


#[macro_use]
extern crate errno;


#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ErrorCode {
    SomeErrorCode = 0,
}


static ERROR: [&'static str; 1] = ["Some Error Message"];


create_errno!(Error, ErrorCode, ERROR);


#[test]
fn test_enums() {
    let error = Error::new(ErrorCode::SomeErrorCode);

    assert_eq!(error.code(), 0);
    assert_eq!(error.message(), "Some Error Message");
    assert_eq!(format!("{:?}", error), "Some Error Message");
}

#[test]
fn test_number() {
    let error = Error::from(0);

    assert_eq!(error.code(), 0);
    assert_eq!(error.message(), "Some Error Message");
    assert_eq!(format!("{:?}", error), "Some Error Message");
}
