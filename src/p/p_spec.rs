#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_spec.h
/////////////////////////////
//
// DESCRIPTION:  none
//	Implements special effects:
//	Texture animation, height or lighting changes
//	 according to adjacent sectors, respective
//	 utility functions, etc.
//

//      Define values for map objects
pub const MO_TELEPORTMAN: u8 = 14;

pub const GLOWSPEED: u8 = 8;
pub const STROBEBRIGHT: u8 = 5;
pub const FASTDARK: u8 = 15;
pub const SLOWDARK: u8 = 35;

// max # of wall switches in a level
pub const MAXSWITCHES: u8 = 50;

// 4 players, 4 buttons each at once, max.
pub const MAXBUTTONS: u8 = 16;

// 1 second, in ticks.
pub const BUTTONTIME: u8 = 35;

pub const PLATWAIT: u8 = 3;
pub const PLATSPEED: i32 = FRACUNIT;
pub const MAXPLATS: u8 = 30;

pub const VDOORSPEED: i32 = FRACUNIT * 2;
pub const VDOORWAIT: u8 = 150;

pub const CEILSPEED: i32 = FRACUNIT;
pub const CEILWAIT: u8 = 150;
pub const MAXCEILINGS: u8 = 30;

pub const FLOORSPEED: i32 = FRACUNIT;

//
// P_LIGHTS
//
pub struct fireflicker_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: i32,
    pub maxlight: i32,
    pub minlight: i32,
}

pub struct lightflash_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: i32,
    pub maxlight: i32,
    pub minlight: i32,
    pub maxtime: i32,
    pub mintime: i32,
}

pub struct strobe_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub count: i32,
    pub minlight: i32,
    pub maxlight: i32,
    pub darktime: i32,
    pub brighttime: i32,
}

pub struct glow_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub minlight: i32,
    pub maxlight: i32,
    pub direction: i32,
}

//
// P_SWITCH
//
pub struct switchlist_t<'a> {
    pub name1: &'a str,
    pub name2: &'a str,
    pub episode: i16,
}

#[derive(Clone, Copy)]
pub enum bwhere_e {
    top,
    middle,
    bottom,
}

#[derive(Clone, Copy)]
pub struct button_t {
    pub line: *mut line_t,
    pub Where: bwhere_e,
    pub btexture: i32,
    pub btimer: i32,
    pub soundorg: *mut degenmobj_t,
}

impl button_t {
    pub fn new() -> Self {
        Self {
            line: ptr::null_mut(),
            Where: bwhere_e::middle,
            btexture: 0,
            btimer: 0,
            soundorg: ptr::null_mut(),
        }
    }
}
//
// P_PLATS
//
pub enum plat_e {
    up,
    down,
    waiting,
    in_stasis,
}

pub enum plattype_e {
    perpetualRaise,
    downWaitUpStay,
    raiseAndChange,
    raiseToNearestAndChange,
    blazeDWUS,
}

pub struct plat_t {
    pub thinker: thinker_t,
    pub sector: *mut sector_t,
    pub speed: fixed_t,
    pub low: fixed_t,
    pub high: fixed_t,
    pub wait: i32,
    pub count: i32,
    pub status: plat_e,
    pub oldstatus: plat_e,
    pub crush: bool,
    pub tag: i32,
    pub Type: plattype_e,
}

//
// P_DOORS
//
pub enum vldoor_e {
    vld_normal,
    vld_close30ThenOpen,
    vld_close,
    vld_open,
    vld_raiseIn5Mins,
    vld_blazeRaise,
    vld_blazeOpen,
    vld_blazeClose,
}

pub struct vldoor_t {
    pub thinker: thinker_t,
    pub Type: vldoor_e,
    pub sector: *mut sector_t,
    pub topheight: fixed_t,
    pub speed: fixed_t,
    // 1 = up, 0 = waiting at top, -1 = down
    pub direction: i32,
    // tics to wait at the top
    pub topwait: i32,
    // (keep in case a door going down is reset)
    // when it reaches 0, start going down
    pub topcountdown: i32,
}

//
// P_CEILNG
//
pub enum ceiling_e {
    lowerToFloor,
    raiseToHighest,
    lowerAndCrush,
    crushAndRaise,
    fastCrushAndRaise,
    silentCrushAndRaise,
}

pub struct ceiling_t {
    pub thinker: thinker_t,
    pub Type: ceiling_e,
    pub sector: *mut sector_t,
    pub bottomheight: fixed_t,
    pub topheight: fixed_t,
    pub speed: fixed_t,
    pub crush: bool,
    // 1 = up, 0 = waiting, -1 = down
    pub direction: i32,
    // ID
    pub tag: i32,
    pub olddirection: i32,
}

//
// P_FLOOR
//
pub enum floor_e {
    // lower floor to highest surrounding floor
    lowerFloor,

    // lower floor to lowest surrounding floor
    lowerFloorToLowest,

    // lower floor to highest surrounding floor VERY FAST
    turboLower,

    // raise floor to lowest surrounding CEILING
    raiseFloor,

    // raise floor to next highest surrounding floor
    raiseFloorToNearest,

    // raise floor to shortest height texture around it
    raiseToTexture,

    // lower floor to lowest surrounding floor
    //  and change floorpic
    lowerAndChange,

    raiseFloor24,
    raiseFloor24AndChange,
    raiseFloorCrush,

    // raise to next highest floor, turbo-speed
    raiseFloorTurbo,
    donutRaise,
    raiseFloor512,
}

pub enum stair_e {
    build8,  // slowly build by 8
    turbo16, // quickly build by 16
}

pub struct floormove_t {
    pub thinker: thinker_t,
    pub Type: floor_e,
    pub crush: bool,
    pub sector: *mut sector_t,
    pub direction: i32,
    pub newspecial: i32,
    pub texture: i16,
    pub floordestheight: fixed_t,
    pub speed: fixed_t,
}
pub enum result_e {
    ok,
    crushed,
    pastdest,
}

/////////////////////////////
// p_spec.c
/////////////////////////////
//
// DESCRIPTION:
//	Implements special effects:
//	Texture animation, height or lighting changes
//	 according to adjacent sectors, respective
//	 utility functions, etc.
//	Line Tag handling. Line and Sector triggers.
//

//
// Animating textures and planes
// There is another anim_t used in wi_stuff, unrelated.
//

pub const MAXANIMS: u8 = 32;

#[derive(Clone, Copy)]
pub struct anim_t {
    pub istexture: bool,
    pub picnum: i32,
    pub basepic: i32,
    pub numpics: i32,
    pub speed: i32,
}

impl anim_t {
    pub fn new() -> Self {
        Self {
            istexture: false,
            picnum: 0,
            basepic: 0,
            numpics: 0,
            speed: 0,
        }
    }
}

//
//      source animation definition
//
pub struct animdef_t<'a> {
    pub istexture: i32, // if false, it is a flat
    pub endname: &'a str,
    pub startname: &'a str,
    pub speed: i32,
}

//
// P_InitPicAnims
//

// Floor/ceiling animation sequences,
//  defined by first and last frame,
//  i.e. the flat (64x64 tile) name to
//  be used.
// The full animation sequence is given
//  using all the flats between the start
//  and end entry, in the order found in
//  the WAD file.
//
pub const animdefs: [animdef_t; 23] = [
    animdef_t {
        istexture: 0,
        endname: "NUKAGE3",
        startname: "NUKAGE1",
        speed: 8,
    },
    animdef_t {
        istexture: 0,
        endname: "FWATER4",
        startname: "FWATER1",
        speed: 8,
    },
    animdef_t {
        istexture: 0,
        endname: "SWATER4",
        startname: "SWATER1",
        speed: 8,
    },
    animdef_t {
        istexture: 0,
        endname: "LAVA4",
        startname: "LAVA1",
        speed: 8,
    },
    animdef_t {
        istexture: 0,
        endname: "BLOOD3",
        startname: "BLOOD1",
        speed: 8,
    },
    // DOOM II flat animations.
    animdef_t {
        istexture: 0,
        endname: "RROCK08",
        startname: "RROCK05",
        speed: 8,
    },
    animdef_t {
        istexture: 0,
        endname: "SLIME04",
        startname: "SLIME01",
        speed: 8,
    },
    animdef_t {
        istexture: 0,
        endname: "SLIME08",
        startname: "SLIME05",
        speed: 8,
    },
    animdef_t {
        istexture: 0,
        endname: "SLIME12",
        startname: "SLIME09",
        speed: 8,
    },
    animdef_t {
        istexture: 1,
        endname: "BLODGR4",
        startname: "BLODGR1",
        speed: 8,
    },
    animdef_t {
        istexture: 1,
        endname: "SLADRIP3",
        startname: "SLADRIP1",
        speed: 8,
    },
    animdef_t {
        istexture: 1,
        endname: "BLODRIP4",
        startname: "BLODRIP1",
        speed: 8,
    },
    animdef_t {
        istexture: 1,
        endname: "FIREWALL",
        startname: "FIREWALA",
        speed: 8,
    },
    animdef_t {
        istexture: 1,
        endname: "GSTFONT3",
        startname: "GSTFONT1",
        speed: 8,
    },
    animdef_t {
        istexture: 1,
        endname: "FIRELAVA",
        startname: "FIRELAV3",
        speed: 8,
    },
    animdef_t {
        istexture: 1,
        endname: "FIREMAG3",
        startname: "FIREMAG1",
        speed: 8,
    },
    animdef_t {
        istexture: 1,
        endname: "FIREBLU2",
        startname: "FIREBLU1",
        speed: 8,
    },
    animdef_t {
        istexture: 1,
        endname: "ROCKRED3",
        startname: "ROCKRED1",
        speed: 8,
    },
    animdef_t {
        istexture: 1,
        endname: "BFALL4",
        startname: "BFALL1",
        speed: 8,
    },
    animdef_t {
        istexture: 1,
        endname: "SFALL4",
        startname: "SFALL1",
        speed: 8,
    },
    animdef_t {
        istexture: 1,
        endname: "WFALL4",
        startname: "WFALL1",
        speed: 8,
    },
    animdef_t {
        istexture: 1,
        endname: "DBRAIN4",
        startname: "DBRAIN1",
        speed: 8,
    },
    animdef_t {
        istexture: -1,
        endname: "",
        startname: "",
        speed: 0,
    },
];

//
//      Animating line specials
//
pub const MAXLINEANIMS: u8 = 64;

pub const DONUT_FLOORHEIGHT_DEFAULT: i32 = 0x00000000;
pub const DONUT_FLOORPIC_DEFAULT: i32 = 0x16;

// 20 adjoining sectors max!
pub const MAX_ADJOINING_SECTORS: u8 = 20;

pub fn P_InitPicAnims() {
    println!("P_InitPicAnims");
}

//
// UTILITIES
//

//
// getSide()
// Will return a side_t*
//  given the number of the current sector,
//  the line number, and the side (0/1) that you want.
//
pub fn getSide(currentSector: i32, line: i32, side: i32) -> *mut side_t {
    println!("getSide");

    return ptr::null_mut();
}

//
// getSector()
// Will return a sector_t*
//  given the number of the current sector,
//  the line number and the side (0/1) that you want.
//
pub fn getSector(currentSector: i32, line: i32, side: i32) -> *mut sector_t {
    println!("getSector");

    return ptr::null_mut();
}

//
// twoSided()
// Given the sector number and the line number,
//  it will tell you whether the line is two-sided or not.
//
pub fn twoSided(sector: i32, line: i32) -> i32 {
    println!("twoSided");

    return 0;
}

//
// getNextSector()
// Return sector_t * of sector next to current.
// NULL if not two-sided line
//
pub fn getNextSector(line: *mut line_t, sec: *mut sector_t) -> *mut sector_t {
    println!("getNextSector");

    return ptr::null_mut();
}

//
// P_FindLowestFloorSurrounding()
// FIND LOWEST FLOOR HEIGHT IN SURROUNDING SECTORS
//
pub fn P_FindLowestFloorSurrounding(sec: *mut sector_t) -> fixed_t {
    println!("P_FindLowestFloorSurrounding");

    return 0;
}

//
// P_FindHighestFloorSurrounding()
// FIND HIGHEST FLOOR HEIGHT IN SURROUNDING SECTORS
//
pub fn P_FindHighestFloorSurrounding(sec: *mut sector_t) -> fixed_t {
    println!("P_FindHighestFloorSurrounding");

    return 0;
}

//
// P_FindNextHighestFloor
// FIND NEXT HIGHEST FLOOR IN SURROUNDING SECTORS
// Note: this should be doable w/o a fixed array.

// Thanks to entryway for the Vanilla overflow emulation.

pub fn P_FindNextHighestFloor(sec: *mut sector_t, currentheight: i32) -> fixed_t {
    println!("P_FindNextHighestFloor");

    return 0;
}

//
// FIND LOWEST CEILING IN THE SURROUNDING SECTORS
//

pub fn P_FindLowestCeilingSurrounding(sec: *mut sector_t) -> fixed_t {
    println!("P_FindLowestCeilingSurrounding");

    return 0;
}

//
// FIND HIGHEST CEILING IN THE SURROUNDING SECTORS
//
pub fn P_FindHighestCeilingSurrounding(sec: *mut sector_t) -> fixed_t {
    println!("P_FindHighestCeilingSurrounding");

    return 0;
}

//
// RETURN NEXT SECTOR # THAT LINE TAG REFERS TO
//
pub fn P_FindSectorFromLineTag(line: *mut line_t, start: i32) -> i32 {
    println!("P_FindSectorFromLineTag");

    return 0;
}

//
// Find minimum light from an adjacent sector
//
pub fn P_FindMinSurroundingLight(sector: *mut sector_t, max: i32) -> i32 {
    println!("P_FindMinSurroundingLight");

    return 0;
}

//
// EVENTS
// Events are operations triggered by using, crossing,
// or shooting special lines, or by timed thinkers.
//

//
// P_CrossSpecialLine - TRIGGER
// Called every time a thing origin is about
//  to cross a line with a non 0 special.
//
pub fn P_CrossSpecialLine(linenum: i32, side: i32, thing: *mut mobj_t) {
    println!("P_CrossSpecialLine");
}

//
// P_ShootSpecialLine - IMPACT SPECIALS
// Called when a thing shoots a special line.
//
pub fn P_ShootSpecialLine(thing: *mut mobj_t, line: *mut line_t) {
    println!("P_ShootSpecialLine");
}

//
// P_PlayerInSpecialSector
// Called every tic frame
//  that the player origin is in a special sector
//
pub fn P_PlayerInSpecialSector(player: *mut player_t) {
    println!("P_PlayerInSpecialSector");
}

//
// P_UpdateSpecials
// Animate planes, scroll walls, etc.
//

pub fn P_UpdateSpecials() {
    println!("P_UpdateSpecials");
}

//
// Donut overrun emulation
//
// Derived from the code from PrBoom+.  Thanks go to Andrey Budko (entryway)
// as usual :-)
//

pub fn DonutOverrun(
    s3_floorheight: *mut fixed_t,
    s3_floorpic: *mut i16,
    line: *mut line_t,
    pillar_sector: *mut sector_t,
) {
    println!("DonutOverrun");
}

//
// Special Stuff that can not be categorized
//
pub fn EV_DoDonut(line: *mut line_t) -> i32 {
    println!("EV_DoDonut");

    return 0;
}

//
// SPECIAL SPAWNING
//

//
// P_SpawnSpecials
// After the map has been loaded, scan for specials
//  that spawn thinkers
//

// Parses command line parameters.
pub fn P_SpawnSpecials() {
    println!("P_SpawnSpecials");
}
