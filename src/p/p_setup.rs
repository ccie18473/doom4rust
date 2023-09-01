#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_setup.c
/////////////////////////////
//
// DESCRIPTION:
//	Do all the WAD I/O, get map description,
//	set up initial state and misc. LUTs.
//

//
// P_LoadVertexes
//
pub fn P_LoadVertexes(lump: i32) {
    println!("P_LoadVertexes");
}

//
// GetSectorAtNullAddress
//
pub fn GetSectorAtNullAddress() -> *mut sector_t {
    println!("GetSectorAtNullAddress");

    return ptr::null_mut();
}

//
// P_LoadSegs
//
pub fn P_LoadSegs(lump: i32) {
    println!("P_LoadSegs");
}

//
// P_LoadSubsectors
//
pub fn P_LoadSubsectors(lump: i32) {
    println!("P_LoadSubsectors");
}

//
// P_LoadSectors
//
pub fn P_LoadSectors(lump: i32) {
    println!("P_LoadSectors");
}

//
// P_LoadNodes
//
pub fn P_LoadNodes(lump: i32) {
    println!("P_LoadNodes");
}

//
// P_LoadThings
//
pub fn P_LoadThings(lump: i32) {
    println!("P_LoadThings");
}

//
// P_LoadLineDefs
// Also counts secret lines for intermissions.
//
pub fn P_LoadLineDefs(lump: i32) {
    println!("P_LoadLineDefs");
}

//
// P_LoadSideDefs
//
pub fn P_LoadSideDefs(lump: i32) {
    println!("P_LoadSideDefs");
}

//
// P_LoadBlockMap
//
pub fn P_LoadBlockMap(lump: i32) {
    println!("P_LoadBlockMap");
}

//
// P_GroupLines
// Builds sector line lists and subsector sector numbers.
// Finds block bounding boxes for sectors.
//
pub fn P_GroupLines() {
    println!("P_GroupLines");
}

// Pad the REJECT lump with extra data when the lump is too small,
// to simulate a REJECT buffer overflow in Vanilla Doom.

pub fn PadRejectArray(array: *mut u8, len: u32) {
    println!("PadRejectArray");
}

pub fn P_LoadReject(lumpnum: i32) {
    println!("P_LoadReject");
}

//
// P_SetupLevel
//
pub fn P_SetupLevel(episode: i32, map: i32, playermask: i32, skill: skill_t) {
    println!("P_SetupLevel");
}

//
// P_Init
//
pub fn P_Init() {
    println!("P_Init");

    P_InitSwitchList();
    P_InitPicAnims();
    R_InitSprites(sprnames.to_vec());
}
