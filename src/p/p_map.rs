#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_map.c
/////////////////////////////
//
// DESCRIPTION:
//	Movement, collision handling.
//	Shooting and aiming.
//

//
// TELEPORT MOVE
//

//
// PIT_StompThing
//
pub fn PIT_StompThing(thing: *mut mobj_t) -> bool {
    println!("PIT_StompThing");

    return false;
}

//
// P_TeleportMove
//
pub fn P_TeleportMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> bool {
    println!("P_TeleportMove");

    return false;
}

//
// MOVEMENT ITERATOR FUNCTIONS
//

//
// PIT_CheckLine
// Adjusts tmfloorz and tmceilingz as lines are contacted
//
pub fn PIT_CheckLine(ld: *mut line_t) -> bool {
    println!("PIT_CheckLine");

    return false;
}

//
// PIT_CheckThing
//
pub fn PIT_CheckThing(thing: *mut mobj_t) -> bool {
    println!("PIT_CheckThing");

    return false;
}

//
// MOVEMENT CLIPPING
//

//
// P_CheckPosition
// This is purely informative, nothing is modified
// (except things picked up).
//
// in:
//  a mobj_t (can be valid or invalid)
//  a position to be checked
//   (doesn't need to be related to the mobj_t->x,y)
//
// during:
//  special things are touched if MF_PICKUP
//  early out on solid lines?
//
// out:
//  newsubsec
//  floorz
//  ceilingz
//  tmdropoffz
//   the lowest point contacted
//   (monsters won't move to a dropoff)
//  speciallines[]
//  numspeciallines
//
pub fn P_CheckPosition(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> bool {
    println!("P_CheckPosition");

    return false;
}

//
// P_TryMove
// Attempt to move to a new position,
// crossing special lines unless MF_TELEPORT is set.
//
pub fn P_TryMove(thing: *mut mobj_t, x: fixed_t, y: fixed_t) -> bool {
    println!("P_TryMove");

    return false;
}

//
// P_ThingHeightClip
// Takes a valid thing and adjusts the thing->floorz,
// thing->ceilingz, and possibly thing->z.
// This is called for all nearby monsters
// whenever a sector changes height.
// If the thing doesn't fit,
// the z will be set to the lowest value
// and false will be returned.
//
pub fn P_ThingHeightClip(thing: *mut mobj_t) -> bool {
    println!("P_ThingHeightClip");

    return false;
}

//
// P_HitSlideLine
// Adjusts the xmove / ymove
// so that the next move will slide along the wall.
//
pub fn P_HitSlideLine(ld: *mut line_t) {
    println!("P_HitSlideLine");
}

//
// PTR_SlideTraverse
//
pub fn PTR_SlideTraverse(In: *mut intercept_t) -> bool {
    println!("PTR_SlideTraverse");

    return false;
}

//
// P_SlideMove
// The momx / momy move is bad, so try to slide
// along a wall.
// Find the first line hit, move flush to it,
// and slide along it
//
// This is a kludgy mess.
//
pub fn P_SlideMove(mo: *mut mobj_t) {
    println!("P_SlideMove");
}

//
// PTR_AimTraverse
// Sets linetaget and aimslope when a target is aimed at.
//
pub fn PTR_AimTraverse(In: *mut intercept_t) -> bool {
    println!("PTR_AimTraverse");

    return false;
}

//
// PTR_ShootTraverse
//
pub fn PTR_ShootTraverse(In: *mut intercept_t) -> bool {
    println!("PTR_ShootTraverse");

    return false;
}

//
// P_AimLineAttack
//
pub fn P_AimLineAttack(t1: *mut mobj_t, angle: angle_t, distance: fixed_t) -> fixed_t {
    println!("P_AimLineAttack");

    return 0;
}

//
// P_LineAttack
// If damage == 0, it is just a test trace
// that will leave linetarget set.
//
pub fn P_LineAttack(
    t1: *mut mobj_t,
    angle: angle_t,
    distance: fixed_t,
    slope: fixed_t,
    damage: i32,
) {
    println!("P_LineAttack");
}

//
// USE LINES
//

pub fn PTR_UseTraverse(In: *mut intercept_t) -> bool {
    println!("PTR_UseTraverse");

    return false;
}

//
// P_UseLines
// Looks for special lines in front of the player to activate.
//
pub fn P_UseLines(player: *mut player_t) {
    println!("P_UseLines");
}

//
// RADIUS ATTACK
//

//
// PIT_RadiusAttack
// "bombsource" is the creature
// that caused the explosion at "bombspot".
//
pub fn PIT_RadiusAttack(thing: *mut mobj_t) -> bool {
    println!("PIT_RadiusAttack");

    return false;
}

//
// P_RadiusAttack
// Source is the creature that caused the explosion at spot.
//
pub fn P_RadiusAttack(spot: *mut mobj_t, source: *mut mobj_t, damage: i32) {
    println!("P_RadiusAttack");
}

//
// SECTOR HEIGHT CHANGING
// After modifying a sectors floor or ceiling height,
// call this routine to adjust the positions
// of all things that touch the sector.
//
// If anything doesn't fit anymore, true will be returned.
// If crunch is true, they will take damage
//  as they are being crushed.
// If Crunch is false, you should set the sector height back
//  the way it was and call P_ChangeSector again
//  to undo the changes.
//

//
// PIT_ChangeSector
//
pub fn PIT_ChangeSector(thing: *mut mobj_t) -> bool {
    println!("PIT_ChangeSector");

    return false;
}

//
// P_ChangeSector
//
pub fn P_ChangeSector(sector: *mut sector_t, crunch: bool) -> bool {
    println!("P_ChangeSector");

    return false;
}

// Code to emulate the behavior of Vanilla Doom when encountering an overrun
// of the spechit array.  This is by Andrey Budko (e6y) and comes from his
// PrBoom plus port.  A big thanks to Andrey for this.

pub fn SpechitOverrun(ld: *mut line_t) {
    println!("SpechitOverrun");
}
