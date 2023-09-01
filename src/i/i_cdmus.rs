#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

pub fn I_CDMusInit() {}

// We cannot print status messages inline during startup, they must
// be deferred until after I_CDMusInit has returned.

pub fn I_CDMusPrintStartup() {
    println!("I_CDMusPrintStartup");
}

pub fn I_CDMusPlay(track: i32) -> i32 {
    println!("I_CDMusPlay");

    return 0;
}

pub fn I_CDMusStop() -> i32 {
    println!("I_CDMusStop");

    return 0;
}

pub fn I_CDMusSetVolume(volume: i32) -> i32 {
    println!("I_CDMusSetVolume");

    return 0;
}

pub fn I_CDMusFirstTrack() -> i32 {
    println!("I_CDMusFirstTrack");

    return 0;
}

pub fn I_CDMusLastTrack() -> i32 {
    println!("I_CDMusLastTrack");

    return 0;
}

pub fn I_CDMusTrackLength(track_num: i32) -> i32 {
    println!("I_CDMusTrackLength");

    return 0;
}
