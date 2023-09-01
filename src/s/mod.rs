#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod s_sound;

pub use s_sound::*;

use crate::*;

/////////////////////////////
// S_* Sound code
/////////////////////////////

pub struct s<'a> {
    /////////////////////////////
    // s_sound.c
    /////////////////////////////

    // The set of channels available
    pub channels: *mut channel_t<'a>,

    // Maximum volume of a sound effect.
    // Internal default is max out of 0-15.
    pub sfxVolume: i32,

    // Maximum volume of music.
    pub musicVolume: i32,

    // Internal volume level, ranging from 0-127
    pub snd_SfxVolume: i32,

    // Whether songs are mus_paused
    pub mus_paused: bool,

    // Music currently being played
    pub mus_playing: *const musicinfo_t<'a>,

    // Number of channels to use
    pub snd_channels: i32,
}

impl<'a> s<'a> {
    pub fn new() -> Self {
        Self {
            /////////////////////////////
            // s_sound.c
            /////////////////////////////
            channels: ptr::null_mut(),
            sfxVolume: 8,
            musicVolume: 8,
            snd_SfxVolume: 0,
            mus_paused: false,
            mus_playing: ptr::null(),
            snd_channels: 8,
        }
    }
}
