#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// i_sound.h
/////////////////////////////
//
// DESCRIPTION:
//	The not so system specific sound interface.
//

//
// SoundFX struct.
//
pub type sfxinfo_t<'a> = sfxinfo_struct<'a>;

#[repr(C)]
pub struct sfxinfo_struct<'a> {
    // tag name, used for hexen.
    pub tagname: &'a str,

    // lump name.  If we are running with use_sfx_prefix=true, a
    // 'DS' (or 'DP' for PC speaker sounds) is prepended to this.
    pub name: &'a str,

    // Sfx priority
    pub priority: i32,

    // referenced sound if a link
    pub link: *const sfxinfo_t<'a>,

    // pitch if a link
    pub pitch: i32,

    // volume if a link
    pub volume: i32,

    // this is checked every second to see if sound
    // can be thrown out (if 0, then decrement, if -1,
    // then throw out, if > 0, then it is in use)
    pub usefulness: i32,

    // lump number of sfx
    pub lumpnum: i32,

    // Maximum number of channels that the sound can be played on
    // (Heretic)
    pub numchannels: i32,

    // data used by the low level code
    pub driver_data: *mut libc::c_void,
}

//
// MusicInfo struct.
//
#[repr(C)]
pub struct musicinfo_t<'a> {
    // up to 6-character name
    pub name: &'a str,

    // lump number of music
    pub lumpnum: i32,

    // music data
    pub data: *mut libc::c_void,

    // music handle once registered
    pub handle: *mut libc::c_void,
}

#[repr(C)]
pub enum snddevice_t {
    SNDDEVICE_NONE = 0,
    SNDDEVICE_PCSPEAKER = 1,
    SNDDEVICE_ADLIB = 2,
    SNDDEVICE_SB = 3,
    SNDDEVICE_PAS = 4,
    SNDDEVICE_GUS = 5,
    SNDDEVICE_WAVEBLASTER = 6,
    SNDDEVICE_SOUNDCANVAS = 7,
    SNDDEVICE_GENMIDI = 8,
    SNDDEVICE_AWE32 = 9,
    SNDDEVICE_CD = 10,
}

// Interface for sound modules
#[repr(C)]
pub struct sound_module_t {
    // List of sound devices that this sound module is used for.
    pub sound_devices: *mut snddevice_t,
    pub num_sound_devices: i32,

    // Initialise sound module
    // Returns true if successfully initialised
    pub Init: fn(doom: &mut modules, use_sfx_prefix: bool) -> bool,

    // Shutdown sound module
    pub Shutdown: fn(),

    // Returns the lump index of the given sound.
    pub GetSfxLumpNum: fn(sfxinfo: *mut sfxinfo_t) -> i32,

    // Called periodically to update the subsystem.
    pub Update: fn(),

    // Update the sound settings on the given channel.
    pub UpdateSoundParams: fn(channel: i32, vol: i32, sep: i32),

    // Start a sound on a given channel.  Returns the channel id
    // or -1 on failure.
    pub StartSound: fn(sfxinfo: *mut sfxinfo_t, channel: i32, vol: i32, sep: i32) -> i32,

    // Stop the sound playing on the given channel.
    pub StopSound: fn(channel: i32),

    // Query if a sound is playing on the given channel
    pub SoundIsPlaying: fn(channel: i32) -> bool,

    // Called on startup to precache sound effects (if necessary)
    pub CacheSounds: fn(sounds: *mut sfxinfo_t, num_sounds: i32),
}

// Interface for music modules
#[repr(C)]
pub struct music_module_t {
    // List of sound devices that this music module is used for.
    pub sound_devices: *const snddevice_t,
    pub num_sound_devices: i32,

    // Initialise the music subsystem
    pub Init: fn(doom: &mut modules) -> bool,

    // Shutdown the music subsystem
    pub Shutdown: fn(),

    // Set music volume - range 0-127
    pub SetMusicVolume: fn(volume: i32),

    // Pause music
    pub PauseMusic: fn(),

    // Un-pause music
    pub ResumeMusic: fn(),

    // Register a song handle from data
    // Returns a handle that can be used to play the song
    pub RegisterSong:
        fn(doom: &mut modules, data: *mut libc::c_void, len: i32) -> *mut libc::c_void,

    // Un-register (free) song data
    pub UnRegisterSong: fn(handle: *mut libc::c_void),

    // Play the song
    pub PlaySong: fn(doom: &mut modules, handle: *mut libc::c_void, looping: i32),

    // Stop playing the current song.
    pub StopSong: fn(),

    // Query if music is playing.
    pub MusicIsPlaying: fn() -> bool,

    // Invoked periodically to poll.
    pub Poll: fn(),
}

/////////////////////////////
// i_sound.c
/////////////////////////////
//
// DESCRIPTION:  none
//

//
// Initializes sound stuff, including volume
// Sets channels, SFX and music volume,
//  allocates channel buffer, sets S_sfx lookup.
//

// Check if a sound device is in the given list of devices

pub fn SndDeviceInList(device: i32, list: *mut snddevice_t, len: i32) -> bool {
    println!("SndDeviceInList");

    return false;
}

// Find and initialize a sound_module_t appropriate for the setting
// in snd_sfxdevice.

pub fn InitSfxModule(doom: &mut modules, use_sfx_prefix: bool) {
    println!("InitSfxModule");

    let mut i: usize = 0;

    doom.i.sound_module = ptr::null_mut();

    unsafe {
        while doom.i.sound_modules[i] != ptr::null_mut() {
            // Is the sfx device in the list of devices supported by
            // this module?

            if SndDeviceInList(
                doom.i.snd_sfxdevice,
                (*doom.i.sound_modules[i]).sound_devices,
                (*doom.i.sound_modules[i]).num_sound_devices,
            ) {
                // Initialize the module

                if ((*(doom.i.sound_modules[i])).Init)(doom, use_sfx_prefix) {
                    doom.i.sound_module = doom.i.sound_modules[i];
                    return;
                }
            }
            i += 1;
        }
    }
}

// Initialize music according to snd_musicdevice.

pub fn InitMusicModule(doom: &mut modules) {
    println!("InitMusicModule");

    doom.i.music_module = &music_sdl_module;

    return;

    /*for (i=0; music_modules[i] != NULL; ++i)
    {
        // Is the music device in the list of devices supported
        // by this module?

        if (SndDeviceInList(snd_musicdevice,
                            music_modules[i]->sound_devices,
                            music_modules[i]->num_sound_devices))
        {
            // Initialize the module

            if (music_modules[i]->Init())
            {
                music_module = music_modules[i];
                return;
            }
        }
    }*/
}

//
// Initializes sound stuff, including volume
// Sets channels, SFX and music volume,
//  allocates channel buffer, sets S_sfx lookup.
//

pub fn I_InitSound(doom: &mut modules, use_sfx_prefix: bool) {
    println!("I_InitSound");

    let nosound: bool;
    let nosfx: bool;
    let nomusic: bool;

    //
    // @vanilla
    //
    // Disable all sound output.
    //

    nosound = M_CheckParm(doom, "-nosound") > 0;

    //
    // @vanilla
    //
    // Disable sound effects.
    //

    nosfx = M_CheckParm(doom, "-nosfx") > 0;

    //
    // @vanilla
    //
    // Disable music.
    //

    nomusic = M_CheckParm(doom, "-nomusic") > 0;

    // Initialize the sound and music subsystems.

    if !nosound && !doom.i.screensaver_mode {
        // This is kind of a hack. If native MIDI is enabled, set up
        // the TIMIDITY_CFG environment variable here before SDL_mixer
        // is opened.

        if !nomusic
            && (doom.i.snd_musicdevice == snddevice_t::SNDDEVICE_GENMIDI as i32
                || doom.i.snd_musicdevice == snddevice_t::SNDDEVICE_GUS as i32)
        {
            I_InitTimidityConfig();
        }

        if !nosfx {
            InitSfxModule(doom, use_sfx_prefix);
        }

        if !nomusic {
            InitMusicModule(doom);
        }
    }
}

pub fn I_ShutdownSound() {
    println!("I_ShutdownSound");
}

pub fn I_GetSfxLumpNum(sfxinfo: *mut sfxinfo_t) -> i32 {
    println!("I_GetSfxLumpNum");

    return 0;
}

pub fn I_UpdateSound() {
    println!("I_UpdateSound");
}

pub fn CheckVolumeSeparation(vol: *mut i32, sep: *mut i32) {
    println!("CheckVolumeSeparation");
}

pub fn I_UpdateSoundParams(channel: i32, vol: i32, sep: i32) {
    println!("I_UpdateSoundParams");
}

pub fn I_StartSound(sfxinfo: *mut sfxinfo_t, channel: i32, vol: i32, sep: i32) -> i32 {
    println!("I_StartSound");

    return 0;
}

pub fn I_StopSound(channel: i32) {
    println!("I_StopSound");
}

pub fn I_SoundIsPlaying(channel: i32) -> bool {
    println!("I_SoundIsPlaying");

    return false;
}

pub fn I_PrecacheSounds(sounds: *mut sfxinfo_t, num_sounds: i32) {
    println!("I_PrecacheSounds");
}

pub fn I_InitMusic(doom: &mut modules) {
    println!("I_InitMusic");

    if doom.i.music_module != ptr::null_mut() {
        unsafe {
            ((*doom.i.music_module).Init)(doom);
        }
    }
}

pub fn I_ShutdownMusic() {
    println!("I_ShutdownMusic");
}

pub fn I_SetMusicVolume(volume: i32) {
    println!("I_SetMusicVolume");
}

pub fn I_PauseSong() {
    println!("I_PauseSong");
}

pub fn I_ResumeSong() {
    println!("I_ResumeSong");
}

pub fn I_RegisterSong(doom: &mut modules, data: *mut libc::c_void, len: i32) -> *mut libc::c_void {
    println!("I_RegisterSong");

    if doom.i.music_module != ptr::null_mut() {
        unsafe {
            return ((*doom.i.music_module).RegisterSong)(doom, data, len);
        }
    } else {
        return ptr::null_mut();
    }
}

pub fn I_UnRegisterSong(handle: *mut libc::c_void) {
    println!("I_UnRegisterSong");
}

pub fn I_PlaySong(doom: &mut modules, handle: *mut libc::c_void, looping: i32) {
    println!("I_PlaySong");

    if doom.i.music_module != ptr::null_mut() {
        unsafe {
            ((*doom.i.music_module).PlaySong)(doom, handle, looping);
        }
    }
}

pub fn I_StopSong() {
    println!("I_StopSong");
}

pub fn I_MusicIsPlaying() -> bool {
    println!("I_MusicIsPlaying");

    return false;
}

pub fn I_BindSoundVariables() {
    println!("I_BindSoundVariables");
}
