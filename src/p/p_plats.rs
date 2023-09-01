#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_plats.c
/////////////////////////////
//
// DESCRIPTION:
//	Plats (i.e. elevator platforms) code, raising/lowering.
//

//
// Move a plat up and down
//
pub fn T_PlatRaise(plat: *mut plat_t) {
    println!("T_PlatRaise");
}

//
// Do Platforms
//  "amount" is only used for SOME platforms.
//
pub fn EV_DoPlat(line: *mut line_t, Type: plattype_e, amount: i32) -> i32 {
    println!("EV_DoPlat");

    return 0;
}

pub fn P_ActivateInStasis(tag: i32) {
    println!("P_ActivateInStasis");
}

pub fn EV_StopPlat(line: *mut line_t) {
    println!("EV_StopPlat");
}

pub fn P_AddActivePlat(plat: *mut plat_t) {
    println!("P_AddActivePlat");
}

pub fn P_RemoveActivePlat(plat: *mut plat_t) {
    println!("P_RemoveActivePlat");
}
