#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// i_swap.h
/////////////////////////////
//
// DESCRIPTION:
//	Endianess handling, swapping 16bit and 32bit.
//

// Just use SDL's endianness swapping functions.

// These are deliberately cast to signed values; this is the behaviour
// of the macros in the original source and some code relies on it.

pub fn SHORT(x: i16) -> i16 {
    //((signed short) SDL_SwapLE16(x))
    return x;
}
pub fn LONG(x: i32) -> i32 {
    //((signed int) SDL_SwapLE32(x))
    return x;
}

// Defines for checking the endianness of the system.

//#if SDL_BYTEORDER == SYS_LIL_ENDIAN
//#define SYS_LITTLE_ENDIAN
//#elif SDL_BYTEORDER == SYS_BIG_ENDIAN
//#define SYS_BIG_ENDIAN
//#endif

// cosmito from lsdldoom
pub fn doom_swap_s(x: i16) -> u16 {
    return (((x as u16) & 0x00ff) << 8) | (((x as u16) & 0xff00) >> 8) as u16;
}

pub fn doom_wtohs(x: i16) -> i16 {
    //(short int)(x)
    return x;
}
