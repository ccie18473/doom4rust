#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod i_cdmus;
pub mod i_endoom;
pub mod i_input;
pub mod i_joystick;
pub mod i_main;
pub mod i_scale;
pub mod i_sdlmusic;
pub mod i_sdlsound;
pub mod i_sound;
pub mod i_swap;
pub mod i_system;
pub mod i_timer;
pub mod i_video;

pub use i_cdmus::*;
pub use i_endoom::*;
pub use i_input::*;
pub use i_joystick::*;
pub use i_main::*;
pub use i_scale::*;
pub use i_sdlmusic::*;
pub use i_sdlsound::*;
pub use i_sound::*;
pub use i_swap::*;
pub use i_system::*;
pub use i_timer::*;
pub use i_video::*;

use crate::*;

/////////////////////////////
// I_* System-specific code
/////////////////////////////

pub struct i<'a> {
    /////////////////////////////
    // i_sdlmusic.c
    /////////////////////////////
    pub music_initialized: bool,
    // If this is true, this module initialized SDL sound and has the
    // responsibility to shut it down
    pub sdl_was_initialized: bool,

    pub musicpaused: bool,
    pub current_music_volume: i32,
    pub timidity_cfg_path: &'a str,
    pub temp_timidity_cfg: &'a str,
    // If true, we are playing a substitute digital track rather than in-WAD
    // MIDI/MUS track, and file_metadata contains loop metadata.
    pub playing_substitute: bool,
    pub file_metadata: file_metadata_t,
    // Currently playing music track.
    pub current_track_music: *mut Mix_Music,
    // If true, the currently playing track is being played on loop.
    pub current_track_loop: i32,
    /////////////////////////////
    // i_sound.c
    /////////////////////////////
    // Sound sample rate to use for digital output (Hz)
    pub snd_samplerate: i32,
    // Maximum number of bytes to dedicate to allocated sound effects.
    // (Default: 64MB)
    pub snd_cachesize: i32,
    // Config variable that controls the sound buffer size.
    // We default to 28ms (1000 / 35fps = 1 buffer per tic).
    pub snd_maxslicetime_ms: i32,
    // External command to invoke to play back music.
    pub snd_musiccmd: &'a str,
    // Low-level sound and music modules we are using
    pub sound_module: *const sound_module_t,
    pub music_module: *const music_module_t,

    pub snd_musicdevice: i32,
    pub snd_sfxdevice: i32,
    // DOS-specific options: These are unused but should be maintained
    // so that the config file can be shared between chocolate
    // doom and doom.exe
    pub snd_sbport: i32,
    pub snd_sbirq: i32,
    pub snd_sbdma: i32,
    pub snd_mport: i32,
    // Compiled-in sound modules:
    pub sound_modules: [*mut sound_module_t; 1],
    // Compiled-in music modules:
    pub music_modules: [*mut music_module_t; 1],
    /////////////////////////////
    // i_system.c
    /////////////////////////////
    pub exit_funcs: *mut atexit_listentry_t,
    pub already_quitting: bool,
    /////////////////////////////
    // i_timer.c
    /////////////////////////////
    //
    // I_GetTime
    // returns time in 1/35th second tics
    //
    pub basetime: u32,
    /////////////////////////////
    // i_video.c
    /////////////////////////////
    pub fb_SDL: *mut u32,
    pub sdl_context: Sdl,
    pub timer: TimerSubsystem,
    pub renderer: Canvas<Window>,
    pub s_Fb: FB_ScreenInfo,
    pub fb_scaling: i32,
    //int usemouse = 0;
    pub colors: [color; 256],
    // The screen buffer; this is modified to draw things to the screen
    pub I_VideoBuffer: *mut u8,

    // If true, game is running as a screensaver
    pub screensaver_mode: bool,

    // Flag indicating whether the screen is currently visible:
    // when the screen isnt visible, don't render the screen
    pub screenvisible: bool,
    // Mouse acceleration
    //
    // This emulates some of the behavior of DOS mouse drivers by increasing
    // the speed when the mouse is moved fast.
    //
    // The mouse input values are input directly to the game, but when
    // the values exceed the value of mouse_threshold, they are multiplied
    // by mouse_acceleration to increase the speed.

    //float mouse_acceleration = 2.0;
    //int mouse_threshold = 10;

    // Gamma correction level to use
    pub usegamma: i32,
    // Palette converted to RGB565

    //static uint16_t rgb565_palette[256];
}

impl<'a> i<'a> {
    pub fn new() -> Self {
        let title: String = "Doom4Rust".to_string();
        let sdl_context = sdl2::init().unwrap();
        let timer = sdl_context.timer().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(&title, 960, 600)
            .resizable()
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        let renderer = window
            .into_canvas()
            .accelerated()
            .target_texture()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();
        Self {
            /////////////////////////////
            // i_sdlmusic.c
            /////////////////////////////
            music_initialized: false,
            sdl_was_initialized: false,
            musicpaused: false,
            current_music_volume: 0,
            timidity_cfg_path: "",
            temp_timidity_cfg: "",
            playing_substitute: false,
            file_metadata: file_metadata_t::new(),
            current_track_music: ptr::null_mut(),
            current_track_loop: 0,
            /////////////////////////////
            // i_sound.c
            /////////////////////////////
            snd_samplerate: 44100,
            snd_cachesize: 64 * 1024 * 1024,
            snd_maxslicetime_ms: 28,
            snd_musiccmd: "",
            sound_module: ptr::null_mut(),
            music_module: ptr::null_mut(),
            snd_musicdevice: snddevice_t::SNDDEVICE_SB as i32,
            snd_sfxdevice: snddevice_t::SNDDEVICE_SB as i32,
            snd_sbport: 0,
            snd_sbirq: 0,
            snd_sbdma: 0,
            snd_mport: 0,
            sound_modules: [ptr::null_mut(); 1],
            music_modules: [ptr::null_mut(); 1],
            /////////////////////////////
            // i_system.c
            /////////////////////////////
            exit_funcs: ptr::null_mut(),
            already_quitting: false,
            /////////////////////////////
            // i_video.c
            /////////////////////////////
            fb_SDL: ptr::null_mut(),
            sdl_context,
            timer,
            renderer,
            s_Fb: FB_ScreenInfo::new(),
            fb_scaling: 1,
            colors: [color {
                b: 8,
                g: 8,
                r: 8,
                a: 8,
            }; 256],
            I_VideoBuffer: ptr::null_mut(),
            screensaver_mode: false,
            screenvisible: false,
            usegamma: 0,
            /////////////////////////////
            // i_timer.c
            /////////////////////////////
            basetime: 0,
        }
    }
}
