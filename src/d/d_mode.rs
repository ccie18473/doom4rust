#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// d_mode.h
/////////////////////////////
//
// DESCRIPTION:
//   Functions and definitions relating to the game type and operational
//   mode.
//

// The "mission" controls what game we are playing.

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameMission_t {
    doom,      // Doom 1
    doom2,     // Doom 2
    pack_tnt,  // Final Doom: TNT: Evilution
    pack_plut, // Final Doom: The Plutonia Experiment
    pack_chex, // Chex Quest (modded doom)
    pack_hacx, // Hacx (modded doom2)
    heretic,   // Heretic
    hexen,     // Hexen
    strife,    // Strife

    none,
}

// The "mode" allows more accurate specification of the game mode we are
// in: eg. shareware vs. registered.  So doom1.wad and doom.wad are the
// same mission, but a different mode.

#[derive(PartialEq)]
pub enum GameMode_t {
    shareware,    // Doom/Heretic shareware
    registered,   // Doom/Heretic registered
    commercial,   // Doom II/Hexen
    retail,       // Ultimate Doom
    indetermined, // Unknown.
}

// What version are we emulating?

#[derive(Clone, Copy, PartialEq)]
pub enum GameVersion_t {
    exe_doom_1_2,   // Doom 1.2: shareware and registered
    exe_doom_1_666, // Doom 1.666: for shareware, registered and commercial
    exe_doom_1_7,   // Doom 1.7/1.7a: "
    exe_doom_1_8,   // Doom 1.8: "
    exe_doom_1_9,   // Doom 1.9: "
    exe_hacx,       // Hacx
    exe_ultimate,   // Ultimate Doom (retail)
    exe_final,      // Final Doom
    exe_final2,     // Final Doom (alternate exe)
    exe_chex,       // Chex Quest executable (based on Final Doom)

    exe_heretic_1_3, // Heretic 1.3

    exe_hexen_1_1,   // Hexen 1.1
    exe_strife_1_2,  // Strife v1.2
    exe_strife_1_31, // Strife v1.31
}

// Skill level.

#[derive(Clone, Copy)]
pub enum skill_t {
    sk_noitems = -1, // the "-skill 0" hack
    sk_baby = 0,
    sk_easy,
    sk_medium,
    sk_hard,
    sk_nightmare,
}

// Check that a gamemode+gamemission received over the network is valid.

pub fn D_ValidGameMode(mission: GameMission_t, mode: GameMode_t) -> bool {
    println!("D_ValidGameMode");

    return false;
}

pub fn D_ValidEpisodeMap(mission: GameMission_t, mode: GameMode_t, episode: i32, map: i32) -> bool {
    println!("D_ValidEpisodeMap");

    return false;
}

// Get the number of valid episodes for the specified mission/mode.

pub fn D_GetNumEpisodes(mission: GameMission_t, mode: GameMode_t) -> i32 {
    println!("D_GetNumEpisodes");

    return 0;
}

// Table of valid versions

pub struct game_version {
    pub mission: GameMission_t,
    pub version: GameVersion_t,
}

pub const valid_versions: [game_version; 10] = [
    game_version {
        mission: GameMission_t::doom,
        version: GameVersion_t::exe_doom_1_9,
    },
    game_version {
        mission: GameMission_t::doom,
        version: GameVersion_t::exe_hacx,
    },
    game_version {
        mission: GameMission_t::doom,
        version: GameVersion_t::exe_ultimate,
    },
    game_version {
        mission: GameMission_t::doom,
        version: GameVersion_t::exe_final,
    },
    game_version {
        mission: GameMission_t::doom,
        version: GameVersion_t::exe_final2,
    },
    game_version {
        mission: GameMission_t::doom,
        version: GameVersion_t::exe_chex,
    },
    game_version {
        mission: GameMission_t::heretic,
        version: GameVersion_t::exe_heretic_1_3,
    },
    game_version {
        mission: GameMission_t::hexen,
        version: GameVersion_t::exe_hexen_1_1,
    },
    game_version {
        mission: GameMission_t::strife,
        version: GameVersion_t::exe_strife_1_2,
    },
    game_version {
        mission: GameMission_t::strife,
        version: GameVersion_t::exe_strife_1_31,
    },
];

pub fn D_ValidGameVersion(mission: GameMission_t, version: GameVersion_t) -> bool {
    println!("D_ValidGameVersion");

    return false;
}

// Does this mission type use ExMy form, rather than MAPxy form?

pub fn D_IsEpisodeMap(mission: GameMission_t) -> bool {
    println!("D_IsEpisodeMap");

    return false;
}

pub fn D_GameMissionString(mission: GameMission_t) -> String {
    println!("D_GameMissionString");

    return "".to_string();
}
