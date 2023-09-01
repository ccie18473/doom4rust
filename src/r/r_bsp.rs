#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// r_bsp.h
/////////////////////////////
//
// DESCRIPTION:
//	Refresh module, BSP traversal and handling.
//

/////////////////////////////
// r_bsp.c
/////////////////////////////
//
// DESCRIPTION:
//	BSP traversal, handling of LineSegs for rendering.
//

//
// R_ClearDrawSegs
//
pub fn R_ClearDrawSegs() {
    println!("R_ClearDrawSegs");
}

//
// R_ClipSolidWallSegment
// Does handle solid walls,
//  e.g. single sided LineDefs (middle texture)
//  that entirely block the view.
//
pub fn R_ClipSolidWallSegment(first: i32, last: i32) {
    println!("R_ClipSolidWallSegment");
}

//
// R_ClipPassWallSegment
// Clips the given range of columns,
//  but does not includes it in the clip list.
// Does handle windows,
//  e.g. LineDefs with upper and lower texture.
//
pub fn R_ClipPassWallSegment(first: i32, last: i32) {
    println!("R_ClipPassWallSegment");
}

//
// R_ClearClipSegs
//
pub fn R_ClearClipSegs() {
    println!("R_ClearClipSegs");
}

//
// R_AddLine
// Clips the given segment
// and adds any visible pieces to the line list.
//
pub fn R_AddLine(line: *mut seg_t) {
    println!("R_AddLine");
}

//
// R_CheckBBox
// Checks BSP node/subtree bounding box.
// Returns true
//  if some part of the bbox might be visible.
//

pub fn R_CheckBBox(bspcoord: *mut fixed_t) -> bool {
    println!("R_CheckBBox");

    return false;
}

//
// R_Subsector
// Determine floor/ceiling planes.
// Add sprites of things in sector.
// Draw one or more line segments.
//
pub fn R_Subsector(num: i32) {
    println!("R_Subsector");
}

//
// RenderBSPNode
// Renders all subsectors below a given node,
//  traversing subtree recursively.
// Just call with BSP root.
pub fn R_RenderBSPNode(bspnum: i32) {
    println!("R_RenderBSPNode");
}
