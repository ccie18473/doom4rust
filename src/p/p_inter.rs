#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_inter.h
/////////////////////////////

/////////////////////////////
// p_inter.c
/////////////////////////////
//
// DESCRIPTION:
//	Handling interactions (i.e., collisions).
//

//
// GET STUFF
//

//
// P_GiveAmmo
// Num is the number of clip loads,
// not the individual count (0= 1/2 clip).
// Returns false if the ammo can't be picked up at all
//

pub fn P_GiveAmmo(player: *mut player_t, ammo: ammotype_t, num: i32) -> bool {
    println!("P_GiveAmmo");

    return false;
}

//
// P_GiveWeapon
// The weapon name may have a MF_DROPPED flag ored in.
//
pub fn P_GiveWeapon(player: *mut player_t, weapon: weapontype_t, dropped: bool) -> bool {
    println!("P_GiveWeapon");

    return false;
}

//
// P_GiveBody
// Returns false if the body isn't needed at all
//
pub fn P_GiveBody(player: *mut player_t, num: i32) -> bool {
    println!("P_GiveBody");

    return false;
}

//
// P_GiveArmor
// Returns false if the armor is worse
// than the current armor.
//
pub fn P_GiveArmor(player: *mut player_t, armortype: i32) -> bool {
    println!("P_GiveArmor");

    return false;
}

//
// P_GiveCard
//
pub fn P_GiveCard(player: *mut player_t, card: card_t) {
    println!("P_GiveCard");
}

//
// P_GivePower
//
pub fn P_GivePower(player: *mut player_t, power: i32) -> bool {
    println!("P_GivePower");

    return false;
}

//
// P_TouchSpecialThing
//
pub fn P_TouchSpecialThing(special: *mut mobj_t, toucher: *mut mobj_t) {
    println!("P_TouchSpecialThing");
}

//
// KillMobj
//
pub fn P_KillMobj(source: *mut mobj_t, target: *mut mobj_t) {
    println!("P_KillMobj");
}

//
// P_DamageMobj
// Damages both enemies and players
// "inflictor" is the thing that caused the damage
//  creature or missile, can be NULL (slime, etc)
// "source" is the thing to target after taking damage
//  creature or NULL
// Source and inflictor are the same for melee attacks.
// Source can be NULL for slime, barrel explosions
// and other environmental stuff.
//
pub fn P_DamageMobj(target: *mut mobj_t, inflictor: *mut mobj_t, source: *mut mobj_t, damage: i32) {
    println!("P_DamageMobj");
}
