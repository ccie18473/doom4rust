#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// d_think.h
/////////////////////////////
//
// DESCRIPTION:
//  MapObj data. Map Objects or mobjs are actors, entities,
//  thinker, take-your-pick... anything that moves, acts, or
//  suffers state changes of more or less violent nature.
//

pub type actionf_v = fn();
pub type actionf_p1 = fn(*mut libc::c_void);
pub type actionf_p2 = fn(*mut libc::c_void, *mut libc::c_void);

pub union actionf_t {
    pub acv: actionf_v,
    pub acp1: actionf_p1,
    pub acp2: actionf_p2,
}

// Historically, "think_t" is yet another
//  function pointer to a routine to handle
//  an actor.

pub type think_t = actionf_t;

// Doubly linked list of actors.
pub type thinker_s = thinker_t;

pub struct thinker_t {
    pub prev: *mut thinker_s,
    pub next: *mut thinker_s,
    pub function: think_t,
}
