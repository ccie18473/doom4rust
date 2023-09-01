#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// d_player.h
/////////////////////////////
//
// DESCRIPTION:
//
//

//
// Player states.
//
pub enum playerstate_t {
    // Playing or camping.
    PST_LIVE,
    // Dead on the ground, view follows killer.
    PST_DEAD,
    // Ready to restart/respawn???
    PST_REBORN,
}

//
// Player internal flags, for cheats and debug.
//
pub enum cheat_t {
    // No clipping, walk through barriers.
    CF_NOCLIP = 1,
    // No damage, no health loss.
    CF_GODMODE = 2,
    // Not really a cheat, just a debug aid.
    CF_NOMOMENTUM = 4,
}

//
// Extended player object info: player_t
//

pub type player_s = player_t;
pub struct player_t {
    pub mo: *mut mobj_t,
    pub playerstate: playerstate_t,
    pub cmd: ticcmd_t,

    // Determine POV,
    //  including viewpoint bobbing during movement.
    // Focal origin above r.z
    pub viewz: i32,
    // Base height above floor for viewz.
    pub viewheight: i32,
    // Bob/squat speed.
    pub deltaviewheight: i32,
    // bounded/scaled total momentum.
    pub bob: i32,

    // This is only used between levels,
    // mo->health is used during levels.
    pub health: i32,
    pub armorpoints: i32,
    // Armor type is 0-2.
    pub armortype: i32,

    // Power ups. invinc and invis are tic counters.
    pub powers: [bool; powertype_t::NUMPOWERS as usize],
    pub cards: [bool; card_t::NUMCARDS as usize],
    pub backpack: bool,

    // Frags, kills of other players.
    pub frags: [i32; MAXPLAYERS],
    pub readyweapon: weapontype_t,

    // Is wp_nochange if not changing.
    pub pendingweapon: weapontype_t,

    pub weaponowned: [bool; weapontype_t::NUMWEAPONS as usize],
    pub ammo: [i32; ammotype_t::NUMAMMO as usize],
    pub maxammo: [i32; ammotype_t::NUMAMMO as usize],

    // True if button down last tic.
    pub attackdown: i32,
    pub usedown: i32,

    // Bit flags, for cheats and debug.
    // See cheat_t, above.
    pub cheats: i32,

    // Refired shots are less accurate.
    pub refire: i32,

    // For intermission stats.
    pub killcount: i32,
    pub itemcount: i32,
    pub secretcount: i32,

    // Hint messages.
    pub message: String,

    // For screen flashing (red or bright).
    pub damagecount: i32,
    pub bonuscount: i32,

    // Who did damage (NULL for floors/ceilings).
    pub attacker: *mut mobj_t,

    // So gun flashes light up areas.
    pub extralight: i32,

    // Current PLAYPAL, ???
    //  can be set to REDCOLORMAP for pain, etc.
    pub fixedcolormap: i32,

    // Player skin colorshift,
    //  0-3 for which color to draw player.
    pub colormap: i32,

    // Overlay view sprites (gun, etc).
    pub psprites: [pspdef_t; psprnum_t::NUMPSPRITES as usize],

    // True if secret level has been done.
    pub didsecret: bool,
}

impl player_t {
    pub fn new() -> Self {
        Self {
            mo: ptr::null_mut(),
            playerstate: playerstate_t::PST_DEAD,
            cmd: ticcmd_t::new(),
            viewz: 0,
            viewheight: 0,
            deltaviewheight: 0,
            bob: 0,
            health: 0,
            armorpoints: 0,
            armortype: 0,
            powers: [false; 6],
            cards: [false; 6],
            backpack: false,
            frags: [0; MAXPLAYERS],
            readyweapon: weapontype_t::wp_fist,
            pendingweapon: weapontype_t::wp_pistol,
            weaponowned: [false; 9],
            ammo: [0; 4],
            maxammo: [0; 4],
            attackdown: 0,
            usedown: 0,
            cheats: 0,
            refire: 0,
            killcount: 0,
            itemcount: 0,
            secretcount: 0,
            message: "".to_string(),
            damagecount: 0,
            bonuscount: 0,
            attacker: ptr::null_mut(),
            extralight: 0,
            fixedcolormap: 0,
            colormap: 0,
            psprites: [pspdef_t::new(); psprnum_t::NUMPSPRITES as usize],
            didsecret: false,
        }
    }
}

//
// INTERMISSION
// Structure passed e.g. to WI_Start(wb)
//
pub struct wbplayerstruct_t {
    pub r#in: bool, // whether the player is in game

    // Player stats, kills, collected items etc.
    pub skills: i32,
    pub sitems: i32,
    pub ssecret: i32,
    pub stime: i32,
    pub frags: [i32; 4],
    pub score: i32, // current score on entry, modified on return
}

pub struct wbstartstruct_t {
    pub epsd: i32, // episode # (0-2)

    // if true, splash the secret level
    pub didsecret: bool,

    // previous and next levels, origin 0
    pub last: i32,
    pub next: i32,

    pub maxkills: i32,
    pub maxitems: i32,
    pub maxsecret: i32,
    pub maxfrags: i32,

    // the par time
    pub partime: i32,

    // index of this player in game
    pub pnum: i32,

    pub plyr: [wbplayerstruct_t; MAXPLAYERS],
}
