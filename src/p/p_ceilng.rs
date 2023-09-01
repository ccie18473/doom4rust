#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_ceilng.c
/////////////////////////////
//
// DESCRIPTION:  Ceiling aninmation (lowering, crushing, raising)
//

//
// T_MoveCeiling
//

pub fn T_MoveCeiling(ceiling: *mut ceiling_t) {
    println!("T_MoveCeiling");
}

//
// EV_DoCeiling
// Move a ceiling up/down and all around!
//
pub fn EV_DoCeiling(line: *mut line_t, Type: ceiling_e) -> i32 {
    println!("EV_DoCeiling");

    return 0;
}

//
// Add an active ceiling
//
pub fn P_AddActiveCeiling(c: *mut ceiling_t) {
    println!("P_AddActiveCeiling");
}

//
// Remove a ceiling's thinker
//
pub fn P_RemoveActiveCeiling(c: *mut ceiling_t) {
    println!("P_RemoveActiveCeiling");
}

//
// Restart a ceiling that's in-stasis
//
pub fn P_ActivateInStasisCeiling(line: *mut line_t) {
    println!("P_ActivateInStasisCeiling");
}

//
// EV_CeilingCrushStop
// Stop a ceiling from crushing!
//
pub fn EV_CeilingCrushStop(line: *mut line_t) -> i32 {
    println!("EV_CeilingCrushStop");

    return 0;
}
