#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// m_config.h
/////////////////////////////
//
// DESCRIPTION:
//      Configuration file interface.
//

/////////////////////////////
// m_config.c
/////////////////////////////
//
// DESCRIPTION:
//      Configuration file interface.
//

// Get the path to the default configuration dir to use, if NULL
// is passed to M_SetConfigDir.

pub fn GetDefaultConfigDir() -> &'static str {
    println!("GetDefaultConfigDir");

    return "";
}

//
// SetConfigDir:
//
// Sets the location of the configuration directory, where configuration
// files are stored - default.cfg, chocolate-doom.cfg, savegames, etc.
//

pub fn M_SetConfigDir(dir: &str) {
    println!("M_SetConfigDir");
}

// Set the default filenames to use for configuration files.

pub fn M_SetConfigFilenames(main_config: &str, extra_config: &str) {
    println!("M_SetConfigFilenames");
}

//
// M_LoadDefaults
//

pub fn M_LoadDefaults() {
    println!("M_LoadDefaults");
}

//
// M_SaveDefaults
//

pub fn M_SaveDefaults(doom: &mut modules) {
    println!("M_SaveDefaults");
}

//
// Calculate the path to the directory to use to store save games.
// Creates the directory as necessary.
//

pub fn M_GetSaveGameDir(iwadname: &str) -> &'static str {
    println!("M_GetSaveGameDir");

    return "";
}
