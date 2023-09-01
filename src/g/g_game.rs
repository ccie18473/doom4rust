#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// g_game.h
/////////////////////////////
//
// DESCRIPTION:
//   Duh.
//

/////////////////////////////
// g_game.c
/////////////////////////////
//
// DESCRIPTION:  none
//

pub const SAVEGAMESIZE: i32 = 0x2c000;

//pub const MAXPLMOVE		(forwardmove[1])

pub const TURBOTHRESHOLD: u8 = 0x32;

pub const weapon_keys: [*mut i32; 8] = [
    ptr::null_mut(), //&key_weapon1,
    ptr::null_mut(), //&key_weapon2,
    ptr::null_mut(), //&key_weapon3,
    ptr::null_mut(), //&key_weapon4,
    ptr::null_mut(), //&key_weapon5,
    ptr::null_mut(), //&key_weapon6,
    ptr::null_mut(), //&key_weapon7,
    ptr::null_mut(), //&key_weapon8
];

// Used for prev/next weapon keys.

pub struct weapon_t {
    pub weapon: weapontype_t,
    pub weapon_num: weapontype_t,
}

pub const weapon_order_table: [weapon_t; 9] = [
    weapon_t {
        weapon: weapontype_t::wp_fist,
        weapon_num: weapontype_t::wp_fist,
    },
    weapon_t {
        weapon: weapontype_t::wp_chainsaw,
        weapon_num: weapontype_t::wp_fist,
    },
    weapon_t {
        weapon: weapontype_t::wp_pistol,
        weapon_num: weapontype_t::wp_pistol,
    },
    weapon_t {
        weapon: weapontype_t::wp_shotgun,
        weapon_num: weapontype_t::wp_shotgun,
    },
    weapon_t {
        weapon: weapontype_t::wp_supershotgun,
        weapon_num: weapontype_t::wp_shotgun,
    },
    weapon_t {
        weapon: weapontype_t::wp_chaingun,
        weapon_num: weapontype_t::wp_chaingun,
    },
    weapon_t {
        weapon: weapontype_t::wp_missile,
        weapon_num: weapontype_t::wp_missile,
    },
    weapon_t {
        weapon: weapontype_t::wp_plasma,
        weapon_num: weapontype_t::wp_plasma,
    },
    weapon_t {
        weapon: weapontype_t::wp_bfg,
        weapon_num: weapontype_t::wp_bfg,
    },
];

pub const SLOWTURNTICS: u8 = 6;

pub const NUMKEYS: i32 = 256;
pub const MAX_JOY_BUTTONS: u8 = 20;

pub const BODYQUESIZE: u8 = 32;

// DOOM Par Times
pub const pars: [[i32; 10]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 30, 75, 120, 90, 165, 180, 180, 30, 165],
    [0, 90, 90, 90, 120, 90, 360, 240, 30, 170],
    [0, 90, 45, 90, 150, 90, 90, 165, 30, 135],
];

// DOOM II Par Times
pub const cpars: [i32; 32] = [
    30, 90, 120, 120, 90, 150, 120, 120, 270, 90, //  1-10
    210, 150, 150, 150, 210, 150, 420, 150, 210, 150, // 11-20
    240, 150, 180, 150, 150, 300, 330, 420, 300, 180, // 21-30
    120, 30, // 31-32
];

pub const VERSIONSIZE: u8 = 16;

pub const DEMOMARKER: u8 = 0x80;

pub fn G_CmdChecksum(cmd: *mut ticcmd_t) -> i32 {
    return 0;
}

pub fn WeaponSelectable(weapon: weapontype_t) -> bool {
    return false;
}

pub fn G_NextWeapon(direction: i32) -> i32 {
    return 0;
}

//
// G_BuildTiccmd
// Builds a ticcmd from all of the available inputs
// or reads it from the demo buffer.
// If recording a demo, write it out
//
pub fn G_BuildTiccmd(cmd: *mut ticcmd_t, maketic: i32) {}

//
// G_DoLoadLevel
//
pub fn G_DoLoadLevel() {}

pub fn SetJoyButtons(buttons_mask: u32) {}

pub fn SetMouseButtons(buttons_mask: u32) {}

//
// G_Responder
// Get info needed to make ticcmd_ts for the players.
//
pub fn G_Responder(ev: *mut event_t) -> bool {
    return false;
}

//
// G_Ticker
// Make ticcmd_ts for the players.
//
pub fn G_Ticker() {}

//
// PLAYER STRUCTURE FUNCTIONS
// also see P_SpawnPlayer in P_Things
//

//
// G_InitPlayer
// Called at the start.
// Called by the game initialization functions.
//
pub fn G_InitPlayer(player: i32) {}

//
// G_PlayerFinishLevel
// Can when a player completes a level.
//
pub fn G_PlayerFinishLevel(player: i32) {}

//
// G_PlayerReborn
// Called after a player dies
// almost everything is cleared and initialized
//
pub fn G_PlayerReborn(player: i32) {}

//
// G_CheckSpot
// Returns false if the player cannot be respawned
// at the given mapthing_t spot
// because something is occupying it
//

pub fn G_CheckSpot(playernum: i32, mthing: *mut mapthing_t) -> bool {
    return false;
}

//
// G_DeathMatchSpawnPlayer
// Spawns a player at one of the random death match spots
// called at level load and each death
//
pub fn G_DeathMatchSpawnPlayer(playernum: i32) {}

//
// G_DoReborn
//
pub fn G_DoReborn(playernum: i32) {}

pub fn G_ScreenShot() {}

//
// G_DoCompleted
//

pub fn G_ExitLevel() {}

// Here's for the german edition.
pub fn G_SecretExitLevel() {}

pub fn G_DoCompleted() {}

//
// G_WorldDone
//
pub fn G_WorldDone() {}

pub fn G_DoWorldDone() {}

//
// G_InitFromSavegame
// Can be called by the startup code or the menu task.
//

pub fn G_LoadGame(name: &str) {}

pub fn G_DoLoadGame() {}

//
// G_SaveGame
// Called by the menu task.
// Description is a 24 byte text &str
//
pub fn G_SaveGame(slot: i32, description: &str) {}

pub fn G_DoSaveGame() {}

//
// G_InitNew
// Can be called by the startup code or the menu task,
// consoleplayer, displayplayer, playeringame[] should be set.
//

pub fn G_DeferedInitNew(skill: skill_t, episode: i32, map: i32) {}

pub fn G_DoNewGame() {}

pub fn G_InitNew(skill: skill_t, episode: i32, map: i32) {}

//
// DEMO RECORDING
//

pub fn G_ReadDemoTiccmd(cmd: *mut ticcmd_t) {}

// Increase the size of the demo buffer to allow unlimited demos

pub fn IncreaseDemoBuffer() {}

pub fn G_WriteDemoTiccmd(cmd: *mut ticcmd_t) {}

//
// G_RecordDemo
//
pub fn G_RecordDemo(name: &str) {}

// Get the demo version code appropriate for the version set in gameversion.
pub fn G_VanillaVersionCode() -> i32 {
    return 0;
}

pub fn G_BeginRecording() {}

//
// G_PlayDemo
//

pub fn G_DeferedPlayDemo(name: &str) {}

// Generate a &str describing a demo version

pub fn DemoVersionDescription(version: i32) -> &'static str {
    return "";
}

pub fn G_DoPlayDemo() {}

//
// G_TimeDemo
//
pub fn G_TimeDemo(name: &str) {}

/*
===================
=
= G_CheckDemoStatus
=
= Called after a death or level completion to allow demos to be cleaned up
= Returns true if a new demo loop action will take place
===================
*/

pub fn G_CheckDemoStatus() -> bool {
    return false;
}
