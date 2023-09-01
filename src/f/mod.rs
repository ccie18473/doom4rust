#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod f_finale;
pub mod f_wipe;

pub use f_finale::*;
pub use f_wipe::*;

use crate::*;

/////////////////////////////
// F_* "Finale" (end of game) and "screen melt" code.
/////////////////////////////

pub struct f<'a> {
    /////////////////////////////
    // f_finale.c
    /////////////////////////////
    // Stage of animation:
    pub pubfinalestage: finalestage_t,

    pub finalecount: u32,

    pub finaletext: &'a str,
    pub finaleflat: &'a str,

    pub castnum: i32,
    pub casttics: i32,
    pub caststate: *mut state_t,
    pub castdeath: bool,
    pub castframes: i32,
    pub castonmelee: i32,
    pub castattacking: bool,
    /////////////////////////////
    // f_wipe.c
    /////////////////////////////
    // when zero, stop the wipe
    pub go: bool,

    pub wipe_scr_start: *mut u8,
    pub wipe_scr_end: *mut u8,
    pub wipe_scr: *mut u8,
    pub y: *mut i32,
}

impl<'a> f<'a> {
    pub fn new() -> Self {
        Self {
            /////////////////////////////
            // f_finale.c
            /////////////////////////////
            pubfinalestage: finalestage_t::F_STAGE_ARTSCREEN,
            finalecount: 0,
            finaletext: "",
            finaleflat: "",

            castnum: 0,
            casttics: 0,
            caststate: ptr::null_mut(),
            castdeath: false,
            castframes: 0,
            castonmelee: 0,
            castattacking: false,

            go: false,

            wipe_scr_start: ptr::null_mut(),
            wipe_scr_end: ptr::null_mut(),
            wipe_scr: ptr::null_mut(),
            y: ptr::null_mut(),
        }
    }
}
