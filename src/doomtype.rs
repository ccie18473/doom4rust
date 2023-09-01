#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/////////////////////////////
// doomtype.h
/////////////////////////////
//
// DESCRIPTION:
//	Simple basic typedefs, isolated here to make it easier
//	 separating modules.
//

// Use builtin bool type with C++.

pub type boolean = bool;

pub type byte = u8;

pub const DIR_SEPARATOR: char = '/';
pub const DIR_SEPARATOR_S: &str = "/";
pub const PATH_SEPARATOR: char = ':';
