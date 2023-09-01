#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_switch.c
/////////////////////////////
//
//
// DESCRIPTION:
//	Switches, buttons. Two-state animation. Exits.
//

//
// CHANGE THE TEXTURE OF A WALL SWITCH TO ITS OPPOSITE
//
pub const alphSwitchList: [switchlist_t; 41] = [
    // Doom shareware episode 1 switches
    switchlist_t {
        name1: "SW1BRCOM",
        name2: "SW2BRCOM",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1BRN1",
        name2: "SW2BRN1",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1BRN2",
        name2: "SW2BRN2",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1BRNGN",
        name2: "SW2BRNGN",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1BROWN",
        name2: "SW2BROWN",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1COMM",
        name2: "SW2COMM",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1COMP",
        name2: "SW2COMP",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1DIRT",
        name2: "SW2DIRT",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1EXIT",
        name2: "SW2EXIT",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1GRAY",
        name2: "SW2GRAY",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1GRAY1",
        name2: "SW2GRAY1",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1METAL",
        name2: "SW2METAL",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1PIPE",
        name2: "SW2PIPE",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1SLAD",
        name2: "SW2SLAD",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1STARG",
        name2: "SW2STARG",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1STON1",
        name2: "SW2STON1",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1STON2",
        name2: "SW2STON2",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1STONE",
        name2: "SW2STONE",
        episode: 1,
    },
    switchlist_t {
        name1: "SW1STRTN",
        name2: "SW2STRTN",
        episode: 1,
    },
    // Doom registered episodes 2&3 switches
    switchlist_t {
        name1: "SW1BLUE",
        name2: "SW2BLUE",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1CMT",
        name2: "SW2CMT",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1GARG",
        name2: "SW2GARG",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1GSTON",
        name2: "SW2GSTON",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1HOT",
        name2: "SW2HOT",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1LION",
        name2: "SW2LION",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1SATYR",
        name2: "SW2SATYR",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1SKIN",
        name2: "SW2SKIN",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1VINE",
        name2: "SW2VINE",
        episode: 2,
    },
    switchlist_t {
        name1: "SW1WOOD",
        name2: "SW2WOOD",
        episode: 2,
    },
    // Doom II switches
    switchlist_t {
        name1: "SW1PANEL",
        name2: "SW2PANEL",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1ROCK",
        name2: "SW2ROCK",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1MET2",
        name2: "SW2MET2",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1WDMET",
        name2: "SW2WDMET",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1BRIK",
        name2: "SW2BRIK",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1MOD1",
        name2: "SW2MOD1",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1ZIM",
        name2: "SW2ZIM",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1STON6",
        name2: "SW2STON6",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1TEK",
        name2: "SW2TEK",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1MARB",
        name2: "SW2MARB",
        episode: 3,
    },
    switchlist_t {
        name1: "SW1SKULL",
        name2: "SW2SKULL",
        episode: 3,
    },
    switchlist_t {
        name1: "\0",
        name2: "\0",
        episode: 0,
    },
];

//
// P_InitSwitchList
// Only called at game initialization.
//
pub fn P_InitSwitchList() {
    println!("P_InitSwitchList");
}

//
// Start a button counting down till it turns off.
//
pub fn P_StartButton(line: *mut line_t, w: bwhere_e, texture: i32, time: i32) {
    println!("P_StartButton");
}

//
// Function that changes wall texture.
// Tell it if switch is ok to use again (1=yes, it's a button).
//
pub fn P_ChangeSwitchTexture(line: *mut line_t, useAgain: i32) {
    println!("P_ChangeSwitchTexture");
}

//
// P_UseSpecialLine
// Called when a thing uses a special line.
// Only the front sides of lines are usable.
//
pub fn P_UseSpecialLine(thing: *mut mobj_t, line: *mut line_t, side: i32) -> bool {
    println!("P_UseSpecialLine");

    return false;
}
