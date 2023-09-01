#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_user.c
/////////////////////////////
//
// DESCRIPTION:
//	Player related stuff.
//	Bobbing POV/weapon, movement.
//	Pending weapon.
//

//
// P_Thrust
// Moves the given origin along a given angle.
//
pub fn P_Thrust(player: *mut player_t, angle: angle_t, Move: fixed_t) {
    println!("P_Thrust");
}

//
// P_CalcHeight
// Calculate the walking / running height adjustment
//
pub fn P_CalcHeight(player: *mut player_t) {
    println!("P_CalcHeight");
}

//
// P_MovePlayer
//
pub fn P_MovePlayer(player: *mut player_t) {
    println!("P_MovePlayer");
}

//
// P_DeathThink
// Fall on your face when dying.
// Decrease POV height to floor height.
//

pub fn P_DeathThink(player: *mut player_t) {
    println!("P_DeathThink");
}

//
// P_PlayerThink
//
pub fn P_PlayerThink(player: *mut player_t) {
    println!("P_PlayerThink");
}
