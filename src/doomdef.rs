#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

/////////////////////////////
// doomdef.h
/////////////////////////////
//
// DESCRIPTION:
//  Internally used data structures for virtually everything,
//   lots of other stuff.
//

//
// Global parameters/defines.
//
// DOOM version
pub const DOOM_VERSION: i32 = 109;

// Version code for cph's longtics hack ("v1.91")
pub const DOOM_191_VERSION: i32 = 111;

// If rangecheck is undefined,
// most parameter validation debugging code will not be compiled
pub const RANGECHECK: bool = false;

// The maximum number of players, multiplayer/networking.
pub const MAXPLAYERS: usize = 4;

// The current state of the game: whether we are
// playing, gazing at the intermission screen,
// the game final animation, or a demo.
#[derive(Default, Clone, Copy, PartialEq)]
pub enum gamestate_t {
    GS_NONE = -1,
    GS_LEVEL = 0,
    GS_INTERMISSION,
    GS_FINALE,
    #[default]
    GS_DEMOSCREEN,
}

#[derive(PartialEq, Default)]
pub enum gameaction_t {
    ga_nothing,
    ga_loadlevel,
    ga_newgame,
    ga_loadgame,
    ga_savegame,
    ga_playdemo,
    ga_completed,
    ga_victory,
    ga_worlddone,
    #[default]
    ga_screenshot,
}

//
// Difficulty/skill settings/filters.
//

// Skill flags.
pub const MTF_EASY: i32 = 1;
pub const MTF_NORMAL: i32 = 2;
pub const MTF_HARD: i32 = 4;

// Deaf monsters/do not react to sound.
pub const MTF_AMBUSH: i32 = 8;

//
// Key cards.
//
pub enum card_t {
    it_bluecard,
    it_yellowcard,
    it_redcard,
    it_blueskull,
    it_yellowskull,
    it_redskull,

    NUMCARDS,
}

// The defined weapons,
//  including a marker indicating
//  user has not changed weapon.
#[derive(Copy, Clone)]
pub enum weapontype_t {
    wp_fist,
    wp_pistol,
    wp_shotgun,
    wp_chaingun,
    wp_missile,
    wp_plasma,
    wp_bfg,
    wp_chainsaw,
    wp_supershotgun,

    NUMWEAPONS,

    // No pending weapon change.
    wp_nochange,
}

// Ammunition types defined.
pub enum ammotype_t {
    am_clip,  // Pistol / chaingun ammo.
    am_shell, // Shotgun / double barreled shotgun.
    am_cell,  // Plasma rifle, BFG.
    am_misl,  // Missile launcher.
    NUMAMMO,
    am_noammo, // Unlimited for chainsaw / fist.
}

// Power up artifacts.
pub enum powertype_t {
    pw_invulnerability,
    pw_strength,
    pw_invisibility,
    pw_ironfeet,
    pw_allmap,
    pw_infrared,
    NUMPOWERS,
}

//
// Power up durations,
//  how many seconds till expiration,
//  assuming TICRATE is 35 ticks/second.
//
pub enum powerduration_t {
    INVULNTICS = 30 * TICRATE as isize,
    INVISTICS = 60 * TICRATE as isize,
    INFRATICS = 120 * TICRATE as isize,
    IRONTICS = 61 * TICRATE as isize, // BUG was 60
}
//BUG enum to constants, see above
pub const INVULNTICS: isize = 30 * TICRATE as isize;
pub const INVISTICS: isize = 60 * TICRATE as isize;
pub const INFRATICS: isize = 120 * TICRATE as isize;
pub const IRONTICS: isize = 60 * TICRATE as isize;
