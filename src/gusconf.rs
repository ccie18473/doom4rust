#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

/////////////////////////////
// gusconf.h
/////////////////////////////
//
// DESCRIPTION:
//     GUS emulation code.
//

/////////////////////////////
// gusconf.c
/////////////////////////////
//
// DESCRIPTION:
//     GUS emulation code.
//
//     Actually emulating a GUS is far too much work; fortunately
//     GUS "emulation" already exists in the form of Timidity, which
//     supports GUS patch files. This code therefore converts Doom's
//     DMXGUS lump into an equivalent Timidity configuration file.
//

pub const MAX_INSTRUMENTS: usize = 256;

pub struct gus_config_t {
    pub patch_names: [char; MAX_INSTRUMENTS],
    pub mapping: [i32; MAX_INSTRUMENTS],
}

pub fn MappingIndex() -> u32 {
    println!("MappingIndex");

    return 0;
}

pub fn SplitLine(line: &str, fields: *mut &str, max_fields: u32) -> i32 {
    println!("SplitLine");

    return 0;
}

pub fn ParseLine(config: *mut gus_config_t, line: &str) {
    println!("ParseLine");
}

pub fn ParseDMXConfig(dmxconf: &str, config: *mut gus_config_t) {
    println!("ParseDMXConfig");
}

pub fn FreeDMXConfig(config: *mut gus_config_t) {
    println!("FreeDMXConfig");
}

pub fn ReadDMXConfig() -> &'static str {
    println!("ReadDMXConfig");

    return "";
}

pub fn WriteTimidityConfig(path: &str, config: *mut gus_config_t) -> bool {
    println!("WriteTimidityConfig");

    return false;
}

pub fn GUS_WriteConfig(path: &str) -> bool {
    println!("GUS_WriteConfig");

    return false;
}
