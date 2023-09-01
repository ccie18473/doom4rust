#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/////////////////////////////
extern crate libc;
extern crate sdl2;

pub const MIX_CHANNEL_POST: i32 = -2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mix_Music {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mix_Chunk {
    pub allocated: libc::c_int,
    pub abuf: *mut u8,
    pub alen: u32,
    pub volume: u8,
}

pub type Mix_Music = _Mix_Music;

extern "C" {
    pub fn Mix_OpenAudio(
        frequency: libc::c_int,
        format: u16,
        channels: libc::c_int,
        chunksize: libc::c_int,
    ) -> libc::c_int;
}

pub type Mix_EffectFunc_t = ::core::option::Option<
    unsafe extern "C" fn(
        chan: libc::c_int,
        stream: *mut libc::c_void,
        len: libc::c_int,
        udata: *mut libc::c_void,
    ),
>;
pub type Mix_EffectDone_t =
    ::core::option::Option<unsafe extern "C" fn(chan: libc::c_int, udata: *mut libc::c_void)>;

extern "C" {
    // SDL mixer missing wrappers in Rust-SDL
    pub fn Mix_RegisterEffect(
        chan: libc::c_int,
        f: Mix_EffectFunc_t,
        d: Mix_EffectDone_t,
        arg: *mut libc::c_void,
    ) -> libc::c_int;
}

extern "C" {
    pub fn Mix_SetMusicCMD(command: *const libc::c_char) -> libc::c_int;
}

extern "C" {
    pub fn Mix_QuerySpec(
        frequency: *mut libc::c_int,
        format: *mut u16,
        channels: *mut libc::c_int,
    ) -> libc::c_int;
}

extern "C" {
    pub fn Mix_PlayMusic(music: *mut Mix_Music, loops: libc::c_int) -> libc::c_int;
}

extern "C" {
    pub fn Mix_LoadMUS(file: *const libc::c_char) -> *mut Mix_Music;
}

use sdl2::mixer::*;
use sdl2::pixels::*;
use sdl2::render::*;
use sdl2::video::*;
use sdl2::Sdl;
use sdl2::TimerSubsystem;

use serde::{Deserialize, Serialize};

use std::env;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::mem;
use std::process::exit;
use std::ptr;
use std::slice;
use std::str;
use std::time::Duration;

/////////////////////////////
/*
https://doom.fandom.com/wiki/Doom_source_code

AM_*
Automap code
D_*
Initialisation/general code
F_*
"Finale" (end of game) and "screen melt" code.
G_*
Main game loop/control
HU_*
Heads-up display
I_*
System-specific code
M_*
Miscellaneous (includes the menu)
P_*
Game logic/behaviour
R_*
Rendering engine
S_*
Sound code
ST_*
Status bar
V_*
General graphic rendering
WI_*
End-of level "intermission" screen
W_*
WAD file loading
Z_*
Zone memory allocation system
*/
pub mod config;
pub mod doom;
pub mod doomdata;
pub mod doomdef;
pub mod doomfeatures;
pub mod doomkeys;
pub mod doomstat;
pub mod doomtype;
pub mod dstrings;
pub mod dummy;
pub mod gusconf;
pub mod icon;
pub mod info;
pub mod memio;
pub mod mus2mid;
pub mod sha1;
pub mod sounds;
pub mod statdump;
pub mod tables;

pub mod am;
pub mod d;
pub mod deh;
pub mod f;
pub mod g;
pub mod hu;
pub mod i;
pub mod m;
pub mod net;
pub mod p;
pub mod r;
pub mod s;
pub mod st;
pub mod v;
pub mod w;
pub mod wi;
pub mod z;

/////////////////////////////

pub use crate::config::*;
pub use crate::doom::*;
pub use crate::doomdata::*;
pub use crate::doomdef::*;
pub use crate::doomfeatures::*;
pub use crate::doomkeys::*;
pub use crate::doomstat::*;
pub use crate::doomtype::*;
pub use crate::dstrings::*;
pub use crate::dummy::*;
pub use crate::gusconf::*;
pub use crate::icon::*;
pub use crate::info::*;
pub use crate::memio::*;
pub use crate::mus2mid::*;
pub use crate::sha1::*;
pub use crate::sounds::*;
pub use crate::statdump::*;
pub use crate::tables::*;

pub use crate::am::*;
pub use crate::d::d_main::*;
pub use crate::d::*;
pub use crate::deh::*;
pub use crate::f::*;
pub use crate::g::*;
pub use crate::hu::*;
pub use crate::i::*;
pub use crate::icon::*;
pub use crate::info::*;
pub use crate::m::m_argv::*;
pub use crate::m::*;
pub use crate::net::*;
pub use crate::p::*;
pub use crate::r::*;
pub use crate::s::*;
pub use crate::st::*;
pub use crate::v::*;
pub use crate::w::*;
pub use crate::wi::*;
pub use crate::z::*;

/////////////////////////////

pub struct modules<'a> {
    am: am::am,
    d: d::d<'a>,
    deh: deh::deh,
    f: f::f<'a>,
    g: g::g,
    hu: hu::hu,
    i: i::i<'a>,
    m: m::m<'a>,
    net: net::net,
    p: p::p,
    r: r::r<'a>,
    s: s::s<'a>,
    st: st::st,
    v: v::v,
    w: w::w<'a>,
    wi: wi::wi,
    z: z::z,
    doomstat: doomstat::doomstat<'a>,
    statdump: statdump::statdump,
    sounds: sounds::sounds<'a>,
}

impl<'a> modules<'a> {
    pub fn new() -> Self {
        Self {
            am: am::am::new(),
            d: d::d::new(),
            deh: deh::deh::new(),
            f: f::f::new(),
            g: g::g::new(),
            hu: hu::hu::new(),
            i: i::i::new(),
            m: m::m::new(),
            net: net::net::new(),
            p: p::p::new(),
            r: r::r::new(),
            s: s::s::new(),
            st: st::st::new(),
            v: v::v::new(),
            w: w::w::new(),
            wi: wi::wi::new(),
            z: z::z::new(),
            doomstat: doomstat::doomstat::new(),
            statdump: statdump::statdump::new(),
            sounds: sounds::sounds::new(),
        }
    }
}

/////////////////////////////
// i_main.c
/////////////////////////////
//
// DESCRIPTION:
//	Main program, simply calls D_DoomMain high level loop.
//

fn main() {
    println!("main");

    let mut doom: modules = modules::new();

    // save arguments

    doom.m.myargv = env::args().collect();
    doom.m.myargc = doom.m.myargv.len();

    M_FindResponseFile(&mut doom);

    // start doom
    println!("Starting D_DoomMain");

    D_DoomMain(&mut doom);
}
