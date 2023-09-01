#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod st_lib;
pub mod st_stuff;

pub use st_lib::*;
pub use st_stuff::*;

use crate::*;

/////////////////////////////
// ST_* Status bar
/////////////////////////////

pub struct st {
    /////////////////////////////
    // st_lib.c
    /////////////////////////////
    pub sttminus: *mut patch_t,
    /////////////////////////////
    // st_stuff.c
    /////////////////////////////
    // graphics are drawn to a backing screen and blitted to the real screen
    pub st_backing_screen: *mut u8,
    // main player in game
    pub plyr: *mut player_t,
    // ST_Start() has just been called
    pub st_firsttime: bool,
    // lump number for PLAYPAL
    pub lu_palette: i32,
    pub st_palette: i32,
    pub st_stopped: bool,
}

impl st {
    pub fn new() -> Self {
        Self {
            /////////////////////////////
            // st_lib.c
            /////////////////////////////
            sttminus: ptr::null_mut(),
            /////////////////////////////
            // st_stuff.c
            /////////////////////////////
            st_backing_screen: ptr::null_mut(),
            plyr: ptr::null_mut(),
            st_firsttime: false,
            lu_palette: 0,
            st_palette: 0,
            st_stopped: true,
        }
    }
}
