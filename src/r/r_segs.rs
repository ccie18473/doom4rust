#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// r_segs.h
/////////////////////////////
//
// DESCRIPTION:
//	Refresh module, drawing LineSegs from BSP.
//

/////////////////////////////
// r_segs.c
/////////////////////////////
//
// DESCRIPTION:
//	All the clipping: columns, horizontal spans, sky columns.
//

//
// R_RenderMaskedSegRange
//
pub fn R_RenderMaskedSegRange(ds: *mut drawseg_t, x1: i32, x2: i32) {
    println!("R_RenderMaskedSegRange");
}

//
// R_RenderSegLoop
// Draws zero, one, or two textures (and possibly a masked
//  texture) for walls.
// Can draw or mark the starting pixel of floor and ceiling
//  textures.
// CALLED: CORE LOOPING ROUTINE.
//

pub fn R_RenderSegLoop() {
    println!("R_RenderSegLoop");
}

//
// R_StoreWallRange
// A wall segment will be drawn
//  between start and stop pixels (inclusive).
//
pub fn R_StoreWallRange(start: i32, stop: i32) {
    println!("R_StoreWallRange");
}
