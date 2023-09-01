#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod deh_main;
pub mod deh_misc;
pub mod deh_str;

pub use deh_main::*;
pub use deh_misc::*;
pub use deh_str::*;

use crate::*;

/////////////////////////////
// DEH_* Dehacking code
/////////////////////////////

pub struct deh {}

impl deh {
    pub fn new() -> Self {
        Self {}
    }
}
