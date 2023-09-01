#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/////////////////////////////
// w_merge.h
/////////////////////////////
//
// DESCRIPTION:
// Handles merging of PWADs, similar to deutex's -merge option
//
// Ideally this should work exactly the same as in deutex, but trying to
// read the deutex source code made my brain hurt.
//

pub const W_NWT_MERGE_SPRITES: u8 = 0x1;
pub const W_NWT_MERGE_FLATS: u8 = 0x2;
