#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// i_input.c
/////////////////////////////
//
//
//
//

//TODO: Convert to key-map
pub fn toDoomKey(key: u32) -> u8 {
    println!("toDoomKey");

    return 0;
}

pub fn queueKeyPress(pressed: i32, keyCode: u32) {
    println!("queueKeyPress");
}

pub fn SDL_PollEvents() {
    println!("SDL_PollEvents");
}

pub fn GetKey(pressed: *mut i32, doomKey: *mut u8) -> i32 {
    println!("GetKey");

    return 0;
}

pub fn GetTypedChar(key: u8) -> u8 {
    println!("GetTypedChar");

    return 0;
}

pub fn UpdateShiftStatus(pressed: i32, key: u8) {
    println!("UpdateShiftStatus");
}

pub fn I_GetEvent() {
    println!("I_GetEvent");
}

pub fn I_InitInput() {
    println!("I_InitInput");
}
