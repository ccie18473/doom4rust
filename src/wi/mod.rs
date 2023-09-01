#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod wi_stuff;

pub use wi_stuff::*;

use crate::*;

/////////////////////////////
// WI_* End-of level "intermission" screen
/////////////////////////////

pub struct wi {
    /////////////////////////////
    // wi_stuff.c
    /////////////////////////////
    // used to accelerate or skip a stage
    pub acceleratestage: i32,

    // wbs->pnum
    pub me: i32,

    // specifies current state
    pub state: stateenum_t,

    // contains information passed into intermission
    pub wbs: *mut wbstartstruct_t,

    pub plrs: *mut wbplayerstruct_t, // wbs->plyr[]

    // used for general timing
    pub cnt: i32,

    // used for timing of background animation
    pub bcnt: i32,

    // signals to refresh everything for one frame
    pub firstrefresh: i32,

    pub cnt_kills: [i32; MAXPLAYERS],
    pub cnt_items: [i32; MAXPLAYERS],
    pub cnt_secret: [i32; MAXPLAYERS],
    pub cnt_time: i32,
    pub cnt_par: i32,
    pub cnt_pause: i32,

    // # of commercial levels
    pub NUMCMAPS: i32,

    //
    //	GRAPHICS
    //

    // You Are Here graphic
    pub yah: [*mut patch_t; 3],

    // splat
    pub splat: [*mut patch_t; 2],

    // %, : graphics
    pub percent: *mut patch_t,
    pub colon: *mut patch_t,

    // 0-9 graphic
    pub num: [*mut patch_t; 10],

    // minus sign
    pub wiminus: *mut patch_t,

    // "Finished!" graphics
    pub finished: *mut patch_t,

    // "Entering" graphic
    pub entering: *mut patch_t,

    // "secret"
    pub sp_secret: *mut patch_t,

    // "Kills", "Scrt", "Items", "Frags"
    pub kills: *mut patch_t,
    pub secret: *mut patch_t,
    pub items: *mut patch_t,
    pub frags: *mut patch_t,

    // Time sucks.
    pub timepatch: *mut patch_t,
    pub par: *mut patch_t,
    pub sucks: *mut patch_t,

    // "killers", "victims"
    pub killers: *mut patch_t,
    pub victims: *mut patch_t,

    // "Total", your face, your dead face
    pub total: *mut patch_t,
    pub star: *mut patch_t,
    pub bstar: *mut patch_t,

    // "red P[1..MAXPLAYERS]"
    pub p: [*mut patch_t; MAXPLAYERS],

    // "gray P[1..MAXPLAYERS]"
    pub bp: [*mut patch_t; MAXPLAYERS],

    // Name graphics of each level (centered)
    pub lnames: *mut *mut patch_t,

    // Buffer storing the backdrop
    pub background: *mut patch_t,

    pub snl_pointeron: bool,
    pub dm_state: i32,
    pub dm_frags: [[i32; MAXPLAYERS]; MAXPLAYERS],
    pub dm_totals: [i32; MAXPLAYERS],
    pub cnt_frags: [i32; MAXPLAYERS],
    pub dofrags: i32,
    pub ng_state: i32,
    pub sp_state: i32,
}

impl wi {
    pub fn new() -> Self {
        Self {
            /////////////////////////////
            // wi_stuff.c
            /////////////////////////////
            acceleratestage: 0,
            me: 0,
            state: stateenum_t::NoState,
            wbs: ptr::null_mut(),
            plrs: ptr::null_mut(), // wbs->plyr[]
            cnt: 0,
            bcnt: 0,
            firstrefresh: 0,
            cnt_kills: [0; MAXPLAYERS],
            cnt_items: [0; MAXPLAYERS],
            cnt_secret: [0; MAXPLAYERS],
            cnt_time: 0,
            cnt_par: 0,
            cnt_pause: 0,
            NUMCMAPS: 0,
            yah: [ptr::null_mut(); 3],
            splat: [ptr::null_mut(); 2],
            percent: ptr::null_mut(),
            colon: ptr::null_mut(),
            num: [ptr::null_mut(); 10],
            wiminus: ptr::null_mut(),
            finished: ptr::null_mut(),
            entering: ptr::null_mut(),
            sp_secret: ptr::null_mut(),
            kills: ptr::null_mut(),
            secret: ptr::null_mut(),
            items: ptr::null_mut(),
            frags: ptr::null_mut(),
            timepatch: ptr::null_mut(),
            par: ptr::null_mut(),
            sucks: ptr::null_mut(),
            killers: ptr::null_mut(),
            victims: ptr::null_mut(),
            total: ptr::null_mut(),
            star: ptr::null_mut(),
            bstar: ptr::null_mut(),
            p: [ptr::null_mut(); MAXPLAYERS],
            bp: [ptr::null_mut(); MAXPLAYERS],
            lnames: ptr::null_mut(),
            background: ptr::null_mut(),
            snl_pointeron: false,
            dm_state: 0,
            dm_frags: [[0; MAXPLAYERS]; MAXPLAYERS],
            dm_totals: [0; MAXPLAYERS],
            cnt_frags: [0; MAXPLAYERS],
            dofrags: 0,
            ng_state: 0,
            sp_state: 0,
        }
    }
}
