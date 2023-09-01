#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// wi_stuff.h
/////////////////////////////
//
// DESCRIPTION:
//  Intermission.
//

// States for the intermission

pub enum stateenum_t {
    NoState = -1,
    StatCount,
    ShowNextLoc,
}

/////////////////////////////
// wi_stuff.c
/////////////////////////////
//
// DESCRIPTION:
//	Intermission screens.
//

//
// Data needed to add patches to full screen intermission pics.
// Patches are statistics messages, and animations.
// Loads of by-pixel layout and placement, offsets etc.
//

//
// Different vetween registered DOOM (1994) and
//  Ultimate DOOM - Final edition (retail, 1995?).
// This is supposedly ignored for commercial
//  release (aka DOOM II), which had 34 maps
//  in one episode. So there.
pub const NUMEPISODES: usize = 4;
pub const NUMMAPS: usize = 9;

// in tics
//U pub const PAUSELEN		(TICRATE*2)
//U pub const SCORESTEP		100
//U pub const ANIMPERIOD		32
// pixel distance from "(YOU)" to "PLAYER N"
//U pub const STARDIST		10
//U pub const WK 1

// GLOBAL LOCATIONS
pub const WI_TITLEY: u8 = 2;
pub const WI_SPACINGY: u8 = 33;

// SINGPLE-PLAYER STUFF
pub const SP_STATSX: u8 = 50;
pub const SP_STATSY: u8 = 50;

pub const SP_TIMEX: u8 = 16;
pub const SP_TIMEY: i32 = SCREENHEIGHT - 32;

// NET GAME STUFF
pub const NG_STATSY: i32 = 50;
//pub const NG_STATSX		:i32=(32 + SHORT(star->width)/2 + 32*!dofrags);

pub const NG_SPACINGX: i32 = 64;

// DEATHMATCH STUFF
pub const DM_MATRIXX: i32 = 42;
pub const DM_MATRIXY: i32 = 68;

pub const DM_SPACINGX: i32 = 40;

pub const DM_TOTALSX: i32 = 269;

pub const DM_KILLERSX: i32 = 10;
pub const DM_KILLERSY: i32 = 100;
pub const DM_VICTIMSX: i32 = 5;
pub const DM_VICTIMSY: i32 = 50;

pub enum animenum_t {
    ANIM_ALWAYS,
    ANIM_RANDOM,
    ANIM_LEVEL,
}

pub struct point_t {
    pub x: i32,
    pub y: i32,
}

//
// Animation.
// There is another anim_t used in p_spec.
//
pub struct anim_t {
    pub Type: animenum_t,

    // period in tics between animations
    pub period: i32,

    // number of animation frames
    pub nanims: i32,

    // location of animation
    pub loc: point_t,

    // ALWAYS: n/a,
    // RANDOM: period deviation (<256),
    // LEVEL: level
    pub data1: i32,

    // ALWAYS: n/a,
    // RANDOM: random base period,
    // LEVEL: n/a
    pub data2: i32,

    // actual graphics for frames of animations
    pub p: [*mut patch_t; 3],

    // following must be initialized to zero before use!

    // next value of bcnt (used in conjunction with period)
    pub nexttic: i32,

    // last drawn animation frame
    pub lastdrawn: i32,

    // next frame number to animate
    pub ctr: i32,

    // used by RANDOM and LEVEL when animating
    pub state: i32,
}

pub const lnodes: [[point_t; NUMMAPS]; NUMEPISODES - 1] = [
    // Episode 0 World Map
    [
        point_t { x: 185, y: 164 }, // location of level 0 (CJ)
        point_t { x: 148, y: 143 }, // location of level 1 (CJ)
        point_t { x: 69, y: 122 },  // location of level 2 (CJ)
        point_t { x: 209, y: 102 }, // location of level 3 (CJ)
        point_t { x: 116, y: 89 },  // location of level 4 (CJ)
        point_t { x: 166, y: 55 },  // location of level 5 (CJ)
        point_t { x: 71, y: 56 },   // location of level 6 (CJ)
        point_t { x: 135, y: 29 },  // location of level 7 (CJ)
        point_t { x: 71, y: 24 },   // location of level 8 (CJ)
    ],
    // Episode 1 World Map should go here
    [
        point_t { x: 254, y: 25 },  // location of level 0 (CJ)
        point_t { x: 97, y: 50 },   // location of level 1 (CJ)
        point_t { x: 188, y: 64 },  // location of level 2 (CJ)
        point_t { x: 128, y: 78 },  // location of level 3 (CJ)
        point_t { x: 214, y: 92 },  // location of level 4 (CJ)
        point_t { x: 133, y: 130 }, // location of level 5 (CJ)
        point_t { x: 208, y: 136 }, // location of level 6 (CJ)
        point_t { x: 148, y: 140 }, // location of level 7 (CJ)
        point_t { x: 235, y: 158 }, // location of level 8 (CJ)
    ],
    // Episode 2 World Map should go here
    [
        point_t { x: 156, y: 168 }, // location of level 0 (CJ)
        point_t { x: 48, y: 154 },  // location of level 1 (CJ)
        point_t { x: 174, y: 95 },  // location of level 2 (CJ)
        point_t { x: 265, y: 75 },  // location of level 3 (CJ)
        point_t { x: 130, y: 48 },  // location of level 4 (CJ)
        point_t { x: 279, y: 23 },  // location of level 5 (CJ)
        point_t { x: 198, y: 48 },  // location of level 6 (CJ)
        point_t { x: 140, y: 25 },  // location of level 7 (CJ)
        point_t { x: 281, y: 136 }, // location of level 8 (CJ)
    ],
];

//
// Animation locations for episode 0 (1).
// Using patches saves a lot of space,
//  as they replace 320x200 full screen frames.
//

pub fn ANIM(Type: animenum_t, period: i32, nanims: i32, x: i32, y: i32, nexttic: i32) -> anim_t {
    let value: anim_t = anim_t {
        Type,
        period,
        nanims,
        loc: point_t { x, y },
        data1: 0,
        data2: 0,
        p: [ptr::null_mut(); 3],
        nexttic,
        lastdrawn: 0,
        ctr: 0,
        state: 0,
    };
    return value;
}
//{ (type), (period), (nanims), { (x), (y) }, (nexttic),    \
//  0, { NULL, NULL, NULL }, 0, 0, 0, 0 }

/*
pub const epsd0animinfo: [anim_t; 10] = [
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 224, 104, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 184, 160, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 112, 136, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 72, 112, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 88, 96, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 64, 48, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 192, 40, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 136, 16, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 80, 16, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 64, 24, 0),
];

pub const epsd1animinfo: [anim_t; 9] = [
    ANIM(animenum_t::ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 1),
    ANIM(animenum_t::ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 2),
    ANIM(animenum_t::ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 3),
    ANIM(animenum_t::ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 4),
    ANIM(animenum_t::ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 5),
    ANIM(animenum_t::ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 6),
    ANIM(animenum_t::ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 7),
    ANIM(animenum_t::ANIM_LEVEL, TICRATE / 3, 3, 192, 144, 8),
    ANIM(animenum_t::ANIM_LEVEL, TICRATE / 3, 1, 128, 136, 8),
];

pub const epsd2animinfo: [anim_t; 6] = [
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 104, 168, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 40, 136, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 160, 96, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 104, 80, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 3, 3, 120, 32, 0),
    ANIM(animenum_t::ANIM_ALWAYS, TICRATE / 4, 3, 40, 0, 0),
];

pub const NUMANIMS: [i32; NUMEPISODES - 1] = [
    epsd0animinfo.len() as i32,
    epsd1animinfo.len() as i32,
    epsd2animinfo.len() as i32,
];
*/
//pub const anims: [*mut anim_t; NUMEPISODES] = [epsd0animinfo, epsd1animinfo, epsd2animinfo];
pub const anims: [*mut anim_t; NUMEPISODES - 1] =
    [ptr::null_mut(), ptr::null_mut(), ptr::null_mut()];

//
// GENERAL DATA
//

//
// Locally used stuff.
//

// States for single-player
pub const SP_KILLS: u8 = 0;
pub const SP_ITEMS: u8 = 2;
pub const SP_SECRET: u8 = 4;
pub const SP_FRAGS: u8 = 6;
pub const SP_TIME: u8 = 8;
//pub const SP_PAR			:u8=ST_TIME; //BUG

pub const SP_PAUSE: u8 = 1;

// in seconds
pub const SHOWNEXTLOCDELAY: u8 = 4;
//pub const SHOWLASTLOCDELAY	SHOWNEXTLOCDELAY

// slam background
pub fn WI_slamBackground() {
    println!("WI_slamBackground");
}

// The ticker is used to detect keys
//  because of timing issues in netgames.
pub fn WI_Responder(ev: *mut event_t) -> bool {
    println!("WI_Responder");

    return false;
}

// Draws "<Levelname> Finished!"
pub fn WI_drawLF() {
    println!("WI_drawLF");
}

// Draws "Entering <LevelName>"
pub fn WI_drawEL() {
    println!("WI_drawEL");
}

pub fn WI_drawOnLnode(n: i32, c: [*mut patch_t; 10]) {
    println!("WI_drawOnLnode");
}

pub fn WI_initAnimatedBack() {
    println!("WI_initAnimatedBack");
}

pub fn WI_updateAnimatedBack() {
    println!("WI_updateAnimatedBack");
}

pub fn WI_drawAnimatedBack() {
    println!("WI_drawAnimatedBack");
}

//
// Draws a number.
// If digits > 0, then use that many digits minimum,
//  otherwise only use as many as necessary.
// Returns new x position.
//

pub fn WI_drawNum(x: i32, y: i32, n: i32, digits: i32) -> i32 {
    println!("WI_drawNum");

    return 0;
}

pub fn WI_drawPercent(x: i32, y: i32, p: i32) {
    println!("WI_drawPercent");
}

//
// Display level completion time and par,
//  or "sucks" message if overflow.
//
pub fn WI_drawTime(x: i32, y: i32, t: i32) {
    println!("WI_drawTime");
}

pub fn WI_End() {
    println!("WI_End");
}

pub fn WI_initNoState() {
    println!("WI_initNoState");
}

pub fn WI_updateNoState() {
    println!("WI_updateNoState");
}

pub fn WI_initShowNextLoc() {
    println!("WI_initShowNextLoc");
}

pub fn WI_updateShowNextLoc() {
    println!("WI_updateShowNextLoc");
}

pub fn WI_drawShowNextLoc() {
    println!("WI_drawShowNextLoc");
}

pub fn WI_drawNoState() {
    println!("WI_drawNoState");
}

pub fn WI_fragSum(playernum: i32) -> i32 {
    println!("WI_fragSum");

    return 0;
}

pub fn WI_initDeathmatchStats() {
    println!("WI_initDeathmatchStats");
}

pub fn WI_updateDeathmatchStats() {
    println!("WI_updateDeathmatchStats");
}

pub fn WI_drawDeathmatchStats() {
    println!("WI_drawDeathmatchStats");
}

pub fn WI_initNetgameStats() {
    println!("WI_initNetgameStats");
}

pub fn WI_updateNetgameStats() {
    println!("WI_updateNetgameStats");
}

pub fn WI_drawNetgameStats() {
    println!("WI_drawNetgameStats");
}

pub fn WI_initStats() {
    println!("WI_initStats");
}

pub fn WI_updateStats() {
    println!("WI_updateStats");
}

pub fn WI_drawStats() {
    println!("WI_drawStats");
}

pub fn WI_checkForAccelerate() {
    println!("WI_checkForAccelerate");
}

// Updates stuff each tick
pub fn WI_Ticker() {
    println!("WI_Ticker");
}

pub type load_callback_t = fn(lumpname: String, variable: *mut *mut patch_t);

// Common load/unload function.  Iterates over all the graphics
// lumps to be loaded/unloaded into memory.

pub fn WI_loadUnloadData(callback: load_callback_t) {
    println!("WI_loadUnloadData");
}

pub fn WI_loadCallback(name: String, variable: *mut *mut patch_t) {
    println!("WI_loadCallback");
}

pub fn WI_loadData() {
    println!("WI_loadData");
}

pub fn WI_unloadCallback(name: String, variable: *mut *mut patch_t) {
    println!("WI_unloadCallback");
}

pub fn WI_unloadData() {
    println!("WI_unloadData");
}

pub fn WI_Drawer() {
    println!("WI_Drawer");
}

pub fn WI_initVariables(wbstartstruct: *mut wbstartstruct_t) {
    println!("WI_initVariables");
}
