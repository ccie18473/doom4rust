#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod z_zone;

pub use z_zone::*;

use crate::*;

/////////////////////////////
// Z_* Zone memory allocation system
/////////////////////////////

pub struct z {}

impl z {
    pub fn new() -> Self {
        Self {}
    }
}
