#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod g_game;

pub use g_game::*;

use crate::*;

/////////////////////////////
// G_* Main game loop/control
/////////////////////////////

pub struct g {
    /////////////////////////////
    // g_game.c
    /////////////////////////////
    // Gamestate the last time G_Ticker was called.
    pub oldgamestate: gamestate_t,

    pub gameaction: gameaction_t,
    pub gamestate: gamestate_t,
    //pub gameskill: skill_t,
    pub respawnmonsters: bool,
    pub gameepisode: i32,
    pub gamemap: i32,

    // If non-zero, exit the level after this number of minutes.
    pub timelimit: i32,

    pub paused: bool,
    pub sendpause: bool, // send a pause event next tic
    pub sendsave: bool,  // send a save event next tic
    pub usergame: bool,  // ok to save / end game

    pub timingdemo: bool, // if true, exit with report on completion
    pub nodrawers: bool,  // for comparative timing purposes
    pub starttime: i32,   // for comparative timing purposes

    pub viewactive: bool,

    pub deathmatch: i32, // only if started as net death
    pub netgame: bool,   // only true if packets are broadcast
    pub playeringame: [bool; MAXPLAYERS],
    pub players: [player_t; MAXPLAYERS],
    pub turbodetected: [bool; MAXPLAYERS],

    pub consoleplayer: i32, // player taking events and displaying
    pub displayplayer: i32, // view being displayed
    pub levelstarttic: i32, // gametic at level start
    pub totalkills: i32,    // for intermission
    pub totalitems: i32,
    pub totalsecret: i32,

    pub demoname: String,
    pub demorecording: bool,
    pub longtics: bool,    // cph's doom 1.91 longtics hack
    pub lowres_turn: bool, // low resolution turning for longtics
    pub demoplayback: bool,
    pub netdemo: bool,
    pub demobuffer: *mut u8,
    pub demo_p: *mut u8,
    pub demoend: *mut u8,
    pub singledemo: bool, // quit after playing a demo from cmdline

    pub precache: bool, // if true, load all graphics at start

    pub testcontrols: bool, // Invoked by setup to test controls
    pub testcontrols_mousespeed: i32,

    //pub wminfo: wbstartstruct_t, // parms for world map / intermission
    pub consistancy: [[u8; MAXPLAYERS]; BACKUPTICS as usize],

    pub forwardmove: [i32; 2],
    pub sidemove: [i32; 2],
    pub angleturn: [i32; 3], // + slow turn

    // Set to -1 or +1 to switch to the previous or next weapon.
    pub next_weapon: i32,

    pub gamekeydown: [bool; NUMKEYS as usize],
    pub turnheld: i32, // for accelerative turning

    pub mousearray: [bool; MAX_MOUSE_BUTTONS as usize + 1],
    pub mousebuttons: *mut bool, //&mousearray[1];  // allow [-1]

    // mouse values are used once
    pub mousex: i32,
    pub mousey: i32,

    pub dclicktime: i32,
    pub dclickstate: bool,
    pub dclicks: i32,
    pub dclicktime2: i32,
    pub dclickstate2: bool,
    pub dclicks2: i32,

    // joystick values are repeated
    pub joyxmove: i32,
    pub joyymove: i32,
    pub joystrafemove: i32,
    pub joyarray: [bool; MAX_JOY_BUTTONS as usize + 1],
    pub joybuttons: *mut bool, // = &joyarray[1];		// allow [-1]

    pub savegameslot: i32,
    pub savedescription: String,

    pub bodyque: [*mut mobj_t; BODYQUESIZE as usize],
    pub bodyqueslot: i32,

    pub vanilla_savegame_limit: i32,
    pub vanilla_demo_limit: i32,

    pub secretexit: bool,
    pub savename: String,

    pub d_skill: skill_t,
    pub d_episode: i32,
    pub d_map: i32,

    pub defdemoname: String,
}

impl g {
    pub fn new() -> Self {
        let player1 = player_t::new();
        let player2 = player_t::new();
        let player3 = player_t::new();
        let player4 = player_t::new();

        let players: [player_t; MAXPLAYERS] = [player1, player2, player3, player4];

        Self {
            /////////////////////////////
            // g_game.c
            /////////////////////////////
            // Gamestate the last time G_Ticker was called.
            oldgamestate: gamestate_t::GS_NONE, // -1

            gameaction: gameaction_t::default(),
            gamestate: gamestate_t::default(),
            //gameskill: skill_t,
            respawnmonsters: false,
            gameepisode: 0,
            gamemap: 0,

            // If non-zero, exit the level after this number of minutes.
            timelimit: 0,

            paused: false,
            sendpause: false, // send a pause event next tic
            sendsave: false,  // send a save event next tic
            usergame: false,  // ok to save / end game

            timingdemo: false, // if true, exit with report on completion
            nodrawers: false,  // for comparative timing purposes
            starttime: 0,      // for comparative timing purposes

            viewactive: false,

            deathmatch: 0,  // only if started as net death
            netgame: false, // only true if packets are broadcast
            playeringame: [false; MAXPLAYERS],
            players,
            turbodetected: [false; MAXPLAYERS],

            consoleplayer: 0, // player taking events and displaying
            displayplayer: 0, // view being displayed
            levelstarttic: 0, // gametic at level start
            totalkills: 0,    // for intermission
            totalitems: 0,
            totalsecret: 0,

            demoname: String::new(),
            demorecording: false,
            longtics: false,    // cph's doom 1.91 longtics hack
            lowres_turn: false, // low resolution turning for longtics
            demoplayback: false,
            netdemo: false,
            demobuffer: ptr::null_mut(),
            demo_p: ptr::null_mut(),
            demoend: ptr::null_mut(),
            singledemo: false, // quit after playing a demo from cmdline

            precache: false, // if true, load all graphics at start

            testcontrols: false, // Invoked by setup to test controls
            testcontrols_mousespeed: 0,

            //wminfo: wbstartstruct_t, // parms for world map / intermission
            consistancy: [[0; MAXPLAYERS]; BACKUPTICS as usize],

            forwardmove: [0; 2],
            sidemove: [0; 2],
            angleturn: [0; 3], // + slow turn

            // Set to -1 or +1 to switch to the previous or next weapon.
            next_weapon: 0,

            gamekeydown: [false; NUMKEYS as usize],
            turnheld: 0, // for accelerative turning

            mousearray: [false; MAX_MOUSE_BUTTONS as usize + 1],
            mousebuttons: ptr::null_mut(), //&mousearray[1];  // allow [-1]

            // mouse values are used once
            mousex: 0,
            mousey: 0,

            dclicktime: 0,
            dclickstate: false,
            dclicks: 0,
            dclicktime2: 0,
            dclickstate2: false,
            dclicks2: 0,

            // joystick values are repeated
            joyxmove: 0,
            joyymove: 0,
            joystrafemove: 0,
            joyarray: [false; MAX_JOY_BUTTONS as usize + 1],
            joybuttons: ptr::null_mut(), // = &joyarray[1];		// allow [-1]

            savegameslot: 0,
            savedescription: String::new(),

            bodyque: [ptr::null_mut(); BODYQUESIZE as usize],
            bodyqueslot: 0,

            vanilla_savegame_limit: 0,
            vanilla_demo_limit: 0,

            secretexit: false,
            savename: String::new(),

            d_skill: skill_t::sk_noitems,
            d_episode: 0,
            d_map: 0,

            defdemoname: String::new(),
        }
    }
}
