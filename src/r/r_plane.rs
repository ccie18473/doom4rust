#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// r_plane.h
/////////////////////////////
//
// DESCRIPTION:
//	Refresh, visplane stuff (floor, ceilings).
//

/////////////////////////////
// r_plane.c
/////////////////////////////
//
// DESCRIPTION:
//	Here is a core component: drawing the floors and ceilings,
//	 while maintaining a per column clipping list only.
//	Moreover, the sky areas have to be determined.
//

//
// R_InitPlanes
// Only at game startup.
//
pub fn R_InitPlanes() {
    // Doh!
}

//
// R_MapPlane
//
// Uses global vars:
//  planeheight
//  ds_source
//  basexscale
//  baseyscale
//  viewx
//  viewy
//
// BASIC PRIMITIVE
//
pub fn R_MapPlane(y: i32, x1: i32, x2: i32) {
    println!("R_MapPlane");
}

//
// R_ClearPlanes
// At begining of frame.
//
pub fn R_ClearPlanes() {
    println!("R_ClearPlanes");
}

//
// R_FindPlane
//
pub fn R_FindPlane(height: fixed_t, picnum: i32, lightlevel: i32) -> *mut visplane_t {
    println!("R_FindPlane");

    return ptr::null_mut();
}

//
// R_CheckPlane
//
pub fn R_CheckPlane(pl: *mut visplane_t, start: i32, stop: i32) -> *mut visplane_t {
    println!("R_CheckPlane");

    return ptr::null_mut();
}

//
// R_MakeSpans
//
pub fn R_MakeSpans(x: i32, t1: i32, b1: i32, t2: i32, b2: i32) {
    println!("R_MakeSpans");
}

//
// R_DrawPlanes
// At the end of each frame.
//
pub fn R_DrawPlanes() {
    println!("R_DrawPlanes");
}
