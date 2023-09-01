#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_doors.c
/////////////////////////////
//
// DESCRIPTION: Door animation code (opening/closing)
//

//
// VERTICAL DOORS
//

//
// T_VerticalDoor
//
pub fn T_VerticalDoor(door: *mut vldoor_t) {
    println!("T_VerticalDoor");
}

//
// EV_DoLockedDoor
// Move a locked door up/down
//

pub fn EV_DoLockedDoor(line: *mut line_t, Type: vldoor_e, thing: *mut mobj_t) -> i32 {
    println!("EV_DoLockedDoor");

    return 0;
}

pub fn EV_DoDoor(line: *mut line_t, Type: vldoor_e) -> i32 {
    println!("EV_DoDoor");

    return 0;
}

//
// EV_VerticalDoor : open a door manually, no tag value
//
pub fn EV_VerticalDoor(line: *mut line_t, thing: *mut mobj_t) {
    println!("EV_VerticalDoor");
}

//
// Spawn a door that closes after 30 seconds
//
pub fn P_SpawnDoorCloseIn30(sec: *mut sector_t) {
    println!("P_SpawnDoorCloseIn30");
}

//
// Spawn a door that opens after 5 minutes
//
pub fn P_SpawnDoorRaiseIn5Mins(sec: *mut sector_t, secnum: i32) {
    println!("P_SpawnDoorRaiseIn5Mins");
}
