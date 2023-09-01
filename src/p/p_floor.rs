#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_floor.c
/////////////////////////////
//
// DESCRIPTION:
//	Floor animation: raising stairs.
//

//
// FLOORS
//

//
// Move a plane (floor or ceiling) and check for crushing
//
pub fn T_MovePlane(
    sector: *mut sector_t,
    speed: fixed_t,
    dest: fixed_t,
    crush: bool,
    floorOrCeiling: i32,
    direction: i32,
) -> result_e {
    println!("T_MovePlane");

    return result_e::ok;
}

//
// MOVE A FLOOR TO IT'S DESTINATION (UP OR DOWN)
//
pub fn T_MoveFloor(floor: *mut floormove_t) {
    println!("T_MoveFloor");
}

//
// HANDLE FLOOR TYPES
//
pub fn EV_DoFloor(line: *mut line_t, floortype: floor_e) -> i32 {
    println!("EV_DoFloor");

    return 0;
}

//
// BUILD A STAIRCASE!
//
pub fn EV_BuildStairs(line: *mut line_t, Type: stair_e) -> i32 {
    println!("EV_BuildStairs");

    return 0;
}
