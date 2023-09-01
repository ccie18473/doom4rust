#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod d_englsh;
pub mod d_event;
pub mod d_items;
pub mod d_iwad;
pub mod d_loop;
pub mod d_main;
pub mod d_mode;
pub mod d_net;
pub mod d_player;
pub mod d_textur;
pub mod d_think;
pub mod d_ticcmd;

pub use d_englsh::*;
pub use d_event::*;
pub use d_items::*;
pub use d_iwad::*;
pub use d_loop::*;
pub use d_main::*;
pub use d_mode::*;
pub use d_net::*;
pub use d_player::*;
pub use d_textur::*;
pub use d_think::*;
pub use d_ticcmd::*;

use crate::*;

/////////////////////////////
// D_* Initialisation/general code
/////////////////////////////
pub struct d<'a> {
    /////////////////////////////
    // d_event.c
    /////////////////////////////
    pub events: [event_t; MAXEVENTS],
    pub eventhead: i32,
    pub eventtail: i32,
    /////////////////////////////
    // d_iwad.c
    /////////////////////////////
    pub iwad_dirs_built: bool,
    pub iwad_dirs: [&'a str; MAX_IWAD_DIRS],
    pub num_iwad_dirs: i32,
    /////////////////////////////
    // d_loop.c
    /////////////////////////////
    pub ticdata: [ticcmd_set_t; BACKUPTICS as usize],
    pub maketic: i32,
    pub recvtic: i32,
    pub gametic: i32,
    pub singletics: bool,
    pub localplayer: i32,
    pub skiptics: i32,
    pub ticdup: i32,
    pub offsetms: fixed_t,
    pub new_sync: bool,                        // = true;
    pub loop_interface: *mut loop_interface_t, // = NULL;
    pub local_playeringame: [bool; NET_MAXPLAYERS],
    pub player_class: i32,
    pub lasttime: i32,
    pub frameon: i32,
    pub frameskip: [i32; 4],
    pub oldnettics: i32,
    pub oldentertics: i32,
    /////////////////////////////
    // d_main.c
    /////////////////////////////
    pub savegamedir: &'a str, // Location where savegames are stored
    pub iwadfile: &'a str,    // location of IWAD and WAD files
    pub devparm: bool,        // started game with -devparm
    pub nomonsters: bool,     // checkparm of -nomonsters
    pub respawnparm: bool,    // checkparm of -respawn
    pub fastparm: bool,       // checkparm of -fast

    pub startskill: skill_t,
    pub startepisode: i32,
    pub startmap: i32,
    pub autostart: bool,
    pub startloadgame: i32,

    pub advancedemo: bool,

    pub storedemo: bool, // Store demo, do not accept any inputs

    pub bfgedition: bool, // "BFG Edition" version of doom2.wad does not include TITLEPIC.

    pub main_loop_started: bool, // If true, the main game loop has started.

    pub wadfile: &'a str, // primary wad file
    pub mapdir: &'a str,  // directory of development maps

    pub show_endoom: i32,

    // wipegamestate can be set to -1 to force a wipe on the next draw
    pub wipegamestate: gamestate_t,

    pub title: &'a str,
    //
    //  DEMO LOOP
    //
    pub demosequence: i32,
    pub pagetic: i32,
    pub pagename: &'a str,

    pub viewactivestate: bool,
    pub menuactivestate: bool,
    pub inhelpscreensstate: bool,
    pub fullscreen: bool,
    pub borderdrawcount: i32,
    /////////////////////////////
    // d_net.c
    /////////////////////////////
    pub netcmds: *mut ticcmd_t,

    pub doom_loop_interface: loop_interface_t,
}

impl<'a> d<'a> {
    pub fn new() -> Self {
        Self {
            /////////////////////////////
            // d_event.c
            /////////////////////////////
            events: [event_t::new(); MAXEVENTS],
            eventhead: 0,
            eventtail: 0,
            /////////////////////////////
            // d_iwad.c
            /////////////////////////////
            iwad_dirs_built: false,
            iwad_dirs: [""; MAX_IWAD_DIRS],
            num_iwad_dirs: 0,
            /////////////////////////////
            // d_loop.c
            /////////////////////////////
            ticdata: [ticcmd_set_t::new(); BACKUPTICS as usize],
            maketic: 0,
            recvtic: 0,
            gametic: 0,
            singletics: false,
            localplayer: 0,
            skiptics: 0,
            ticdup: 1,
            offsetms: 0,
            new_sync: true,
            loop_interface: ptr::null_mut(),
            local_playeringame: [false; NET_MAXPLAYERS],
            player_class: 0,
            lasttime: 0,
            frameon: 0,
            frameskip: [0; 4],
            oldnettics: 0,
            oldentertics: 0,
            /////////////////////////////
            // d_main.c
            /////////////////////////////
            savegamedir: "",
            iwadfile: "",
            devparm: false,
            nomonsters: false,
            respawnparm: false,
            fastparm: false,

            startskill: skill_t::sk_noitems,
            startepisode: 0,
            startmap: 0,
            autostart: false,
            startloadgame: 0,

            advancedemo: false,

            storedemo: false,

            bfgedition: false,

            main_loop_started: false,

            wadfile: "",
            mapdir: "",

            show_endoom: 0,

            wipegamestate: gamestate_t::GS_DEMOSCREEN,

            title: "",

            demosequence: 0,
            pagetic: 0,
            pagename: "",

            viewactivestate: false,
            menuactivestate: false,
            inhelpscreensstate: false,
            fullscreen: false,
            borderdrawcount: 0,
            /////////////////////////////
            // d_net.c
            /////////////////////////////
            netcmds: ptr::null_mut(),

            doom_loop_interface: loop_interface_t {
                ProcessEvents: D_ProcessEvents,
                BuildTiccmd: G_BuildTiccmd,
                RunTic,
                RunMenu: M_Ticker,
            },
        }
    }
}
