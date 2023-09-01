#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// r_things.h
/////////////////////////////
//
// DESCRIPTION:
//	Rendering of moving objects, sprites.
//

pub const MAXVISSPRITES: u8 = 128;

/////////////////////////////
// r_things.c
/////////////////////////////
//
// DESCRIPTION:
//	Refresh of things, i.e. objects represented by sprites.
//

pub const MINZ: i32 = FRACUNIT * 4;
pub const BASEYCENTER: u8 = 100;

pub struct maskdraw_t {
    pub x1: i32,
    pub x2: i32,

    pub column: i32,
    pub topclip: i32,
    pub bottomclip: i32,
}

//
// INITIALIZATION FUNCTIONS
//

//
// R_InstallSpriteLump
// Local function for R_InitSprites.
//
pub fn R_InstallSpriteLump(lump: i32, frame: u32, rotation: u32, flipped: bool) {
    println!("R_InstallSpriteLump");
}

//
// R_InitSpriteDefs
// Pass a null terminated list of sprite names
//  (4 chars exactly) to be used.
// Builds the sprite rotation matrixes to account
//  for horizontally flipped sprites.
// Will report an error if the lumps are inconsistant.
// Only called at startup.
//
// Sprite lump names are 4 characters for the actor,
//  a letter for the frame, and a number for the rotation.
// A sprite that is flippable will have an additional
//  letter/number appended.
// The rotation character can be 0 to signify no rotations.
//
pub fn R_InitSpriteDefs(namelist: Vec<&str>) {
    println!("R_InitSpriteDefs");
}

//
// GAME FUNCTIONS
//

//
// R_InitSprites
// Called at program start.
//
pub fn R_InitSprites(namelist: Vec<&str>) {
    println!("R_InitSprites");
}

//
// R_ClearSprites
// Called at frame start.
//
pub fn R_ClearSprites() {
    println!("R_ClearSprites");
}

//
// R_NewVisSprite
//

pub fn R_NewVisSprite() -> *mut vissprite_t {
    println!("R_DrawMaskedColumn");

    return ptr::null_mut();
}

//
// R_DrawMaskedColumn
// Used for sprites and masked mid textures.
// Masked means: partly transparent, i.e. stored
//  in posts/runs of opaque pixels.
//

pub fn R_DrawMaskedColumn(column: *mut column_t) {
    println!("R_DrawMaskedColumn");
}

//
// R_DrawVisSprite
//  mfloorclip and mceilingclip should also be set.
//
pub fn R_DrawVisSprite(vis: *mut vissprite_t, x1: i32, x2: i32) {
    println!("R_DrawVisSprite");
}

//
// R_ProjectSprite
// Generates a vissprite for a thing
//  if it might be visible.
//
pub fn R_ProjectSprite(thing: *mut mobj_t) {
    println!("R_ProjectSprite");
}

//
// R_AddSprites
// During BSP traversal, this adds sprites by sector.
//
pub fn R_AddSprites(sec: *mut sector_t) {
    println!("R_AddSprites");
}

//
// R_DrawPSprite
//
pub fn R_DrawPSprite(psp: *mut pspdef_t) {
    println!("R_DrawPSprite");
}

//
// R_DrawPlayerSprites
//
pub fn R_DrawPlayerSprites() {
    println!("R_DrawPlayerSprites");
}

//
// R_SortVisSprites
//

pub fn R_SortVisSprites() {
    println!("R_SortVisSprites");
}

//
// R_DrawSprite
//

pub fn R_DrawSprite(spr: *mut vissprite_t) {
    println!("R_DrawSprite");
}

//
// R_DrawMasked
//
pub fn R_DrawMasked() {
    println!("R_DrawMasked");
}
