#![no_std]


pub use core::fmt::{Debug, Display, Formatter, Error};
pub use core::result::Result;


#[macro_export]
macro_rules! create_errno {
    ($Name: ident, $EnumType: ty, $strings: ident) => (


        pub struct $Name {
            errno: $EnumType,
        }

        impl $Name {

            pub fn new(errno: $EnumType) -> Self {
                $Name {
                    errno: errno
                }
            }

            pub fn get_message(&self) -> &str {
                if let Some(description) = $strings.get(self.errno as usize) {
                    description
                } else {
                    "Unknown Error"
                }
            }
            pub fn get_number(&self) -> usize {
                self.errno as usize
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
