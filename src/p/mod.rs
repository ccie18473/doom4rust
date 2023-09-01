#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod p_ceilng;
pub mod p_doors;
pub mod p_enemy;
pub mod p_floor;
pub mod p_inter;
pub mod p_lights;
pub mod p_local;
pub mod p_map;
pub mod p_maputl;
pub mod p_mobj;
pub mod p_plats;
pub mod p_pspr;
pub mod p_saveg;
pub mod p_setup;
pub mod p_sight;
pub mod p_spec;
pub mod p_switch;
pub mod p_telept;
pub mod p_tick;
pub mod p_user;

pub use p_ceilng::*;
pub use p_doors::*;
pub use p_enemy::*;
pub use p_floor::*;
pub use p_inter::*;
pub use p_lights::*;
pub use p_local::*;
pub use p_map::*;
pub use p_maputl::*;
pub use p_mobj::*;
pub use p_plats::*;
pub use p_pspr::*;
pub use p_saveg::*;
pub use p_setup::*;
pub use p_sight::*;
pub use p_spec::*;
pub use p_switch::*;
pub use p_telept::*;
pub use p_tick::*;
pub use p_user::*;

use crate::*;

/////////////////////////////
// P_* Game logic/behaviour
/////////////////////////////

pub struct p {
    /////////////////////////////
    // p_spec.c
    /////////////////////////////
    pub numlinespecials: i16,
    pub linespeciallist: [*mut line_t; MAXLINEANIMS as usize],
    pub levelTimer: bool,
    pub levelTimeCount: i32,
    pub anims: [p_spec::anim_t; MAXANIMS as usize],
    pub lastanim: *mut p_spec::anim_t,
    /////////////////////////////
    // p_switch.c
    /////////////////////////////
    pub switchlist: [i32; MAXSWITCHES as usize * 2],
    pub numswitches: i32,
    pub buttonlist: [button_t; MAXBUTTONS as usize],
}

impl p {
    pub fn new() -> Self {
        Self {
            /////////////////////////////
            // p_spec.c
            /////////////////////////////
            numlinespecials: 0,
            linespeciallist: [ptr::null_mut(); MAXLINEANIMS as usize],
            levelTimer: false,
            levelTimeCount: 0,
            anims: [p_spec::anim_t::new(); MAXANIMS as usize],
            lastanim: ptr::null_mut(),
            /////////////////////////////
            // p_switch.c
            /////////////////////////////
            switchlist: [0; MAXSWITCHES as usize * 2],
            numswitches: 0,
            buttonlist: [button_t::new(); MAXBUTTONS as usize],
        }
    }
}
