#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod am_map;

pub use am_map::*;

use super::*;

/////////////////////////////
// AM_* Automap code
/////////////////////////////
pub struct am {
    /////////////////////////////
    // am_map.c
    /////////////////////////////
    pub cheating: i32,
    pub grid: i32,

    pub leveljuststarted: i32, // kluge until AM_LevelInit() is called

    pub automapactive: bool,
    pub finit_width: i32,
    pub finit_height: i32,

    // location of window on screen
    pub f_x: i32,
    pub f_y: i32,

    // size of window on screen
    pub f_w: i32,
    pub f_h: i32,

    pub lightlev: i32, // used for funky strobing effect
    pub fb: *mut u8,   // pseudo-frame buffer
    pub amclock: i32,

    pub m_paninc: mpoint_t, // how far the window pans each tic (map coords)
    pub mtof_zoommul: i32,  // how far the window zooms in each tic (map coords)
    pub ftom_zoommul: i32,  // how far the window zooms in each tic (fb coords)

    pub m_x: i32, // LL x,y where the window is on the map (map coords)
    pub m_y: i32,
    pub m_x2: i32, // UR x,y where the window is on the map (map coords)
    pub m_y2: i32,
    //
    // width/height of window on map (map coords)
    //
    pub m_w: i32,
    pub m_h: i32,

    // based on level size
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,

    pub max_w: i32, // max_x-min_x,
    pub max_h: i32, // max_y-min_y

    // based on player size
    pub min_w: i32,
    pub min_h: i32,

    pub min_scale_mtof: i32, // used to tell when to stop zooming out
    pub max_scale_mtof: i32, // used to tell when to stop zooming in

    // old stuff for recovery later
    pub old_m_w: i32,
    pub old_m_h: i32,
    pub old_m_x: i32,
    pub old_m_y: i32,

    // old location used by the Follower routine
    pub f_oldloc: mpoint_t,

    // used by MTOF to scale from map-to-frame-buffer coords
    pub scale_mtof: i32,
    // used by FTOM to scale from frame-buffer-to-map coords (=1/scale_mtof)
    pub scale_ftom: i32,

    pub plr: *mut player_t, // the player represented by an arrow

    pub marknums: [*mut patch_t; 10], // numbers used for marking by the automap
    pub markpoints: [mpoint_t; AM_NUMMARKPOINTS], // where the points are
    pub markpointnum: i32,            // next point to be assigned

    pub followplayer: i32, // specifies whether to follow the player around

    pub cheat_amap: cheatseq_t,

    pub stopped: bool,
}

impl am {
    pub fn new() -> Self {
        Self {
            /////////////////////////////
            // am_map.c
            /////////////////////////////
            cheating: 0,
            grid: 0,
            leveljuststarted: 0,
            automapactive: false,
            finit_width: 0,
            finit_height: 0,
            f_x: 0,
            f_y: 0,
            f_w: 0,
            f_h: 0,
            lightlev: 0,
            fb: ptr::null_mut(),
            amclock: 0,
            m_paninc: mpoint_t::new(),
            mtof_zoommul: 0,
            ftom_zoommul: 0,
            m_x: 0,
            m_y: 0,
            m_x2: 0,
            m_y2: 0,
            m_w: 0,
            m_h: 0,
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
            max_w: 0,
            max_h: 0,
            min_w: 0,
            min_h: 0,
            min_scale_mtof: 0,
            max_scale_mtof: 0,
            old_m_w: 0,
            old_m_h: 0,
            old_m_x: 0,
            old_m_y: 0,
            f_oldloc: mpoint_t::new(),
            scale_mtof: 0,
            scale_ftom: 0,
            plr: ptr::null_mut(),
            marknums: [ptr::null_mut(); 10],
            markpoints: [mpoint_t::new(); AM_NUMMARKPOINTS],
            markpointnum: 0,
            followplayer: 0,
            cheat_amap: cheatseq_t::new(),
            stopped: false,
        }
    }
}
