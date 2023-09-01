#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// d_textur.h
/////////////////////////////
//
// DESCRIPTION:
//	Typedefs related to to textures etc.,
//	 isolated here to make it easier separating modules.
//

//
// Flats?
//
// a pic is an unmasked block of pixels
pub struct pic_t {
    pub width: u8,
    pub height: u8,
    pub data: u8,
}
