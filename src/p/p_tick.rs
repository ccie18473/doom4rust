#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_tick.h
/////////////////////////////
//
// DESCRIPTION:
//	?
//

/////////////////////////////
// p_tick.c
/////////////////////////////
//
// DESCRIPTION:
//	Archiving: SaveGame I/O.
//	Thinker, Ticker.
//

//
// THINKERS
// All thinkers should be allocated by Z_Malloc
// so they can be operated on uniformly.
// The actual structures will vary in size,
// but the first element must be thinker_t.
//

//
// P_InitThinkers
//
pub fn P_InitThinkers() {
    println!("P_InitThinkers");
}

//
// P_AddThinker
// Adds a new thinker at the end of the list.
//
pub fn P_AddThinker(thinker: *mut thinker_t) {
    println!("P_AddThinker");
}

//
// P_RemoveThinker
// Deallocation is lazy -- it will not actually be freed
// until its thinking turn comes up.
//
pub fn P_RemoveThinker(thinker: *mut thinker_t) {
    println!("P_RemoveThinker");
}

//
// P_AllocateThinker
// Allocates memory and adds a new thinker at the end of the list.
//
pub fn P_AllocateThinker(thinker: *mut thinker_t) {
    println!("P_AllocateThinker");
}

//
// P_RunThinkers
//
pub fn P_RunThinkers() {
    println!("P_RunThinkers");
}

//
// P_Ticker
//

pub fn P_Ticker() {
    println!("P_Ticker");
}
