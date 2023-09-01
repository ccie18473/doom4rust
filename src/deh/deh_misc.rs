#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/////////////////////////////
// deh_misc.h
/////////////////////////////
//
// Parses "Misc" sections in dehacked files
//

pub const DEH_DEFAULT_INITIAL_HEALTH: u8 = 100;
pub const DEH_DEFAULT_INITIAL_BULLETS: u8 = 50;
pub const DEH_DEFAULT_MAX_HEALTH: u8 = 200;
pub const DEH_DEFAULT_MAX_ARMOR: u8 = 200;
pub const DEH_DEFAULT_GREEN_ARMOR_CLASS: u8 = 1;
pub const DEH_DEFAULT_BLUE_ARMOR_CLASS: u8 = 2;
pub const DEH_DEFAULT_MAX_SOULSPHERE: u8 = 200;
pub const DEH_DEFAULT_SOULSPHERE_HEALTH: u8 = 100;
pub const DEH_DEFAULT_MEGASPHERE_HEALTH: u8 = 200;
pub const DEH_DEFAULT_GOD_MODE_HEALTH: u8 = 100;
pub const DEH_DEFAULT_IDFA_ARMOR: u8 = 200;
pub const DEH_DEFAULT_IDFA_ARMOR_CLASS: u8 = 2;
pub const DEH_DEFAULT_IDKFA_ARMOR: u8 = 200;
pub const DEH_DEFAULT_IDKFA_ARMOR_CLASS: u8 = 2;
pub const DEH_DEFAULT_BFG_CELLS_PER_SHOT: u8 = 40;
pub const DEH_DEFAULT_SPECIES_INFIGHTING: u8 = 0;

// If dehacked is disabled, hard coded values

pub const deh_initial_health: u8 = DEH_DEFAULT_INITIAL_HEALTH;
pub const deh_initial_bullets: u8 = DEH_DEFAULT_INITIAL_BULLETS;
pub const deh_max_health: u8 = DEH_DEFAULT_MAX_HEALTH;
pub const deh_max_armor: u8 = DEH_DEFAULT_MAX_ARMOR;
pub const deh_green_armor_class: u8 = DEH_DEFAULT_GREEN_ARMOR_CLASS;
pub const deh_blue_armor_class: u8 = DEH_DEFAULT_BLUE_ARMOR_CLASS;
pub const deh_max_soulsphere: u8 = DEH_DEFAULT_MAX_SOULSPHERE;
pub const deh_soulsphere_health: u8 = DEH_DEFAULT_SOULSPHERE_HEALTH;
pub const deh_megasphere_health: u8 = DEH_DEFAULT_MEGASPHERE_HEALTH;
pub const deh_god_mode_health: u8 = DEH_DEFAULT_GOD_MODE_HEALTH;
pub const deh_idfa_armor: u8 = DEH_DEFAULT_IDFA_ARMOR;
pub const deh_idfa_armor_class: u8 = DEH_DEFAULT_IDFA_ARMOR_CLASS;
pub const deh_idkfa_armor: u8 = DEH_DEFAULT_IDKFA_ARMOR;
pub const deh_idkfa_armor_class: u8 = DEH_DEFAULT_IDKFA_ARMOR_CLASS;
pub const deh_bfg_cells_per_shot: u8 = DEH_DEFAULT_BFG_CELLS_PER_SHOT;
pub const deh_species_infighting: u8 = DEH_DEFAULT_SPECIES_INFIGHTING;
