#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod v_patch;
pub mod v_video;

pub use v_patch::*;
pub use v_video::*;

use crate::*;

/////////////////////////////
// V_* General graphic rendering
/////////////////////////////

pub struct v {
    /////////////////////////////
    // v_video.c
    /////////////////////////////
    // Blending table used for fuzzpatch, etc.
    // Only used in Heretic/Hexen
    pub tinttable: *mut u8,

    // villsa [STRIFE] Blending table used for Strife
    pub xlatab: *mut u8,

    // The screen buffer that the v_video.c code draws to.
    pub dest_screen: *mut u8,

    pub dirtybox: [i32; 4],

    // haleyjd 08/28/10: clipping callback function for patches.
    // This is needed for Chocolate Strife, which clips patches to the screen.
    pub patchclip_callback: vpatchclipfunc_t,
}

impl v {
    pub fn new() -> Self {
        Self {
            tinttable: ptr::null_mut(),
            xlatab: ptr::null_mut(),
            dest_screen: ptr::null_mut(),
            dirtybox: [0; 4],
            patchclip_callback: func,
        }
    }
}

pub fn func(patch: *mut patch_t, x: i32, y: i32) -> bool {
    return false;
}
