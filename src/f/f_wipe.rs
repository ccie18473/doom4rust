#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// f_wipe.h
/////////////////////////////
//
// DESCRIPTION:
//	Mission begin melt/wipe screen special effect.
//

//
//                       SCREEN WIPE PACKAGE
//

pub enum WIPE {
    // simple gradual pixel change for 8-bit only
    wipe_ColorXForm,

    // weird screen melt
    wipe_Melt,

    wipe_NUMWIPES,
}

/////////////////////////////
// f_wipe.c
/////////////////////////////
//
// DESCRIPTION:
//	Mission begin melt/wipe screen special effect.
//

pub fn wipe_shittyColMajorXform(array: *mut i16, width: i32, height: i32) {}

pub fn wipe_initColorXForm(width: i32, height: i32, ticks: i32) -> i32 {
    return 0;
}

pub fn wipe_doColorXForm(width: i32, height: i32, ticks: i32) -> i32 {
    return 0;
}

pub fn wipe_exitColorXForm(width: i32, height: i32, ticks: i32) -> i32 {
    return 0;
}

pub fn wipe_initMelt(width: i32, height: i32, ticks: i32) -> i32 {
    return 0;
}

pub fn wipe_doMelt(width: i32, height: i32, ticks: i32) -> i32 {
    return 0;
}

pub fn wipe_exitMelt(width: i32, height: i32, ticks: i32) -> i32 {
    return 0;
}

pub fn wipe_StartScreen(x: i32, y: i32, width: i32, height: i32) -> i32 {
    return 0;
}

pub fn wipe_EndScreen(x: i32, y: i32, width: i32, height: i32) -> i32 {
    return 0;
}

pub fn wipe_ScreenWipe(wipeno: i32, x: i32, y: i32, width: i32, height: i32, ticks: i32) -> i32 {
    return 0;
}
