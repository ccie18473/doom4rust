#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/////////////////////////////
// m_controls.h
/////////////////////////////

/////////////////////////////
// m_controls.c
/////////////////////////////

//
// Bind all of the common controls used by Doom and all other games.
//

pub fn M_BindBaseControls() {
    println!("M_BindBaseControls");
}

pub fn M_BindHereticControls() {
    println!("M_BindHereticControls");
}

pub fn M_BindHexenControls() {
    println!("M_BindHexenControls");
}

pub fn M_BindStrifeControls() {
    println!("M_BindStrifeControls");
}

pub fn M_BindWeaponControls() {
    println!("M_BindWeaponControls");
}

pub fn M_BindMapControls() {
    println!("M_BindMapControls");
}

pub fn M_BindMenuControls() {
    println!("M_BindMenuControls");
}

pub fn M_BindChatControls(num_players: u32) {
    println!("M_BindChatControls");
}

//
// Apply custom patches to the default values depending on the
// platform we are running on.
//

pub fn M_ApplyPlatformDefaults() {
    println!("M_ApplyPlatformDefaults");
}
