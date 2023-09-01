#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_mobj.h
/////////////////////////////
//
// DESCRIPTION:
//	Map Objects, MObj, definition and handling.
//

//
// NOTES: mobj_t
//
// mobj_ts are used to tell the refresh where to draw an image,
// tell the world simulation when objects are contacted,
// and tell the sound driver how to position a sound.
//
// The refresh uses the next and prev links to follow
// lists of things in sectors as they are being drawn.
// The sprite, frame, and angle elements determine which patch_t
// is used to draw the sprite if it is visible.
// The sprite and frame values are allmost allways set
// from state_t structures.
// The statescr.exe utility generates the states.h and states.c
// files that contain the sprite/frame numbers from the
// statescr.txt source file.
// The xyz origin point represents a point at the bottom middle
// of the sprite (between the feet of a biped).
// This is the default origin position for patch_ts grabbed
// with lumpy.exe.
// A walking creature will have its z equal to the floor
// it is standing on.
//
// The sound code uses the x,y, and subsector fields
// to do stereo positioning of any sound effited by the mobj_t.
//
// The play simulation uses the blocklinks, x,y,z, radius, height
// to determine when mobj_ts are touching each other,
// touching lines in the map, or hit by trace lines (gunshots,
// lines of sight, etc).
// The mobj_t->flags element has various bit flags
// used by the simulation.
//
// Every mobj_t is linked into a single sector
// based on its origin coordinates.
// The subsector_t is found with R_PointInSubsector(x,y),
// and the sector_t can be found with subsector->sector.
// The sector links are only used by the rendering code,
// the play simulation does not care about them at all.
//
// Any mobj_t that needs to be acted upon by something else
// in the play world (block movement, be shot, etc) will also
// need to be linked into the blockmap.
// If the thing has the MF_NOBLOCK flag set, it will not use
// the block links. It can still interact with other things,
// but only as the instigator (missiles will run into other
// things, but nothing can run into a missile).
// Each block in the grid is 128*128 units, and knows about
// every line_t that it contains a piece of, and every
// interactable mobj_t that has its origin contained.
//
// A valid mobj_t is a mobj_t that has the proper subsector_t
// filled in for its xy coordinates and is linked into the
// sector from which the subsector was made, or has the
// MF_NOSECTOR flag set (the subsector_t needs to be valid
// even if MF_NOSECTOR is set), and is linked into a blockmap
// block or has the MF_NOBLOCKMAP flag set.
// Links should only be modified by the P_[Un]SetThingPosition()
// functions.
// Do not change the MF_NO? flags while a thing is valid.
//
// Any questions?
//

//
// Misc. mobj flags
//
pub enum mobjflag_t {
    // Call P_SpecialThing when touched.
    MF_SPECIAL = 1,
    // Blocks.
    MF_SOLID = 2,
    // Can be hit.
    MF_SHOOTABLE = 4,
    // Don't use the sector links (invisible but touchable).
    MF_NOSECTOR = 8,
    // Don't use the blocklinks (inert but displayable)
    MF_NOBLOCKMAP = 16,

    // Not to be activated by sound, deaf monster.
    MF_AMBUSH = 32,
    // Will try to attack right back.
    MF_JUSTHIT = 64,
    // Will take at least one step before attacking.
    MF_JUSTATTACKED = 128,
    // On level spawning (initial position),
    //  hang from ceiling instead of stand on floor.
    MF_SPAWNCEILING = 256,
    // Don't apply gravity (every tic),
    //  that is, object will float, keeping current height
    //  or changing it actively.
    MF_NOGRAVITY = 512,

    // Movement flags.
    // This allows jumps from high places.
    MF_DROPOFF = 0x400,
    // For players, will pick up items.
    MF_PICKUP = 0x800,
    // Player cheat. ???
    MF_NOCLIP = 0x1000,
    // Player: keep info about sliding along walls.
    MF_SLIDE = 0x2000,
    // Allow moves to any height, no gravity.
    // For active floaters, e.g. cacodemons, pain elementals.
    MF_FLOAT = 0x4000,
    // Don't cross lines
    //   ??? or look at heights on teleport.
    MF_TELEPORT = 0x8000,
    // Don't hit same species, explode on block.
    // Player missiles as well as fireballs of various kinds.
    MF_MISSILE = 0x10000,
    // Dropped by a demon, not level spawned.
    // E.g. ammo clips dropped by dying former humans.
    MF_DROPPED = 0x20000,
    // Use fuzzy draw (shadow demons or spectres),
    //  temporary player invisibility powerup.
    MF_SHADOW = 0x40000,
    // Flag: don't bleed when shot (use puff),
    //  barrels and shootable furniture shall not bleed.
    MF_NOBLOOD = 0x80000,
    // Don't stop moving halfway off a step,
    //  that is, have dead bodies slide down all the way.
    MF_CORPSE = 0x100000,
    // Floating to a height for a move, ???
    //  don't auto float to target's height.
    MF_INFLOAT = 0x200000,

    // On kill, count this enemy object
    //  towards intermission kill total.
    // Happy gathering.
    MF_COUNTKILL = 0x400000,

    // On picking up, count this item object
    //  towards intermission item total.
    MF_COUNTITEM = 0x800000,

    // Special handling: skull in flight.
    // Neither a cacodemon nor a missile.
    MF_SKULLFLY = 0x1000000,

    // Don't spawn this object
    //  in death match mode (e.g. key cards).
    MF_NOTDMATCH = 0x2000000,

    // Player sprites in multiplayer modes are modified
    //  using an internal color lookup table for re-indexing.
    // If 0x4 0x8 or 0xc,
    //  use a translation table for player colormaps
    MF_TRANSLATION = 0xc000000,
    // Hmm ???.
    MF_TRANSSHIFT = 26,
}

// Map Object definition.

type mobj_s = mobj_t;
pub struct mobj_t {
    // List: thinker links.
    pub thinker: thinker_t,

    // Info for drawing: position.
    pub x: i32,
    pub y: i32,
    pub z: i32,

    // More list: links in sector (if needed)
    pub snext: *mut mobj_s,
    pub sprev: *mut mobj_s,

    //More drawing info: to determine current sprite.
    pub angle: u32,          // orientation
    pub sprite: spritenum_t, // used to find patch_t and flip value
    pub frame: i32,          // might be ORed with FF_FULLBRIGHT

    // Interaction info, by BLOCKMAP.
    // Links in blocks (if needed).
    pub bnext: *mut mobj_s,
    pub bprev: *mut mobj_s,

    pub subsector: *mut subsector_s,

    // The closest interval over all contacted Sectors.
    pub floorz: i32,
    pub ceilingz: i32,

    // For movement checking.
    pub radius: i32,
    pub height: i32,

    // Momentums, used to update position.
    pub momx: i32,
    pub momy: i32,
    pub momz: i32,

    // If == validcount, already checked.
    pub validcount: i32,

    pub Type: mobjtype_t,
    pub info: *mut mobjinfo_t, // &mobjinfo[mobj->type]

    pub tics: i32, // state tic counter
    pub state: *mut state_t,
    pub flags: i32,
    pub health: i32,

    // Movement direction, movement generation (zig-zagging).
    pub movedir: i32,   // 0-7
    pub movecount: i32, // when 0, select a new dir

    // Thing being chased/attacked (or NULL),
    // also the originator for missiles.
    pub target: *mut mobj_s,

    // Reaction time: if non 0, don't attack yet.
    // Used by player to freeze a bit after teleporting.
    pub reactiontime: i32,

    // If >0, the target will be chased
    // no matter what (even if shot)
    pub threshold: i32,

    // Additional info record for player avatars only.
    // Only valid if type == MT_PLAYER
    pub player: *mut player_s,

    // Player number last looked for.
    pub lastlook: i32,

    // For nightmare respawn.
    pub spawnpoint: mapthing_t,

    // Thing being chased/attacked for tracers.
    pub tracer: *mut mobj_s,
}

/////////////////////////////
// p_mobj.c
/////////////////////////////
//
// DESCRIPTION:
//	Moving object handling. Spawn functions.
//

//
// P_SetMobjState
// Returns true if the mobj is still present.
//

pub fn P_SetMobjState(mobj: *mut mobj_t, state: statenum_t) -> bool {
    println!("P_SetMobjState");

    return false;
}

//
// P_ExplodeMissile
//
pub fn P_ExplodeMissile(mo: *mut mobj_t) {
    println!("P_ExplodeMissile");
}

//
// P_XYMovement
//

pub fn P_XYMovement(mo: *mut mobj_t) {
    println!("P_XYMovement");
}

//
// P_ZMovement
//
pub fn P_ZMovement(mo: *mut mobj_t) {
    println!("P_ZMovement");
}

//
// P_NightmareRespawn
//
pub fn P_NightmareRespawn(mobj: *mut mobj_t) {
    println!("P_NightmareRespawn");
}

//
// P_MobjThinker
//
pub fn P_MobjThinker(mobj: *mut mobj_t) {
    println!("P_MobjThinker");
}

//
// P_SpawnMobj
//
pub fn P_SpawnMobj(x: fixed_t, y: fixed_t, z: fixed_t, Type: mobjtype_t) -> *mut mobj_t {
    println!("P_SpawnMobj");

    return ptr::null_mut();
}

//
// P_RemoveMobj
//

pub fn P_RemoveMobj(mobj: *mut mobj_t) {
    println!("P_RemoveMobj");
}

//
// P_RespawnSpecials
//
pub fn P_RespawnSpecials() {
    println!("P_RespawnSpecials");
}

//
// P_SpawnPlayer
// Called when a player is spawned on the level.
// Most of the player structure stays unchanged
//  between levels.
//
pub fn P_SpawnPlayer(mthing: *mut mapthing_t) {
    println!("P_SpawnPlayer");
}

//
// P_SpawnMapThing
// The fields of the mapthing should
// already be in host byte order.
//
pub fn P_SpawnMapThing(mthing: *mut mapthing_t) {
    println!("P_SpawnMapThing");
}

//
// GAME SPAWN FUNCTIONS
//

//
// P_SpawnPuff
//

pub fn P_SpawnPuff(x: fixed_t, y: fixed_t, z: fixed_t) {
    println!("P_SpawnPuff");
}

//
// P_SpawnBlood
//
pub fn P_SpawnBlood(x: fixed_t, y: fixed_t, z: fixed_t, damage: i32) {
    println!("P_SpawnBlood");
}

//
// P_CheckMissileSpawn
// Moves the missile forward a bit
//  and possibly explodes it right there.
//
pub fn P_CheckMissileSpawn(th: *mut mobj_t) {
    println!("P_CheckMissileSpawn");
}

// Certain functions assume that a mobj_t pointer is non-NULL,
// causing a crash in some situations where it is NULL.  Vanilla
// Doom did not crash because of the lack of proper memory
// protection. This function substitutes NULL pointers for
// pointers to a dummy mobj, to avoid a crash.

pub fn P_SubstNullMobj(mobj: *mut mobj_t) -> *mut mobj_t {
    println!("P_SubstNullMobj");

    return ptr::null_mut();
}

//
// P_SpawnMissile
//
pub fn P_SpawnMissile(source: *mut mobj_t, dest: *mut mobj_t, Type: mobjtype_t) -> *mut mobj_t {
    println!("P_SpawnMissile");

    return ptr::null_mut();
}

//
// P_SpawnPlayerMissile
// Tries to aim at a nearby monster
//
pub fn P_SpawnPlayerMissile(source: *mut mobj_t, Type: mobjtype_t) {
    println!("P_SpawnPlayerMissile");
}
