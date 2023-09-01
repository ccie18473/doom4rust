#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// r_sky.h
/////////////////////////////
//
// DESCRIPTION:
//	Sky rendering.
//

// SKY, store the number for name.
pub const SKYFLATNAME: &str = "F_SKY1";

// The sky map is 256*128*4 maps.
pub const ANGLETOSKYSHIFT: u8 = 22;

/////////////////////////////
// r_sky.c
/////////////////////////////
//
// DESCRIPTION:
//  Sky rendering. The DOOM sky is a texture map like any
//  wall, wrapping around. A 1024 columns equal 360 degrees.
//  The default sky map is 256 columns and repeats 4 times
//  on a 320 screen?
//
//

//
// R_InitSkyMap
// Called whenever the view size changes.
//
pub fn R_InitSkyMap() {
    println!("R_InitSkyMap");
}
