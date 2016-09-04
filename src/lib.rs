#![no_std]


pub use core::fmt::{Debug, Display, Formatter, Error};
pub use core::result::Result;
pub use core::intrinsics::transmute;


#[macro_export]
macro_rules! create_errno {
    ($Name: ident, $EnumType: ty, $strings: ident) => (

        #[derive(Clone, Eq, PartialEq)]
        pub struct $Name {
            errno: $EnumType,
        }

        impl $Name {

            pub fn new(errno: $EnumType) -> Self {
                $Name {
                    errno: errno
                }
            }
            pub fn from(errno: u8) -> Self {
                assert!(errno < $strings.len() as u8);
                $Name {
                    errno: unsafe { $crate::transmute(errno) }
                }
            }

            pub fn get_message(&self) -> &str {
                if let Some(description) = $strings.get(self.errno as usize) {
                    description
                } else {
                    "Unknown Error"
                }
            }
            pub fn get_number(&self) -> u8 {
                self.errno as u8
            }
        }

        impl $crate::Debug for $Name {
            fn fmt(&self, f: &mut $crate::Formatter) -> $crate::Result<(), $crate::Error> {
                f.write_str(self.get_message())
            }
        }

        impl $crate::Display for $Name {
            fn fmt(&self, f: &mut $crate::Formatter) -> $crate::Result<(), $crate::Error> {
                f.write_str(self.get_message())
            }
        }
    )
}
