#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// i_sdlmusic.c
/////////////////////////////
//
// DESCRIPTION:
//	System interface for music.
//

pub const MAXMIDLENGTH: i32 = 96 * 1024;
pub const MID_HEADER_MAGIC: &str = "MThd";
pub const MUS_HEADER_MAGIC: &str = "MUS\x1a";

pub const FLAC_HEADER: &str = "fLaC";
pub const OGG_HEADER: &str = "OggS";

// Looping Vorbis metadata tag names. These have been defined by ZDoom
// for specifying the start and end positions for looping music tracks
// in .ogg and .flac files.
// More information is here: http://zdoom.org/wiki/Audio_loop
pub const LOOP_START_TAG: &str = "LOOP_START";
pub const LOOP_END_TAG: &str = "LOOP_END";

// FLAC metadata headers that we care about.
pub const FLAC_STREAMINFO: u8 = 0;
pub const FLAC_VORBIS_COMMENT: u8 = 4;

// Ogg metadata headers that we care about.
pub const OGG_ID_HEADER: u8 = 1;
pub const OGG_COMMENT_HEADER: u8 = 3;

// Structure for music substitution.
// We store a mapping based on SHA1 checksum -> filename of substitute music
// file to play, so that substitution occurs based on content rather than
// lump name. This has some inherent advantages:
//  * Music for Plutonia (reused from Doom 1) works automatically.
//  * If a PWAD replaces music, the replacement music is used rather than
//    the substitute music for the IWAD.
//  * If a PWAD reuses music from an IWAD (even from a different game), we get
//    the high quality version of the music automatically (neat!)

pub struct subst_music_t<'a> {
    pub hash: sha1_digest_t,
    pub filename: &'a str,
}

// Structure containing parsed metadata read from a digital music track:
pub struct file_metadata_t {
    pub valid: bool,
    pub samplerate_hz: u32,
    pub start_time: i32,
    pub end_time: i32,
}

impl file_metadata_t {
    pub fn new() -> Self {
        Self {
            valid: false,
            samplerate_hz: 0,
            start_time: 0,
            end_time: 0,
        }
    }
}

pub const subst_config_filenames: [&str; 6] = [
    "doom1-music.cfg",
    "doom2-music.cfg",
    "tnt-music.cfg",
    "heretic-music.cfg",
    "hexen-music.cfg",
    "strife-music.cfg",
];

pub const music_sdl_devices: [snddevice_t; 6] = [
    snddevice_t::SNDDEVICE_PAS,
    snddevice_t::SNDDEVICE_GUS,
    snddevice_t::SNDDEVICE_WAVEBLASTER,
    snddevice_t::SNDDEVICE_SOUNDCANVAS,
    snddevice_t::SNDDEVICE_GENMIDI,
    snddevice_t::SNDDEVICE_AWE32,
];

pub const music_sdl_module: music_module_t = music_module_t {
    sound_devices: music_sdl_devices.as_ptr(),
    num_sound_devices: music_sdl_devices.len() as i32,
    Init: I_SDL_InitMusic,
    Shutdown: I_SDL_ShutdownMusic,
    SetMusicVolume: I_SDL_SetMusicVolume,
    PauseMusic: I_SDL_PauseSong,
    ResumeMusic: I_SDL_ResumeSong,
    RegisterSong: I_SDL_RegisterSong,
    UnRegisterSong: I_SDL_UnRegisterSong,
    PlaySong: I_SDL_PlaySong,
    StopSong: I_SDL_StopSong,
    MusicIsPlaying: I_SDL_MusicIsPlaying,
    Poll: I_SDL_PollMusic,
};

// Position (in samples) that we have reached in the current track.
// This is updated by the TrackPositionCallback function.
pub static mut current_track_pos: u32 = 0;

// Given a time string (for LOOP_START/LOOP_END), parse it and return
// the time (in # samples since start of track) it represents.
pub fn ParseVorbisTime(samplerate_hz: u32, value: &str) -> u32 {
    println!("ParseVorbisTime");

    return 0;
}

// Given a vorbis comment string (eg. "LOOP_START=12345"), set fields
// in the metadata structure as appropriate.
pub fn ParseVorbisComment(metadata: *mut file_metadata_t, comment: &str) {
    println!("ParseVorbisComment");
}

// Parse a vorbis comments structure, reading from the given file.
pub fn ParseVorbisComments(metadata: *mut file_metadata_t, fs: File) {
    println!("ParseVorbisComments");
}

pub fn ParseFlacStreaminfo(metadata: *mut file_metadata_t, fs: File) {
    println!("ParseFlacStreaminfo");
}

pub fn ParseFlacFile(metadata: *mut file_metadata_t, fs: File) {
    println!("ParseFlacFile");
}

pub fn ParseOggIdHeader(metadata: *mut file_metadata_t, fs: File) {
    println!("ParseOggIdHeader");
}

pub fn ParseOggFile(metadata: *mut file_metadata_t, fs: File) {
    println!("ParseOggFile");
}

pub fn ReadLoopPoints(filename: &str, metadata: *mut file_metadata_t) {
    println!("ReadLoopPoints");
}

// Given a MUS lump, look up a substitute MUS file to play instead
// (or NULL to just use normal MIDI playback).

pub fn GetSubstituteMusicFile(data: *mut libc::c_void, data_len: usize) -> &'static str {
    println!("GetSubstituteMusicFile");

    return "";
}

// Add a substitute music file to the lookup list.

pub fn AddSubstituteMusic(subst: *mut subst_music_t) {
    println!("AddSubstituteMusic");
}

pub fn ParseHexDigit(c: u8) -> i32 {
    println!("ParseHexDigit");

    return 0;
}

pub fn GetFullPath(base_filename: &str, path: &str) -> &'static str {
    println!("GetFullPath");

    return "";
}

// Parse a line from substitute music configuration file; returns error
// message or NULL for no error.

pub fn ParseSubstituteLine(filename: &str, line: &str) -> &'static str {
    println!("ParseSubstituteLine");

    return "";
}

// Read a substitute music configuration file.

pub fn ReadSubstituteConfig(filename: &str) -> bool {
    println!("ReadSubstituteConfig");

    return false;
}

// Find substitute configs and try to load them.

pub fn LoadSubstituteConfigs() {
    println!("LoadSubstituteConfigs");
}

// Returns true if the given lump number is a music lump that should
// be included in substitute configs.
// Identifying music lumps by name is not feasible; some games (eg.
// Heretic, Hexen) don't have a common naming pattern for music lumps.

pub fn IsMusicLump(lumpnum: i32) -> bool {
    println!("IsMusicLump");

    return false;
}

// Dump an example config file containing checksums for all MIDI music
// found in the WAD directory.

pub fn DumpSubstituteConfig(filename: &str) {
    println!("DumpSubstituteConfig");
}

// If the temp_timidity_cfg config variable is set, generate a "wrapper"
// config file for Timidity to point to the actual config file. This
// is needed to inject a "dir" command so that the patches are read
// relative to the actual config file.

pub fn WriteWrapperTimidityConfig(write_path: &str) -> bool {
    println!("WriteWrapperTimidityConfig");

    return false;
}

pub fn I_InitTimidityConfig() {
    println!("I_InitTimidityConfig");
}

// Remove the temporary config file generated by I_InitTimidityConfig().

pub fn RemoveTimidityConfig() {
    println!("RemoveTimidityConfig");
}

// Shutdown music

pub fn I_SDL_ShutdownMusic() {
    println!("I_SDL_ShutdownMusic");
}

pub fn SDLIsInitialized() -> bool {
    println!("SDLIsInitialized");

    let mut freq: i32 = 0;
    let mut channels: i32 = 0;
    let mut format: u16 = 0;

    unsafe {
        return Mix_QuerySpec(&mut freq, &mut format, &mut channels) != 0;
    }
}

// Callback function that is invoked to track current track position.
pub unsafe extern "C" fn TrackPositionCallback(
    chan: libc::c_int,
    stream: *mut libc::c_void,
    len: libc::c_int,
    udata: *mut libc::c_void,
) {
    println!("TrackPositionCallback");

    // Position is doubled up twice: for 16-bit samples and for stereo.
    current_track_pos += len as u32 / 4;
}

// Initialize music subsystem
pub fn I_SDL_InitMusic(doom: &mut modules) -> bool {
    println!("I_SDL_InitMusic");

    //
    // @arg <output filename>
    //
    // Read all MIDI files from loaded WAD files, dump an example substitution
    // music config file to the specified filename and quit.
    //

    let i = M_CheckParmWithArgs(doom, "-dumpsubstconfig", 1);

    if i > 0 {
        DumpSubstituteConfig(&doom.m.myargv[i as usize + 1]);
    }

    // If SDL_mixer is not initialized, we have to initialize it
    // and have the responsibility to shut it down later on.

    if SDLIsInitialized() {
        doom.i.music_initialized = true;
    } else {
        let result =
            sdl2::mixer::init(InitFlag::MID | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG);

        match result {
            Ok(_) => (),
            Err(_) => {
                println!("Unable to set up sound.\n");
            }
        }

        if unsafe { Mix_OpenAudio(doom.i.snd_samplerate, AUDIO_S16SYS, 2, 1024) < 0 } {
            println!("Error initializing SDL_mixer");
            //SDL_QuitSubSystem(SDL_INIT_AUDIO);
        } else {
            //SDL_PauseAudio(0);

            doom.i.sdl_was_initialized = true;
            doom.i.music_initialized = true;
        }
    }

    // Once initialization is complete, the temporary Timidity config
    // file can be removed.

    RemoveTimidityConfig();

    // If snd_musiccmd is set, we need to call Mix_SetMusicCMD to
    // configure an external music playback program.
    unsafe {
        if doom.i.snd_musiccmd != "" {
            Mix_SetMusicCMD(doom.i.snd_musiccmd.to_string().as_mut_ptr() as *const i8);
        }
    }
    // Register an effect function to track the music position.
    unsafe {
        Mix_RegisterEffect(
            MIX_CHANNEL_POST,
            Some(TrackPositionCallback),
            None,
            ptr::null_mut(),
        );
    }
    // If we're in GENMIDI mode, try to load sound packs.
    if doom.i.snd_musicdevice == snddevice_t::SNDDEVICE_GENMIDI as i32 {
        LoadSubstituteConfigs();
    }

    return doom.i.music_initialized;
}

//
// SDL_mixer's native MIDI music playing does not pause properly.
// As a workaround, set the volume to 0 when paused.
//

pub fn UpdateMusicVolume() {
    println!("UpdateMusicVolume");
}

// Set music volume (0 - 127)

pub fn I_SDL_SetMusicVolume(volume: i32) {
    println!("I_SDL_SetMusicVolume");
}

// Start playing a mid

pub fn I_SDL_PlaySong(doom: &mut modules, handle: *mut libc::c_void, looping: i32) {
    println!("I_SDL_PlaySong");

    let mut loops: i32;

    if !doom.i.music_initialized {
        return;
    }

    if handle == ptr::null_mut() {
        return;
    }

    doom.i.current_track_music = handle as *mut Mix_Music;
    doom.i.current_track_loop = looping;

    if looping != 0 {
        loops = -1;
    } else {
        loops = 1;
    }

    // Don't loop when playing substitute music, as we do it
    // ourselves instead.
    if doom.i.playing_substitute && doom.i.file_metadata.valid {
        loops = 1;
        //BUG
        //SDL_LockAudio();
        unsafe {
            current_track_pos = 0; // start of track
        }
        //BUG
        //SDL_UnlockAudio();
    }
    unsafe {
        Mix_PlayMusic(doom.i.current_track_music, loops);
    }
}

pub fn I_SDL_PauseSong() {
    println!("I_SDL_PauseSong");
}

pub fn I_SDL_ResumeSong() {
    println!("I_SDL_ResumeSong");
}

pub fn I_SDL_StopSong() {
    println!("I_SDL_StopSong");
}

pub fn I_SDL_UnRegisterSong(handle: *mut libc::c_void) {
    println!("I_SDL_UnRegisterSong");
}

// Determine whether memory block is a .mid file

pub fn IsMid(mem: *mut u8, len: i32) -> bool {
    println!("IsMid");

    return false;
}

pub fn ConvertMus(musdata: *mut u8, len: i32, filename: &str) -> bool {
    println!("ConvertMus");

    return false;
}

pub fn I_SDL_RegisterSong(
    doom: &mut modules,
    data: *mut libc::c_void,
    len: i32,
) -> *mut libc::c_void {
    println!("I_SDL_RegisterSong");

    let mut filename: &str;
    let mut music: *mut Mix_Music;

    if !doom.i.music_initialized {
        return ptr::null_mut();
    }

    doom.i.playing_substitute = false;

    // See if we're substituting this MUS for a high-quality replacement.
    filename = GetSubstituteMusicFile(data, len as usize);

    if filename != "" {
        unsafe {
            music = Mix_LoadMUS(filename.to_string().as_mut_ptr() as *const i8);
        }

        if music == ptr::null_mut() {
            // Fall through and play MIDI normally, but print an error
            // message.
            println!("Failed to load substitute music file:{}", filename);
        } else {
            // Read loop point metadata from the file so that we know where
            // to loop the music.
            doom.i.playing_substitute = true;
            ReadLoopPoints(filename, &mut doom.i.file_metadata);
            return music as *mut libc::c_void;
        }
    }

    // MUS files begin with "MUS"
    // Reject anything which doesnt have this signature

    filename = M_TempFile("doom.mid");

    if IsMid(data as *mut u8, len) && len < MAXMIDLENGTH {
        M_WriteFile(filename, data, len);
    } else {
        // Assume a MUS file and try to convert

        ConvertMus(data as *mut u8, len, filename);
    }

    // Load the MIDI. In an ideal world we'd be using Mix_LoadMUS_RW()
    // by now, but Mix_SetMusicCMD() only works with Mix_LoadMUS(), so
    // we have to generate a temporary file.

    unsafe {
        music = Mix_LoadMUS(filename.to_string().as_mut_ptr() as *const i8);
    }
    if music == ptr::null_mut() {
        // Failed to load

        println!("Error loading midi");
    }

    // Remove the temporary MIDI file; however, when using an external
    // MIDI program we can't delete the file. Otherwise, the program
    // won't find the file to play. This means we leave a mess on
    // disk :(

    if doom.i.snd_musiccmd.len() == 0 {
        //BUG
        //unsafe { libc::remove(filename.to_string().as_mut_ptr() as *const i8) };
    }
    //BUG
    //unsafe { libc::free(filename.to_string().as_mut_ptr() as *mut libc::c_void) };

    return music as *mut libc::c_void;
}

// Is the song playing?
pub fn I_SDL_MusicIsPlaying() -> bool {
    println!("I_SDL_MusicIsPlaying");

    return false;
}

// Get position in substitute music track, in seconds since start of track.
pub fn GetMusicPosition() -> f64 {
    println!("GetMusicPosition");

    return 0.0;
}

pub fn RestartCurrentTrack() {
    println!("RestartCurrentTrack");
}

// Poll music position; if we have passed the loop point end position
// then we need to go back.
pub fn I_SDL_PollMusic() {
    println!("I_SDL_PollMusic");
}
