#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_lights.c
/////////////////////////////
//
// DESCRIPTION:
//	Handle Sector base lighting effects.
//	Muzzle flash?
//

//
// FIRELIGHT FLICKER
//

//
// T_FireFlicker
//
pub fn T_FireFlicker(flick: *mut fireflicker_t) {
    println!("T_FireFlicker");
}

//
// P_SpawnFireFlicker
//
pub fn P_SpawnFireFlicker(sector: *mut sector_t) {
    println!("P_SpawnFireFlicker");
}

//
// BROKEN LIGHT FLASHING
//

//
// T_LightFlash
// Do flashing lights.
//
pub fn T_LightFlash(flash: *mut lightflash_t) {
    println!("T_LightFlash");
}

//
// P_SpawnLightFlash
// After the map has been loaded, scan each sector
// for specials that spawn thinkers
//
pub fn P_SpawnLightFlash(sector: *mut sector_t) {
    println!("P_SpawnLightFlash");
}

//
// STROBE LIGHT FLASHING
//

//
// T_StrobeFlash
//
pub fn T_StrobeFlash(flash: *mut strobe_t) {
    println!("T_StrobeFlash");
}

//
// P_SpawnStrobeFlash
// After the map has been loaded, scan each sector
// for specials that spawn thinkers
//
pub fn P_SpawnStrobeFlash(sector: *mut sector_t, fastOrSlow: i32, inSync: i32) {
    println!("P_SpawnStrobeFlash");
}

//
// Start strobing lights (usually from a trigger)
//
pub fn EV_StartLightStrobing(line: *mut line_t) {
    println!("EV_StartLightStrobing");
}

//
// TURN LINE'S TAG LIGHTS OFF
//
pub fn EV_TurnTagLightsOff(line: *mut line_t) {
    println!("EV_TurnTagLightsOff");
}

//
// TURN LINE'S TAG LIGHTS ON
//
pub fn EV_LightTurnOn(line: *mut line_t, bright: i32) {
    println!("EV_LightTurnOn");
}

//
// Spawn glowing light
//

pub fn T_Glow(g: *mut glow_t) {
    println!("T_Glow");
}

pub fn P_SpawnGlowingLight(sector: *mut sector_t) {
    println!("P_SpawnGlowingLight");
}
