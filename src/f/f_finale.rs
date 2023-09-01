#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// f_finale.h
/////////////////////////////
//
// DESCRIPTION:
//
//

/////////////////////////////
// f_finale.c
/////////////////////////////
//
// DESCRIPTION:
//	Game completion, final screen animation.
//

pub enum finalestage_t {
    F_STAGE_TEXT,
    F_STAGE_ARTSCREEN,
    F_STAGE_CAST,
}

pub const TEXTSPEED: u8 = 3;
pub const TEXTWAIT: u8 = 250;

pub struct textscreen_t<'a> {
    pub mission: GameMission_t,
    pub episode: i32,
    pub level: i32,
    pub background: &'a str,
    pub text: &'a str,
}

pub const textscreens: [textscreen_t; 22] = [
    textscreen_t {
        mission: GameMission_t::doom,
        episode: 1,
        level: 8,
        background: "FLOOR4_8",
        text: E1TEXT,
    },
    textscreen_t {
        mission: GameMission_t::doom,
        episode: 2,
        level: 8,
        background: "SFLR6_1",
        text: E2TEXT,
    },
    textscreen_t {
        mission: GameMission_t::doom,
        episode: 3,
        level: 8,
        background: "MFLR8_4",
        text: E3TEXT,
    },
    textscreen_t {
        mission: GameMission_t::doom,
        episode: 4,
        level: 8,
        background: "MFLR8_3",
        text: E4TEXT,
    },
    textscreen_t {
        mission: GameMission_t::doom2,
        episode: 1,
        level: 6,
        background: "SLIME16",
        text: C1TEXT,
    },
    textscreen_t {
        mission: GameMission_t::doom2,
        episode: 1,
        level: 11,
        background: "RROCK14",
        text: C2TEXT,
    },
    textscreen_t {
        mission: GameMission_t::doom2,
        episode: 1,
        level: 20,
        background: "RROCK07",
        text: C3TEXT,
    },
    textscreen_t {
        mission: GameMission_t::doom2,
        episode: 1,
        level: 30,
        background: "RROCK17",
        text: C4TEXT,
    },
    textscreen_t {
        mission: GameMission_t::doom2,
        episode: 1,
        level: 15,
        background: "RROCK13",
        text: C5TEXT,
    },
    textscreen_t {
        mission: GameMission_t::doom2,
        episode: 1,
        level: 31,
        background: "RROCK19",
        text: C6TEXT,
    },
    textscreen_t {
        mission: GameMission_t::pack_tnt,
        episode: 1,
        level: 6,
        background: "SLIME16",
        text: T1TEXT,
    },
    textscreen_t {
        mission: GameMission_t::pack_tnt,
        episode: 1,
        level: 11,
        background: "RROCK14",
        text: T2TEXT,
    },
    textscreen_t {
        mission: GameMission_t::pack_tnt,
        episode: 1,
        level: 20,
        background: "RROCK07",
        text: T3TEXT,
    },
    textscreen_t {
        mission: GameMission_t::pack_tnt,
        episode: 1,
        level: 30,
        background: "RROCK17",
        text: T4TEXT,
    },
    textscreen_t {
        mission: GameMission_t::pack_tnt,
        episode: 1,
        level: 15,
        background: "RROCK13",
        text: T5TEXT,
    },
    textscreen_t {
        mission: GameMission_t::pack_tnt,
        episode: 1,
        level: 31,
        background: "RROCK19",
        text: T6TEXT,
    },
    textscreen_t {
        mission: GameMission_t::pack_plut,
        episode: 1,
        level: 6,
        background: "SLIME16",
        text: P1TEXT,
    },
    textscreen_t {
        mission: GameMission_t::pack_plut,
        episode: 1,
        level: 11,
        background: "RROCK14",
        text: P2TEXT,
    },
    textscreen_t {
        mission: GameMission_t::pack_plut,
        episode: 1,
        level: 20,
        background: "RROCK07",
        text: P3TEXT,
    },
    textscreen_t {
        mission: GameMission_t::pack_plut,
        episode: 1,
        level: 30,
        background: "RROCK17",
        text: P4TEXT,
    },
    textscreen_t {
        mission: GameMission_t::pack_plut,
        episode: 1,
        level: 15,
        background: "RROCK13",
        text: P5TEXT,
    },
    textscreen_t {
        mission: GameMission_t::pack_plut,
        episode: 1,
        level: 31,
        background: "RROCK19",
        text: P6TEXT,
    },
];

//
// F_StartFinale
//
pub fn F_StartFinale() {}

pub fn F_Responder(event: *mut event_t) -> bool {
    return false;
}

//
// F_Ticker
//
pub fn F_Ticker() {}

//
// F_TextWrite
//

pub fn F_TextWrite() {}

//
// Final DOOM 2 animation
// Casting by id Software.
//   in order of appearance
//
pub struct castinfo_t<'a> {
    pub name: &'a str,
    pub Type: mobjtype_t,
}

pub const castorder: [castinfo_t; 18] = [
    castinfo_t {
        name: CC_ZOMBIE,
        Type: mobjtype_t::MT_POSSESSED,
    },
    castinfo_t {
        name: CC_SHOTGUN,
        Type: mobjtype_t::MT_SHOTGUY,
    },
    castinfo_t {
        name: CC_HEAVY,
        Type: mobjtype_t::MT_CHAINGUY,
    },
    castinfo_t {
        name: CC_IMP,
        Type: mobjtype_t::MT_TROOP,
    },
    castinfo_t {
        name: CC_DEMON,
        Type: mobjtype_t::MT_SERGEANT,
    },
    castinfo_t {
        name: CC_LOST,
        Type: mobjtype_t::MT_SKULL,
    },
    castinfo_t {
        name: CC_CACO,
        Type: mobjtype_t::MT_HEAD,
    },
    castinfo_t {
        name: CC_HELL,
        Type: mobjtype_t::MT_KNIGHT,
    },
    castinfo_t {
        name: CC_BARON,
        Type: mobjtype_t::MT_BRUISER,
    },
    castinfo_t {
        name: CC_ARACH,
        Type: mobjtype_t::MT_BABY,
    },
    castinfo_t {
        name: CC_PAIN,
        Type: mobjtype_t::MT_PAIN,
    },
    castinfo_t {
        name: CC_REVEN,
        Type: mobjtype_t::MT_UNDEAD,
    },
    castinfo_t {
        name: CC_MANCU,
        Type: mobjtype_t::MT_FATSO,
    },
    castinfo_t {
        name: CC_ARCH,
        Type: mobjtype_t::MT_VILE,
    },
    castinfo_t {
        name: CC_SPIDER,
        Type: mobjtype_t::MT_SPIDER,
    },
    castinfo_t {
        name: CC_CYBER,
        Type: mobjtype_t::MT_CYBORG,
    },
    castinfo_t {
        name: CC_HERO,
        Type: mobjtype_t::MT_PLAYER,
    },
    castinfo_t {
        name: "",
        Type: mobjtype_t::MT_PLAYER,
    },
];

//
// F_StartCast
//
pub fn F_StartCast() {}

//
// F_CastTicker
//
pub fn F_CastTicker() {}

//
// F_CastResponder
//

pub fn F_CastResponder(ev: *mut event_t) -> bool {
    return false;
}

pub fn F_CastPrint(text: String) {}

//
// F_CastDrawer
//

pub fn F_CastDrawer() {}

//
// F_DrawPatchCol
//
pub fn F_DrawPatchCol(x: i32, patch: *mut patch_t, col: i32) {}

//
// F_BunnyScroll
//
pub fn F_BunnyScroll() {}

pub fn F_ArtScreenDrawer() {}

//
// F_Drawer
//
pub fn F_Drawer() {}
