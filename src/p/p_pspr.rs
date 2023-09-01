#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_pspr.h
/////////////////////////////
//
// DESCRIPTION:
//  Sprite animation.
//

//
// Frame flags:
// handles maximum brightness (torches, muzzle flare, light sources)
//
pub const FF_FULLBRIGHT: i32 = 0x8000; // flag in thing->frame
pub const FF_FRAMEMASK: i32 = 0x7fff;

//
// Overlay psprites are scaled shapes
// drawn directly on the view screen,
// coordinates are given for a 320*200 view screen.
//
pub enum psprnum_t {
    ps_weapon,
    ps_flash,
    NUMPSPRITES,
}

#[derive(Clone, Copy)]
pub struct pspdef_t {
    pub state: *mut state_t, // a NULL state means not active
    pub tics: i32,
    pub sx: i32,
    pub sy: i32,
}

impl pspdef_t {
    pub fn new() -> Self {
        Self {
            state: ptr::null_mut(),
            tics: 0,
            sx: 0,
            sy: 0,
        }
    }
}

/////////////////////////////
// p_pspr.c
/////////////////////////////
//
// DESCRIPTION:
//	Weapon sprite animation, weapon objects.
//	Action functions for weapons.
//

//
// P_SetPsprite
//
pub fn P_SetPsprite(player: *mut player_t, position: i32, stnum: statenum_t) {
    println!("P_SetPsprite");
}

//
// P_CalcSwing
//

pub fn P_CalcSwing(player: *mut player_t) {
    println!("P_CalcSwing");
}

//
// P_BringUpWeapon
// Starts bringing the pending weapon up
// from the bottom of the screen.
// Uses player
//
pub fn P_BringUpWeapon(player: *mut player_t) {
    println!("P_BringUpWeapon");
}

//
// P_CheckAmmo
// Returns true if there is enough ammo to shoot.
// If not, selects the next weapon to use.
//
pub fn P_CheckAmmo(player: *mut player_t) -> bool {
    println!("P_CheckAmmo");

    return false;
}

//
// P_FireWeapon.
//
pub fn P_FireWeapon(player: *mut player_t) {
    println!("P_FireWeapon");
}

//
// P_DropWeapon
// Player died, so put the weapon away.
//
pub fn P_DropWeapon(player: *mut player_t) {
    println!("P_DropWeapon");
}

//
// A_WeaponReady
// The player can fire the weapon
// or change to another weapon at this time.
// Follows after getting weapon up,
// or after previous attack/fire sequence.
//
pub fn A_WeaponReady(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_WeaponReady");
}

//
// A_ReFire
// The player can re-fire the weapon
// without lowering it entirely.
//
pub fn A_ReFire(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_ReFire");
}

pub fn A_CheckReload(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_CheckReload");
}

//
// A_Lower
// Lowers current weapon,
//  and changes weapon at bottom.
//
pub fn A_Lower(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_Lower");
}

//
// A_Raise
//
pub fn A_Raise(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_Raise");
}

//
// A_GunFlash
//
pub fn A_GunFlash(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_GunFlash");
}

//
// WEAPON ATTACKS
//

//
// A_Punch
//
pub fn A_Punch(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_Punch");
}

//
// A_Saw
//
pub fn A_Saw(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_Saw");
}

// Doom does not check the bounds of the ammo array.  As a result,
// it is possible to use an ammo type > 4 that overflows into the
// maxammo array and affects that instead.  Through dehacked, for
// example, it is possible to make a weapon that decreases the max
// number of ammo for another weapon.  Emulate this.

pub fn DecreaseAmmo(player: *mut player_t, ammonum: i32, amount: i32) {
    println!("DecreaseAmmo");
}

//
// A_FireMissile
//
pub fn A_FireMissile(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_FireMissile");
}

//
// A_FireBFG
//
pub fn A_FireBFG(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_FireBFG");
}

//
// A_FirePlasma
//
pub fn A_FirePlasma(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_FirePlasma");
}

//
// P_BulletSlope
// Sets a slope so a near miss is at aproximately
// the height of the intended target
//

pub fn P_BulletSlope(mo: *mut mobj_t) {
    println!("P_BulletSlope");
}

//
// P_GunShot
//
pub fn P_GunShot(mo: *mut mobj_t, accurate: bool) {
    println!("P_GunShot");
}

//
// A_FirePistol
//
pub fn A_FirePistol(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_FirePistol");
}

//
// A_FireShotgun
//
pub fn A_FireShotgun(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_FireShotgun");
}

//
// A_FireShotgun2
//
pub fn A_FireShotgun2(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_FireShotgun2");
}

//
// A_FireCGun
//
pub fn A_FireCGun(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_FireCGun");
}

//
// ?
//
pub fn A_Light0(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_Light0");
}

pub fn A_Light1(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_Light1");
}

pub fn A_Light2(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_Light2");
}

//
// A_BFGSpray
// Spawn a BFG explosion on every monster in view
//
pub fn A_BFGSpray(mo: *mut mobj_t) {
    println!("A_BFGSpray");
}

//
// A_BFGsound
//
pub fn A_BFGsound(player: *mut player_t, psp: *mut pspdef_t) {
    println!("A_BFGsound");
}

//
// P_SetupPsprites
// Called at start of level for each player.
//
pub fn P_SetupPsprites(player: *mut player_t) {
    println!("P_SetupPsprites");
}

//
// P_MovePsprites
// Called every tic by player thinking routine.
//
pub fn P_MovePsprites(player: *mut player_t) {
    println!("P_MovePsprites");
}
