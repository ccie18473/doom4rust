#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

/////////////////////////////
// doomstat.h
/////////////////////////////
//
// DESCRIPTION:
//   All the global variables that store the internal state.
//   Theoretically speaking, the internal state of the engine
//    should be found by looking at the variables collected
//    here, and every relevant module will have to include
//    this header file.
//   In practice, things are a bit messy.
//

// Convenience macro.
// 'gamemission' can be equal to pack_chex or pack_hacx, but these are
// just modified versions of doom and doom2, and should be interpreted
// as the same most of the time.

//#define logical_gamemission                             \
//  (gamemission == pack_chex ? doom :                  \
//    gamemission == pack_hacx ? doom2 : gamemission)

// Player spawn spots for deathmatch.
pub const MAX_DM_STARTS: u8 = 10;

/////////////////////////////
// doomstat.c
/////////////////////////////
//
// DESCRIPTION:
//	Put all global tate variables here.
//

/////////////////////////////
// Doomstat
/////////////////////////////

pub struct doomstat<'a> {
    // Game Mode - identify IWAD as shareware, retail etc.
    pub gamemode: GameMode_t,
    pub gamemission: GameMission_t,
    pub gameversion: GameVersion_t,
    pub gamedescription: &'a str,

    // Set if homebrew PWAD stuff has been added.
    pub modifiedgame: bool,
}

impl<'a> doomstat<'a> {
    pub fn new() -> Self {
        Self {
            gamemode: GameMode_t::indetermined,
            gamemission: GameMission_t::doom,
            gameversion: GameVersion_t::exe_final2,
            gamedescription: "",
            modifiedgame: false,
        }
    }
}

// Convenience macro.
// 'gamemission' can be equal to pack_chex or pack_hacx, but these are
// just modified versions of doom and doom2, and should be interpreted
// as the same most of the time.

pub fn logical_gamemission(doom: &mut modules) {
    if doom.doomstat.gamemission == GameMission_t::pack_chex {
        doom.doomstat.gamemission = GameMission_t::doom;
    } else if doom.doomstat.gamemission == GameMission_t::pack_hacx {
        doom.doomstat.gamemission = GameMission_t::doom2;
    }
}
