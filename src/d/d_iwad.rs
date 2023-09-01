#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// d_iwad.h
/////////////////////////////
//
// DESCRIPTION:
//     Find IWAD and initialize according to IWAD type.
//

pub const IWAD_MASK_DOOM: i32 = (1 << GameMission_t::doom as i32)
    | (1 << GameMission_t::doom2 as i32)
    | (1 << GameMission_t::pack_tnt as i32)
    | (1 << GameMission_t::pack_plut as i32)
    | (1 << GameMission_t::pack_chex as i32)
    | (1 << GameMission_t::pack_hacx as i32);
pub const IWAD_MASK_HERETIC: i32 = 1 << GameMission_t::heretic as i32;
pub const IWAD_MASK_HEXEN: i32 = 1 << GameMission_t::hexen as i32;
pub const IWAD_MASK_STRIFE: i32 = 1 << GameMission_t::strife as i32;

pub struct iwad_t<'a> {
    pub name: &'a str,
    pub mission: GameMission_t,
    pub mode: GameMode_t,
    pub description: &'a str,
}

/////////////////////////////
// d_iwad.c
/////////////////////////////
//
// DESCRIPTION:
//     Search for and locate an IWAD file, and initialize according
//     to the IWAD type.
//

pub const iwads: [iwad_t; 14] = [
    iwad_t {
        name: "doom2.wad",
        mission: GameMission_t::doom2,
        mode: GameMode_t::commercial,
        description: "Doom II",
    },
    iwad_t {
        name: "plutonia.wad",
        mission: GameMission_t::pack_plut,
        mode: GameMode_t::commercial,
        description: "Final Doom: Plutonia Experiment",
    },
    iwad_t {
        name: "tnt.wad",
        mission: GameMission_t::pack_tnt,
        mode: GameMode_t::commercial,
        description: "Final Doom: TNT: Evilution",
    },
    iwad_t {
        name: "doom.wad",
        mission: GameMission_t::doom,
        mode: GameMode_t::retail,
        description: "Doom",
    },
    iwad_t {
        name: "doom1.wad",
        mission: GameMission_t::doom,
        mode: GameMode_t::shareware,
        description: "Doom Shareware",
    },
    iwad_t {
        name: "chex.wad",
        mission: GameMission_t::pack_chex,
        mode: GameMode_t::shareware,
        description: "Chex Quest",
    },
    iwad_t {
        name: "hacx.wad",
        mission: GameMission_t::pack_hacx,
        mode: GameMode_t::commercial,
        description: "Hacx",
    },
    iwad_t {
        name: "freedm.wad",
        mission: GameMission_t::doom2,
        mode: GameMode_t::commercial,
        description: "FreeDM",
    },
    iwad_t {
        name: "freedoom2.wad",
        mission: GameMission_t::doom2,
        mode: GameMode_t::commercial,
        description: "Freedoom: Phase 2",
    },
    iwad_t {
        name: "freedoom1.wad",
        mission: GameMission_t::doom,
        mode: GameMode_t::retail,
        description: "Freedoom: Phase 1",
    },
    iwad_t {
        name: "heretic.wad",
        mission: GameMission_t::heretic,
        mode: GameMode_t::retail,
        description: "Heretic",
    },
    iwad_t {
        name: "heretic1.wad",
        mission: GameMission_t::heretic,
        mode: GameMode_t::shareware,
        description: "Heretic Shareware",
    },
    iwad_t {
        name: "hexen.wad",
        mission: GameMission_t::hexen,
        mode: GameMode_t::commercial,
        description: "Hexen",
    },
    //{ "strife0.wad",  strife,    commercial, "Strife" }, // haleyjd: STRIFE-FIXME
    iwad_t {
        name: "strife1.wad",
        mission: GameMission_t::strife,
        mode: GameMode_t::commercial,
        description: "Strife",
    },
];

// Array of locations to search for IWAD files
//
// "128 IWAD search directories should be enough for anybody".

pub const MAX_IWAD_DIRS: usize = 128;

pub const FILES_DIR: &str = ".";

pub fn AddIWADDir(doom: &mut modules, dir: &'static str) {
    println!("AddIWADDir");

    if doom.d.num_iwad_dirs < MAX_IWAD_DIRS as i32 {
        doom.d.iwad_dirs[doom.d.num_iwad_dirs as usize] = dir;
        doom.d.num_iwad_dirs += 1;
    }
}

// Returns true if the specified path is a path to a file
// of the specified name.

pub fn DirIsFile(path: &str, filename: &str) -> bool {
    println!("DirIsFile");

    let path_len: usize;
    let filename_len: usize;

    path_len = path.len();
    filename_len = filename.len();

    //BUG
    //return path_len >= filename_len + 1
    //&& path[path_len - filename_len - 1] == DIR_SEPARATOR
    //&& !strcasecmp(&path[path_len - filename_len], filename);
    return false;
}

// Check if the specified directory contains the specified IWAD
// file, returning the full path to the IWAD if found, or NULL
// if not found.

pub fn CheckDirectoryHasIWAD(dir: &'static str, iwadname: &'static str) -> &'static str {
    println!("CheckDirectoryHasIWAD");

    let filename: &'static str;

    // As a special case, the "directory" may refer directly to an
    // IWAD file if the path comes from DOOMWADDIR or DOOMWADPATH.

    if DirIsFile(dir, iwadname) && M_FileExists(dir) {
        return dir;
    }
    // Construct the full path to the IWAD if it is located in
    // this directory, and check if it exists.

    if dir != "." {
        filename = iwadname;
    } else {
        //BUG
        //filename = M_StringJoin(dir, DIR_SEPARATOR_S, iwadname, NULL);
        //filename = format!("{} {} {}", dir, DIR_SEPARATOR_S, iwadname);
        filename = "doom1.wad";
    }
    println!("Trying IWAD file:{}", filename);

    if M_FileExists(filename) {
        return filename;
    }
    return "\0";
}

// Search a directory to try to find an IWAD
// Returns the location of the IWAD if found, otherwise NULL.

pub fn SearchDirectoryForIWAD(
    dir: &'static str,
    mask: i32,
    mission: *mut GameMission_t,
) -> &'static str {
    println!("SearchDirectoryForIWAD");

    let mut filename: &str;

    for i in 0..iwads.len() {
        if ((1 << iwads[i].mission as i32) & mask) == 0 {
            continue;
        }
        filename = CheckDirectoryHasIWAD(dir, DEH_String(iwads[i].name));

        if !filename.is_empty() {
            unsafe { *mission = iwads[i].mission };
            return filename;
        }
    }

    return "\0";
}

// When given an IWAD with the '-iwad' parameter,
// attempt to identify it by its name.

pub fn IdentifyIWADByName(name: &str, mask: i32) -> GameMission_t {
    println!("IdentifyIWADByName");

    return GameMission_t::none;
}

//
// Build a list of IWAD files
//

pub fn BuildIWADDirList(doom: &mut modules) {
    println!("BuildIWADDirList");

    AddIWADDir(doom, FILES_DIR);

    // Don't run this function again.

    doom.d.iwad_dirs_built = true;
}

//
// Searches WAD search paths for an WAD with a specific filename.
//

pub fn D_FindWADByName(name: &str) -> &'static str {
    println!("D_FindWADByName");

    return "";
}

//
// D_TryWADByName
//
// Searches for a WAD by its filename, or passes through the filename
// if not found.
//

pub fn D_TryFindWADByName(filename: &str) -> &'static str {
    println!("D_TryFindWADByName");

    return "";
}

//
// FindIWAD
// Checks availability of IWAD files by name,
// to determine whether registered/commercial features
// should be executed (notably loading PWADs).
//

pub fn D_FindIWAD(doom: &mut modules, mask: i32, mut mission: *mut GameMission_t) -> &'static str {
    println!("D_FindIWAD");

    mission = &mut doom.doomstat.gamemission;

    let mut result: &'static str;
    let iwadfile: &str;
    let iwadparm: i32;

    // Check for the -iwad parameter

    //
    // Specify an IWAD file to use.
    //
    // @arg <file>
    //

    iwadparm = M_CheckParmWithArgs(doom, "-iwad", 1) as i32;

    if iwadparm != 0 {
        // Search through IWAD dirs for an IWAD with the given name.

        iwadfile = &doom.m.myargv[iwadparm as usize + 1];

        result = D_FindWADByName(iwadfile);

        if result.is_empty() {
            println!("IWAD file '{}' not found!", iwadfile);
            I_Error();
        }

        unsafe { *mission = IdentifyIWADByName(result, mask) };
    } else {
        // Search through the list and look for an IWAD

        println!("-iwad not specified, trying a few iwad names");

        result = "\0";

        BuildIWADDirList(doom);

        for i in 0..doom.d.num_iwad_dirs {
            //BUG
            //let dir = doom.d.iwad_dirs[i as usize].clone();
            let dir = ".";
            result = SearchDirectoryForIWAD(dir, mask, mission);
            if !result.is_empty() {
                break;
            }
        }
    }

    return result;
}

// Find all IWADs in the IWAD search path matching the given mask.

pub fn D_FindAllIWADs(mask: i32) //-> *mut *mut iwad_t
{
    println!("D_FindAllIWADs");
}

//
// Get the IWAD name used for savegames.
//

pub fn D_SaveGameIWADName(gamemission: GameMission_t) -> &'static str {
    println!("D_SaveGameIWADName");

    return "";
}

pub fn D_SuggestIWADName(mission: GameMission_t, mode: GameMode_t) -> &'static str {
    println!("D_SuggestIWADName");

    return "";
}

pub fn D_SuggestGameName(mission: GameMission_t, mode: GameMode_t) -> &'static str {
    println!("D_SuggestGameName");

    return "";
}
