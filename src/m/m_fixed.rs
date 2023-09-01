#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/////////////////////////////
// m_fixed.h
/////////////////////////////
//
// DESCRIPTION:
//	Fixed point arithemtics, implementation.
//

//
// Fixed point, 32bit as 16.16.
//
pub const FRACBITS: i32 = 16;
pub const FRACUNIT: i32 = 1 << FRACBITS;

pub type fixed_t = i32;

/////////////////////////////
// m_fixed.c
/////////////////////////////
//
// DESCRIPTION:
//	Fixed point implementation.
//

// Fixme. __USE_C_FIXED__ or something.

pub fn FixedMul(a: fixed_t, b: fixed_t) -> fixed_t {
    //println!("FixedMul");

    return 0;
}

//
// FixedDiv, C version.
//

pub fn FixedDiv(a: fixed_t, b: fixed_t) -> fixed_t {
    //println!("FixedDiv");

    return 0;
}
