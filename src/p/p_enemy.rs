#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_enemy.c
/////////////////////////////
//
// DESCRIPTION:
//	Enemy thinking, AI.
//	Action Pointer Functions
//	that are associated with states/frames.
//

//
// ENEMY THINKING
// Enemies are allways spawned
// with targetplayer = -1, threshold = 0
// Most monsters are spawned unaware of all players,
// but some can be made preaware
//

//
// Called by P_NoiseAlert.
// Recursively traverse adjacent sectors,
// sound blocking lines cut off traversal.
//

pub fn P_RecursiveSound(sec: *mut sector_t, soundblocks: i32) {
    println!("P_RecursiveSound");
}

//
// P_NoiseAlert
// If a monster yells at a player,
// it will alert other monsters to the player.
//
pub fn P_NoiseAlert(target: *mut mobj_t, emmiter: *mut mobj_t) {
    println!("P_NoiseAlert");
}

//
// P_CheckMeleeRange
//
pub fn P_CheckMeleeRange(actor: *mut mobj_t) -> bool {
    println!("P_CheckMeleeRange");

    return false;
}

//
// P_CheckMissileRange
//
pub fn P_CheckMissileRange(actor: *mut mobj_t) -> bool {
    println!("P_CheckMissileRange");

    return false;
}

//
// P_Move
// Move in the current direction,
// returns false if the move is blocked.
//

pub fn P_Move(actor: *mut mobj_t) -> bool {
    println!("P_Move");

    return false;
}

//
// TryWalk
// Attempts to move actor on
// in its current (ob->moveangle) direction.
// If blocked by either a wall or an actor
// returns FALSE
// If move is either clear or blocked only by a door,
// returns TRUE and sets...
// If a door is in the way,
// an OpenDoor call is made to start it opening.
//
pub fn P_TryWalk(actor: *mut mobj_t) -> bool {
    println!("P_TryWalk");

    return false;
}

pub fn P_NewChaseDir(actor: *mut mobj_t) {
    println!("P_NewChaseDir");
}

//
// P_LookForPlayers
// If allaround is false, only look 180 degrees in front.
// Returns true if a player is targeted.
//
pub fn P_LookForPlayers(actor: *mut mobj_t, allaround: bool) -> bool {
    println!("P_LookForPlayers");

    return false;
}

//
// A_KeenDie
// DOOM II special, map 32.
// Uses special tag 666.
//
pub fn A_KeenDie(mo: *mut mobj_t) {
    println!("A_KeenDie");
}

//
// ACTION ROUTINES
//

//
// A_Look
// Stay in state until a player is sighted.
//
pub fn A_Look(actor: *mut mobj_t) {
    println!("A_Look");
}

//
// A_Chase
// Actor has a melee attack,
// so it tries to close as fast as possible
//
pub fn A_Chase(actor: *mut mobj_t) {
    println!("A_Chase");
}

//
// A_FaceTarget
//
pub fn A_FaceTarget(actor: *mut mobj_t) {
    println!("A_FaceTarget");
}

//
// A_PosAttack
//
pub fn A_PosAttack(actor: *mut mobj_t) {
    println!("A_PosAttack");
}

pub fn A_SPosAttack(actor: *mut mobj_t) {
    println!("A_SPosAttack");
}

pub fn A_CPosAttack(actor: *mut mobj_t) {
    println!("A_CPosAttack");
}

pub fn A_CPosRefire(actor: *mut mobj_t) {
    println!("A_CPosRefire");
}

pub fn A_SpidRefire(actor: *mut mobj_t) {
    println!("A_SpidRefire");
}

pub fn A_BspiAttack(actor: *mut mobj_t) {
    println!("A_BspiAttack");
}

//
// A_TroopAttack
//
pub fn A_TroopAttack(actor: *mut mobj_t) {
    println!("A_TroopAttack");
}

pub fn A_SargAttack(actor: *mut mobj_t) {
    println!("A_SargAttack");
}

pub fn A_HeadAttack(actor: *mut mobj_t) {
    println!("A_HeadAttack");
}

pub fn A_CyberAttack(actor: *mut mobj_t) {
    println!("A_CyberAttack");
}

pub fn A_BruisAttack(actor: *mut mobj_t) {
    println!("A_BruisAttack");
}

//
// A_SkelMissile
//
pub fn A_SkelMissile(actor: *mut mobj_t) {
    println!("A_SkelMissile");
}

pub fn A_Tracer(actor: *mut mobj_t) {
    println!("A_Tracer");
}

pub fn A_SkelWhoosh(actor: *mut mobj_t) {
    println!("A_SkelWhoosh");
}

pub fn A_SkelFist(actor: *mut mobj_t) {
    println!("A_SkelFist");
}

//
// PIT_VileCheck
// Detect a corpse that could be raised.
//

pub fn PIT_VileCheck(thing: *mut mobj_t) -> bool {
    println!("PIT_VileCheck");

    return false;
}

//
// A_VileChase
// Check for ressurecting a body
//
pub fn A_VileChase(actor: *mut mobj_t) {
    println!("A_VileChase");
}

//
// A_VileStart
//
pub fn A_VileStart(actor: *mut mobj_t) {
    println!("A_VileStart");
}

//
// A_Fire
// Keep fire in front of player unless out of sight
//

pub fn A_StartFire(actor: *mut mobj_t) {
    println!("A_StartFire");
}

pub fn A_FireCrackle(actor: *mut mobj_t) {
    println!("A_FireCrackle");
}

pub fn A_Fire(actor: *mut mobj_t) {
    println!("A_Fire");
}

//
// A_VileTarget
// Spawn the hellfire
//
pub fn A_VileTarget(actor: *mut mobj_t) {
    println!("A_VileTarget");
}

//
// A_VileAttack
//
pub fn A_VileAttack(actor: *mut mobj_t) {
    println!("A_VileAttack");
}

//
// Mancubus attack,
// firing three missiles (bruisers)
// in three different directions?
// Doesn't look like it.
//

pub fn A_FatRaise(actor: *mut mobj_t) {
    println!("A_FatRaise");
}

pub fn A_FatAttack1(actor: *mut mobj_t) {
    println!("A_FatAttack1");
}

pub fn A_FatAttack2(actor: *mut mobj_t) {
    println!("A_FatAttack2");
}

pub fn A_FatAttack3(actor: *mut mobj_t) {
    println!("A_FatAttack3");
}

//
// SkullAttack
// Fly at the player like a missile.
//

pub fn A_SkullAttack(actor: *mut mobj_t) {
    println!("A_SkullAttack");
}

//
// A_PainShootSkull
// Spawn a lost soul and launch it at the target
//
pub fn A_PainShootSkull(actor: *mut mobj_t, angle: angle_t) {
    println!("A_PainShootSkull");
}

//
// A_PainAttack
// Spawn a lost soul and launch it at the target
//
pub fn A_PainAttack(actor: *mut mobj_t) {
    println!("A_PainAttack");
}

pub fn A_PainDie(actor: *mut mobj_t) {
    println!("A_PainDie");
}

pub fn A_Scream(actor: *mut mobj_t) {
    println!("A_Scream");
}

pub fn A_XScream(actor: *mut mobj_t) {
    println!("A_XScream");
}

pub fn A_Pain(actor: *mut mobj_t) {
    println!("A_Pain");
}

pub fn A_Fall(actor: *mut mobj_t) {
    println!("A_Fall");
}

//
// A_Explode
//
pub fn A_Explode(thingy: *mut mobj_t) {
    println!("A_Explode");
}

// Check whether the death of the specified monster type is allowed
// to trigger the end of episode special action.
//
// This behavior changed in v1.9, the most notable effect of which
// was to break uac_dead.wad

pub fn CheckBossEnd(motype: mobjtype_t) -> bool {
    println!("CheckBossEnd");

    return false;
}

//
// A_BossDeath
// Possibly trigger special effects
// if on first boss level
//
pub fn A_BossDeath(mo: *mut mobj_t) {
    println!("A_BossDeath");
}

pub fn A_Hoof(mo: *mut mobj_t) {
    println!("A_Hoof");
}

pub fn A_Metal(mo: *mut mobj_t) {
    println!("A_Metal");
}

pub fn A_BabyMetal(mo: *mut mobj_t) {
    println!("A_BabyMetal");
}

pub fn A_OpenShotgun2(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_OpenShotgun2");
}

pub fn A_LoadShotgun2(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_LoadShotgun2");
}

pub fn A_CloseShotgun2(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_CloseShotgun2");
}

pub fn A_BrainAwake(mo: *mut mobj_t) {
    println!("A_BrainAwake");
}

pub fn A_BrainPain(mo: *mut mobj_t) {
    println!("A_BrainPain");
}

pub fn A_BrainScream(mo: *mut mobj_t) {
    println!("A_BrainScream");
}

pub fn A_BrainExplode(mo: *mut mobj_t) {
    println!("A_BrainExplode");
}

pub fn A_BrainDie(mo: *mut mobj_t) {
    println!("A_BrainDie");
}

pub fn A_BrainSpit(mo: *mut mobj_t) {
    println!("A_BrainSpit");
}

// travelling cube sound
pub fn A_SpawnSound(mo: *mut mobj_t) {
    println!("A_SpawnSound");
}

pub fn A_SpawnFly(mo: *mut mobj_t) {
    println!("A_SpawnFly");
}

pub fn A_PlayerScream(mo: *mut mobj_t) {
    println!("A_PlayerScream");
}
