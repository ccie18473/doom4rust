#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// i_endoom.h
/////////////////////////////
//
// DESCRIPTION:
//    Exit text-mode ENDOOM screen.
//

pub const ENDOOM_W: u8 = 80;
pub const ENDOOM_H: u8 = 25;

/////////////////////////////
// i_endoom.c
/////////////////////////////
//
// DESCRIPTION:
//    Exit text-mode ENDOOM screen.
//

//
// Displays the text mode ending screen after the game quits
//

pub fn I_Endoom(endoom_data: *mut u8) {
    println!("I_Endoom");
}
