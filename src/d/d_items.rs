#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// d_items.c
/////////////////////////////
//
// DESCRIPTION:
//

//
// PSPRITE ACTIONS for waepons.
// This struct controls the weapon animations.
//
// Each entry is:
//   ammo/amunition type
//  upstate
//  downstate
// readystate
// atkstate, i.e. attack/fire/hit frame
// flashstate, muzzle flash
//
pub const weaponinfo: [weaponinfo_t; weapontype_t::NUMWEAPONS as usize] = [
    weaponinfo_t {
        // fist
        ammo: ammotype_t::am_noammo,
        downstate: statenum_t::S_PUNCHUP as i32,
        upstate: statenum_t::S_PUNCHDOWN as i32,
        readystate: statenum_t::S_PUNCH as i32,
        atkstate: statenum_t::S_PUNCH1 as i32,
        flashstate: statenum_t::S_NULL as i32,
    },
    weaponinfo_t {
        // pistol
        ammo: ammotype_t::am_clip,
        downstate: statenum_t::S_PISTOLUP as i32,
        upstate: statenum_t::S_PISTOLDOWN as i32,
        readystate: statenum_t::S_PISTOL as i32,
        atkstate: statenum_t::S_PISTOL1 as i32,
        flashstate: statenum_t::S_PISTOLFLASH as i32,
    },
    weaponinfo_t {
        // shotgun
        ammo: ammotype_t::am_shell,
        downstate: statenum_t::S_SGUNUP as i32,
        upstate: statenum_t::S_SGUNDOWN as i32,
        readystate: statenum_t::S_SGUN as i32,
        atkstate: statenum_t::S_SGUN1 as i32,
        flashstate: statenum_t::S_SGUNFLASH1 as i32,
    },
    weaponinfo_t {
        // chaingun
        ammo: ammotype_t::am_clip,
        downstate: statenum_t::S_CHAINUP as i32,
        upstate: statenum_t::S_CHAINDOWN as i32,
        readystate: statenum_t::S_CHAIN as i32,
        atkstate: statenum_t::S_CHAIN1 as i32,
        flashstate: statenum_t::S_CHAINFLASH1 as i32,
    },
    weaponinfo_t {
        // missile launcher
        ammo: ammotype_t::am_misl,
        downstate: statenum_t::S_MISSILEUP as i32,
        upstate: statenum_t::S_MISSILEDOWN as i32,
        readystate: statenum_t::S_MISSILE as i32,
        atkstate: statenum_t::S_MISSILE1 as i32,
        flashstate: statenum_t::S_MISSILEFLASH1 as i32,
    },
    weaponinfo_t {
        // plasma rifle
        ammo: ammotype_t::am_cell,
        downstate: statenum_t::S_PLASMAUP as i32,
        upstate: statenum_t::S_PLASMADOWN as i32,
        readystate: statenum_t::S_PLASMA as i32,
        atkstate: statenum_t::S_PLASMA1 as i32,
        flashstate: statenum_t::S_PLASMAFLASH1 as i32,
    },
    weaponinfo_t {
        // bfg 9000
        ammo: ammotype_t::am_cell,
        downstate: statenum_t::S_BFGUP as i32,
        upstate: statenum_t::S_BFGDOWN as i32,
        readystate: statenum_t::S_BFG as i32,
        atkstate: statenum_t::S_BFG1 as i32,
        flashstate: statenum_t::S_BFGFLASH1 as i32,
    },
    weaponinfo_t {
        // chainsaw
        ammo: ammotype_t::am_noammo,
        downstate: statenum_t::S_SAWUP as i32,
        upstate: statenum_t::S_SAWDOWN as i32,
        readystate: statenum_t::S_SAW as i32,
        atkstate: statenum_t::S_SAW1 as i32,
        flashstate: statenum_t::S_NULL as i32,
    },
    weaponinfo_t {
        // super shotgun
        ammo: ammotype_t::am_shell,
        downstate: statenum_t::S_DSGUNUP as i32,
        upstate: statenum_t::S_DSGUNDOWN as i32,
        readystate: statenum_t::S_DSGUN as i32,
        atkstate: statenum_t::S_DSGUN1 as i32,
        flashstate: statenum_t::S_DSGUNFLASH1 as i32,
    },
];
