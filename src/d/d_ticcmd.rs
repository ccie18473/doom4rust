#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// d_ticcmd.h
/////////////////////////////
//
// DESCRIPTION:
//	System specific interface stuff.
//

// The data sampled per tick (single player)
// and transmitted to other peers (multiplayer).
// Mainly movements/button commands per game tick,
// plus a checksum for internal state consistency.

#[derive(Copy, Clone)]
pub struct ticcmd_t {
    pub forwardmove: i8, // *2048 for move
    pub sidemove: i8,    // *2048 for move
    pub angleturn: i16,  // <<16 for angle delta
    pub chatchar: u8,
    pub buttons: u8,
    // villsa [STRIFE] according to the asm,
    // consistancy is a short, not a byte
    pub consistancy: u8, // checks for net game

    // villsa - Strife specific:
    pub buttons2: u8,
    pub inventory: i32,

    // Heretic/Hexen specific:
    pub lookfly: u8, // look/fly up/down/centering
    pub arti: u8,    // artitype_t to use
}

impl ticcmd_t {
    pub fn new() -> Self {
        Self {
            forwardmove: 0,
            sidemove: 0,
            angleturn: 0,
            chatchar: 0,
            buttons: 0,
            consistancy: 0,
            buttons2: 0,
            inventory: 0,
            lookfly: 0,
            arti: 0,
        }
    }
}
