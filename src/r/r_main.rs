#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// r_main.h
/////////////////////////////
//
// DESCRIPTION:
//	System specific interface stuff.
//

//
// Lighting LUT.
// Used for z-depth cuing per column/row,
//  and other lighting effects (sector ambient, flash).
//

// Lighting constants.
// Now why not 32 levels here?
pub const LIGHTLEVELS: usize = 16;
pub const LIGHTSEGSHIFT: usize = 4;

pub const MAXLIGHTSCALE: usize = 48;
pub const LIGHTSCALESHIFT: usize = 12;
pub const MAXLIGHTZ: usize = 128;
pub const LIGHTZSHIFT: usize = 20;

/////////////////////////////
// r_main.c
/////////////////////////////
//
// DESCRIPTION:
//	Rendering main loop and setup functions,
//	 utility functions (BSP, geometry, trigonometry).
//	See tables.c, too.
//

// Fineangles in the SCREENWIDTH wide window.
pub const FIELDOFVIEW: i32 = 2048;
pub const DISTMAP: u8 = 2;

//
// R_AddPointToBox
// Expand a given bbox
// so that it encloses a given point.
//
pub fn R_AddPointToBox(x: i32, y: i32, Box: *mut fixed_t) {
    println!("R_AddPointToBox");
}

//
// R_PointOnSide
// Traverse BSP (sub) tree,
//  check point against partition plane.
// Returns side 0 (front) or 1 (back).
//
pub fn R_PointOnSide(x: fixed_t, y: fixed_t, node: *mut node_t) -> i32 {
    println!("R_PointOnSide");

    return 1;
}

pub fn R_PointOnSegSide(x: fixed_t, y: fixed_t, line: *mut seg_t) -> i32 {
    println!("R_PointOnSegSide");

    return 1;
}

//
// R_PointToAngle
// To get a global angle from cartesian coordinates,
//  the coordinates are flipped until they are in
//  the first octant of the coordinate system, then
//  the y (<=x) is scaled and divided by x to get a
//  tangent (slope) value which is looked up in the
//  tantoangle[] table.

//

pub fn R_PointToAngle(x: fixed_t, y: fixed_t) -> angle_t {
    println!("R_PointToAngle");

    return 0;
}

pub fn R_PointToAngle2(x1: fixed_t, y1: fixed_t, x2: fixed_t, y2: fixed_t) -> angle_t {
    println!("R_PointToAngle2");

    return 1;
}

pub fn R_PointToDist(x: fixed_t, y: fixed_t) -> fixed_t {
    println!("R_PointToDist");

    return 1;
}

//
// R_InitPointToAngle
//
pub fn R_InitPointToAngle() {
    println!("R_InitPointToAngle");
}

//
// R_ScaleFromGlobalAngle
// Returns the texture mapping scale
//  for the current line (horizontal span)
//  at the given angle.
// rw_distance must be calculated first.
//
pub fn R_ScaleFromGlobalAngle(visangle: angle_t) -> fixed_t {
    println!("R_ScaleFromGlobalAngle");

    return 1;
}

//
// R_InitTables
//
pub fn R_InitTables() {
    println!("R_InitTables");
}

//
// R_InitTextureMapping
//
pub fn R_InitTextureMapping() {
    println!("R_InitTextureMapping");
}

//
// R_InitLightTables
// Only inits the zlight table,
//  because the scalelight table changes with view size.
//

pub fn R_InitLightTables() {
    println!("R_InitLightTables");
}

//
// R_SetViewSize
// Do not really change anything here,
//  because it might be in the middle of a refresh.
// The change will take effect next refresh.
//

pub fn R_SetViewSize(blocks: i32, detail: i32) {
    println!("R_SetViewSize");
}

//
// R_ExecuteSetViewSize
//
pub fn R_ExecuteSetViewSize() {
    println!("R_ExecuteSetViewSize");
}

//
// R_Init
//

pub fn R_Init(doom: &mut modules) {
    println!("R_Init");

    R_InitData();
    print!(".");
    R_InitPointToAngle();
    print!(".");
    R_InitTables();
    // viewwidth / viewheight / detailLevel are set by the defaults
    print!(".");

    R_SetViewSize(doom.m.screenblocks, doom.m.detailLevel);
    R_InitPlanes();
    print!(".");
    R_InitLightTables();
    print!(".");
    R_InitSkyMap();
    R_InitTranslationTables();
    print!(".");

    doom.r.framecount = 0;
}

//
// R_PointInSubsector
//
pub fn R_PointInSubsector(x: fixed_t, y: fixed_t) -> *mut subsector_t {
    println!("R_PointInSubsector");

    return ptr::null_mut();
}
//
// R_SetupFrame
//
pub fn R_SetupFrame(player: *mut player_t) {
    println!("R_SetupFrame");
}
//
// R_RenderView
//
pub fn R_RenderPlayerView(player: *mut player_t) {
    println!("R_RenderPlayerView");
}
