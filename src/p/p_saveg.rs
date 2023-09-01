#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_saveg.h
/////////////////////////////
//
// DESCRIPTION:
//	Savegame I/O, archiving, persistence.
//

/////////////////////////////
// p_saveg.c
/////////////////////////////
//
// DESCRIPTION:
//	Archiving: SaveGame I/O.
//

// Get the filename of a temporary file to write the savegame to.  After
// the file has been successfully saved, it will be renamed to the
// real file.

pub fn P_TempSaveGameFile() -> &'static str {
    println!("P_TempSaveGameFile");

    return "";
}

// Get the filename of the save game file to use for the specified slot.

pub fn P_SaveGameFile(slot: i32) -> &'static str {
    println!("P_SaveGameFile");

    return "";
}

// Endian-safe integer read/write functions

pub fn saveg_read8() -> u8 {
    println!("saveg_read8");

    return 0;
}

pub fn saveg_write8(value: u8) {
    println!("saveg_write8");
}

pub fn saveg_read16() -> i16 {
    println!("saveg_read16");

    return 0;
}

pub fn saveg_write16(value: i16) {
    println!("saveg_write16");
}

pub fn saveg_read32() -> i32 {
    println!("saveg_read32");

    return 0;
}

pub fn saveg_write32(value: i32) {
    println!("saveg_write32");
}

// Pad to 4-byte boundaries

pub fn saveg_read_pad() {
    println!("saveg_read_pad");
}

pub fn saveg_write_pad() {
    println!("saveg_write_pad");
}

// Pointers

pub fn saveg_readp() -> *mut libc::c_void {
    println!("saveg_readp");

    return ptr::null_mut();
}

pub fn saveg_writep(p: *mut libc::c_void) {
    println!("saveg_writep");
}

//
// Structure read/write functions
//

//
// mapthing_t
//

pub fn saveg_read_mapthing_t(str: *mut mapthing_t) {
    println!("saveg_read_mapthing_t");
}

pub fn saveg_write_mapthing_t(str: *mut mapthing_t) {
    println!("saveg_write_mapthing_t");
}

//
// actionf_t
//

pub fn saveg_read_actionf_t(str: *mut actionf_t) {
    println!("saveg_read_actionf_t");
}

pub fn saveg_write_actionf_t(str: *mut actionf_t) {
    println!("saveg_write_actionf_t");
}

//
// thinker_t
//

pub fn saveg_read_thinker_t(str: *mut thinker_t) {
    println!("saveg_read_thinker_t");
}

pub fn saveg_write_thinker_t(str: *mut thinker_t) {
    println!("saveg_write_thinker_t");
}

//
// mobj_t
//

pub fn saveg_read_mobj_t(str: *mut mobj_t) {
    println!("saveg_read_mobj_t");
}

pub fn saveg_write_mobj_t(str: *mut mobj_t) {
    println!("saveg_write_mobj_t");
}

//
// ticcmd_t
//

pub fn saveg_read_ticcmd_t(str: *mut ticcmd_t) {
    println!("saveg_read_ticcmd_t");
}

pub fn saveg_write_ticcmd_t(str: *mut ticcmd_t) {
    println!("saveg_write_ticcmd_t");
}

//
// pspdef_t
//

pub fn saveg_read_pspdef_t(str: *mut pspdef_t) {
    println!("saveg_read_pspdef_t");
}

pub fn saveg_write_pspdef_t(str: *mut pspdef_t) {
    println!("saveg_write_pspdef_t");
}

//
// player_t
//

pub fn saveg_read_player_t(str: *mut player_t) {
    println!("saveg_read_player_t");
}

pub fn saveg_write_player_t(str: *mut player_t) {
    println!("saveg_write_player_t");
}

//
// ceiling_t
//

pub fn saveg_read_ceiling_t(str: *mut ceiling_t) {
    println!("saveg_read_ceiling_t");
}

pub fn saveg_write_ceiling_t(str: *mut ceiling_t) {
    println!("saveg_write_ceiling_t");
}

//
// vldoor_t
//

pub fn saveg_read_vldoor_t(str: *mut vldoor_t) {
    println!("saveg_read_vldoor_t");
}

pub fn saveg_write_vldoor_t(str: *mut vldoor_t) {
    println!("saveg_write_vldoor_t");
}

//
// floormove_t
//

pub fn saveg_read_floormove_t(str: *mut floormove_t) {
    println!("saveg_read_floormove_t");
}

pub fn saveg_write_floormove_t(str: *mut floormove_t) {
    println!("saveg_write_floormove_t");
}

//
// plat_t
//

pub fn saveg_read_plat_t(str: *mut plat_t) {
    println!("saveg_read_plat_t");
}

pub fn saveg_write_plat_t(str: *mut plat_t) {
    println!("saveg_write_plat_t");
}

//
// lightflash_t
//

pub fn saveg_read_lightflash_t(str: *mut lightflash_t) {
    println!("saveg_read_lightflash_t");
}

pub fn saveg_write_lightflash_t(str: *mut lightflash_t) {
    println!("saveg_write_lightflash_t");
}

//
// strobe_t
//

pub fn saveg_read_strobe_t(str: *mut strobe_t) {
    println!("saveg_read_strobe_t");
}

pub fn saveg_write_strobe_t(str: *mut strobe_t) {
    println!("saveg_write_strobe_t");
}

//
// glow_t
//

pub fn saveg_read_glow_t(str: *mut glow_t) {
    println!("saveg_read_glow_t");
}

pub fn saveg_write_glow_t(str: *mut glow_t) {
    println!("saveg_write_glow_t");
}

//
// Write the header for a savegame
//

pub fn P_WriteSaveGameHeader(description: &str) {
    println!("P_WriteSaveGameHeader");
}

//
// Read the header for a savegame
//

pub fn P_ReadSaveGameHeader() -> bool {
    println!("P_ReadSaveGameHeader");

    return false;
}

//
// Read the end of file marker.  Returns true if read successfully.
//

pub fn P_ReadSaveGameEOF() -> bool {
    println!("P_ReadSaveGameEOF");

    return false;
}

//
// Write the end of file marker
//

pub fn P_WriteSaveGameEOF() {
    println!("P_WriteSaveGameEOF");
}

//
// P_ArchivePlayers
//
pub fn P_ArchivePlayers() {
    println!("P_ArchivePlayers");
}

//
// P_UnArchivePlayers
//
pub fn P_UnArchivePlayers() {
    println!("P_UnArchivePlayers");
}

//
// P_ArchiveWorld
//
pub fn P_ArchiveWorld() {
    println!("P_ArchiveWorld");
}

//
// P_UnArchiveWorld
//
pub fn P_UnArchiveWorld() {
    println!("P_UnArchiveWorld");
}

//
// P_ArchiveThinkers
//
pub fn P_ArchiveThinkers() {
    println!("P_ArchiveThinkers");
}

//
// P_UnArchiveThinkers
//
pub fn P_UnArchiveThinkers() {
    println!("P_UnArchiveThinkers");
}

//
// P_ArchiveSpecials
//

//
// Things to handle:
//
// T_MoveCeiling, (ceiling_t: sector_t * swizzle), - active list
// T_VerticalDoor, (vldoor_t: sector_t * swizzle),
// T_MoveFloor, (floormove_t: sector_t * swizzle),
// T_LightFlash, (lightflash_t: sector_t * swizzle),
// T_StrobeFlash, (strobe_t: sector_t *),
// T_Glow, (glow_t: sector_t *),
// T_PlatRaise, (plat_t: sector_t *), - active list
//
pub fn P_ArchiveSpecials() {
    println!("P_ArchiveSpecials");
}

//
// P_UnArchiveSpecials
//
pub fn P_UnArchiveSpecials() {
    println!("P_UnArchiveSpecials");
}
