#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_local.h
/////////////////////////////
//
// DESCRIPTION:
//	Play functions, animation, global header.
//

//
// P_MAPUTL
//
#[repr(C)]
pub struct divline_t {
    pub x: fixed_t,
    pub y: fixed_t,
    pub dx: fixed_t,
    pub dy: fixed_t,
}
#[repr(C)]
pub union d {
    pub thing: *mut mobj_t,
    pub line: *mut line_t,
}
#[repr(C)]
pub struct intercept_t {
    pub frac: fixed_t, // along trace line
    pub isaline: bool,
    pub d: d,
}

//typedef boolean (*traverser_t) (intercept_t *in);
pub type traverser_t = fn(In: *mut intercept_t);
