#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// s_sound.h
/////////////////////////////
//
// DESCRIPTION:
//	The not so system specific sound interface.
//

/////////////////////////////
// s_sound.c
/////////////////////////////
//
// DESCRIPTION:  none
//

// when to clip out sounds
// Does not fit the large outdoor areas.

pub const S_CLIPPING_DIST: i32 = 1200 * FRACUNIT;

// Distance tp origin when sounds should be maxed out.
// This should relate to movement clipping resolution
// (see BLOCKMAP handling).
// In the source code release: (160*FRACUNIT).  Changed back to the
// Vanilla value of 200 (why was this changed?)

pub const S_CLOSE_DIST: i32 = 200 * FRACUNIT;

// The range over which sound attenuates

pub const S_ATTENUATOR: i32 = (S_CLIPPING_DIST - S_CLOSE_DIST) >> FRACBITS;

// Stereo separation

pub const S_STEREO_SWING: i32 = 96 * FRACUNIT;

pub const NORM_PITCH: u8 = 128;
pub const NORM_PRIORITY: u8 = 64;
pub const NORM_SEP: u8 = 128;

pub struct channel_t<'a> {
    // sound information (if null, channel avail.)
    pub sfxinfo: *mut sfxinfo_t<'a>,

    // origin of sound
    pub origin: *mut mobj_t,

    // handle of the sound being played
    pub handle: i32,
}

//
// Initializes sound stuff, including volume
// Sets channels, SFX and music volume,
//  allocates channel buffer, sets S_sfx lookup.
//

pub fn S_Init(sfxVolume: i32, musicVolume: i32) {
    println!("S_Init");
}

pub fn S_Shutdown() {
    println!("S_Shutdown");
}

pub fn S_StopChannel(cnum: i32) {
    println!("S_StopChannel");
}

//
// Per level startup code.
// Kills playing sounds at start of level,
//  determines music if any, changes music.
//

pub fn S_Start() {
    println!("S_Start");
}

pub fn S_StopSound(origin: *mut mobj_t) {
    println!("S_StopSound");
}

//
// S_GetChannel :
//   If none available, return -1.  Otherwise channel #.
//

pub fn S_GetChannel(origin: *mut mobj_t, sfxinfo: *mut sfxinfo_t) -> i32 {
    println!("S_GetChannel");

    return 0;
}

//
// Changes volume and stereo-separation variables
//  from the norm of a sound effect to be played.
// If the sound is not audible, returns a 0.
// Otherwise, modifies parameters and returns 1.
//

pub fn S_AdjustSoundParams(
    listener: *mut mobj_t,
    source: *mut mobj_t,
    vol: *mut i32,
    sep: *mut i32,
) -> i32 {
    println!("S_AdjustSoundParams");

    return 0;
}

pub fn S_StartSound(origin_p: *mut libc::c_void, sfx_id: i32) {
    println!("S_StartSound");
}

//
// Stop and resume music, during game PAUSE.
//

pub fn S_PauseSound() {
    println!("S_PauseSound");
}

pub fn S_ResumeSound() {
    println!("S_ResumeSound");
}

//
// Updates music & sounds
//

pub fn S_UpdateSounds(listener: *mut mobj_t) {
    println!("S_UpdateSounds");
}

pub fn S_SetMusicVolume(volume: i32) {
    println!("S_SetMusicVolume");
}

pub fn S_SetSfxVolume(volume: i32) {
    println!("S_SetSfxVolume");
}

//
// Starts some music with the music id found in sounds.h.
//

pub fn S_StartMusic(doom: &mut modules, m_id: i32) {
    println!("S_StartMusic");

    S_ChangeMusic(doom, m_id, 0);
}

pub fn S_ChangeMusic(doom: &mut modules, mut musicnum: i32, looping: i32) {
    println!("S_ChangeMusic");

    let mut music: *mut musicinfo_t = ptr::null_mut();
    let namebuf: &str;
    let handle: *mut libc::c_void;

    // The Doom IWAD file has two versions of the intro music: d_intro
    // and d_introa.  The latter is used for OPL playback.

    if musicnum == musicenum_t::mus_intro as i32
        && (doom.i.snd_musicdevice == snddevice_t::SNDDEVICE_ADLIB as i32
            || doom.i.snd_musicdevice == snddevice_t::SNDDEVICE_SB as i32)
    {
        musicnum = musicenum_t::mus_introa as i32;
    }

    if musicnum <= musicenum_t::mus_None as i32 || musicnum >= musicenum_t::NUMMUSIC as i32 {
        println!("Bad music number {}", musicnum);
        I_Error();
    } else {
        music = &mut doom.sounds.S_music[musicnum as usize];
    }

    if doom.s.mus_playing == music {
        return;
    }

    // shutdown old music
    S_StopMusic();

    unsafe {
        // get lumpnum if neccessary
        if (*music).lumpnum == 0 {
            let namebuf = format!("d_{}", DEH_String((*music).name));
            //namebuf = "d_introa";
            (*music).lumpnum = W_GetNumForName(doom, &namebuf);
        }

        (*music).data = W_CacheLumpNum(doom, (*music).lumpnum, PURGE::PU_STATIC as i32);
        let len = W_LumpLength(doom, (*music).lumpnum as u32);
        handle = I_RegisterSong(doom, (*music).data, len);
        (*music).handle = handle;
        I_PlaySong(doom, handle, looping);

        doom.s.mus_playing = music;
    }
}

pub fn S_MusicPlaying() -> bool {
    println!("S_MusicPlaying");

    return false;
}

pub fn S_StopMusic() {
    println!("S_StopMusic");
}
