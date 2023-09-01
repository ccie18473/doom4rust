#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// i_sound.c
/////////////////////////////
//
// DESCRIPTION:
//	System interface for sound.
//

pub const LOW_PASS_FILTER: u8 = 1;
//#define DEBUG_DUMP_WAVS
pub const NUM_CHANNELS: u8 = 16;

pub type allocated_sound_t<'a> = allocated_sound_s<'a>;

pub struct allocated_sound_s<'a> {
    pub sfxinfo: *mut sfxinfo_t<'a>,
    pub chunk: Mix_Chunk,
    pub use_count: i32,
    pub prev: *mut allocated_sound_t<'a>,
    pub next: *mut allocated_sound_t<'a>,
}

// Hook a sound into the linked list at the head.

pub fn AllocatedSoundLink(snd: *mut allocated_sound_t) {
    println!("AllocatedSoundLink");
}

// Unlink a sound from the linked list.

pub fn AllocatedSoundUnlink(snd: *mut allocated_sound_t) {
    println!("AllocatedSoundUnlink");
}

pub fn FreeAllocatedSound(snd: *mut allocated_sound_t) {
    println!("FreeAllocatedSound");
}

// Search from the tail backwards along the allocated sounds list, find
// and free a sound that is not in use, to free up memory.  Return true
// for success.

pub fn FindAndFreeSound() -> bool {
    println!("FindAndFreeSound");

    return false;
}

// Enforce SFX cache size limit.  We are just about to allocate "len"
// bytes on the heap for a new sound effect, so free up some space
// so that we keep allocated_sounds_size < snd_cachesize

pub fn ReserveCacheSpace(len: usize) {
    println!("ReserveCacheSpace");
}

// Allocate a block for a new sound effect.

pub fn AllocateSound(sfxinfo: *mut sfxinfo_t, len: usize) -> *mut Mix_Chunk {
    println!("AllocateSound");

    return ptr::null_mut();
}

// Lock a sound, to indicate that it may not be freed.

pub fn LockAllocatedSound(snd: *mut allocated_sound_t) {
    println!("LockAllocatedSound");
}

// Unlock a sound to indicate that it may now be freed.

pub fn UnlockAllocatedSound(snd: *mut allocated_sound_t) {
    println!("UnlockAllocatedSound");
}

// When a sound stops, check if it is still playing.  If it is not,
// we can mark the sound data as CACHE to be freed back for other
// means.

pub fn ReleaseSoundOnChannel(channel: i32) {
    println!("ReleaseSoundOnChannel");
}

pub fn ConvertibleRatio(freq1: i32, freq2: i32) -> bool {
    println!("ConvertibleRatio");

    return false;
}

// Generic sound expansion function for any sample rate.
// Returns number of clipped samples (always 0).

pub fn ExpandSoundData_SDL(
    sfxinfo: *mut sfxinfo_t,
    data: *mut u8,
    samplerate: i32,
    length: i32,
) -> bool {
    println!("ExpandSoundData_SDL");

    return false;
}

// Load and convert a sound effect
// Returns true if successful

pub fn CacheSFX(sfxinfo: *mut sfxinfo_t) -> bool {
    println!("CacheSFX");

    return false;
}

pub fn GetSfxLumpName(sfx: *mut sfxinfo_t, buf: *mut u8, buf_len: usize) {
    println!("GetSfxLumpName");
}

// Preload all the sound effects - stops nasty ingame freezes

pub fn I_SDL_PrecacheSounds(sounds: *mut sfxinfo_t, num_sounds: i32) {
    println!("I_SDL_PrecacheSounds");
}

// Load a SFX chunk into memory and ensure that it is locked.

pub fn LockSound(sfxinfo: *mut sfxinfo_t) -> bool {
    println!("LockSound");

    return false;
}

//
// Retrieve the raw data lump index
//  for a given SFX name.
//

pub fn I_SDL_GetSfxLumpNum(sfx: *mut sfxinfo_t) -> i32 {
    println!("I_SDL_GetSfxLumpNum");

    return 0;
}

pub fn I_SDL_UpdateSoundParams(handle: i32, vol: i32, sep: i32) {
    println!("I_SDL_UpdateSoundParams");
}

//
// Starting a sound means adding it
//  to the current list of active sounds
//  in the internal channels.
// As the SFX info struct contains
//  e.g. a pointer to the raw data,
//  it is ignored.
// As our sound handling does not handle
//  priority, it is ignored.
// Pitching (that is, increased speed of playback)
//  is set, but currently not used by mixing.
//

pub fn I_SDL_StartSound(sfxinfo: *mut sfxinfo_t, channel: i32, vol: i32, sep: i32) -> i32 {
    println!("I_SDL_StartSound");

    return 0;
}

pub fn I_SDL_StopSound(handle: i32) {
    println!("I_SDL_StopSound");
}

pub fn I_SDL_SoundIsPlaying(handle: i32) -> bool {
    println!("I_SDL_SoundIsPlaying");

    return false;
}

//
// Periodically called to update the sound system
//

pub fn I_SDL_UpdateSound() {
    println!("I_SDL_UpdateSound");
}

pub fn I_SDL_ShutdownSound() {
    println!("I_SDL_ShutdownSound");
}

// Calculate slice size, based on snd_maxslicetime_ms.
// The result must be a power of two.

pub fn GetSliceSize() -> i32 {
    println!("GetSliceSize");

    return 0;
}

pub fn I_SDL_InitSound(_use_sfx_prefix: bool) -> bool {
    println!("I_SDL_InitSound");

    return false;
}
