#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod hu_lib;
pub mod hu_stuff;

pub use hu_lib::*;
pub use hu_stuff::*;

use crate::*;

/////////////////////////////
// HU_* Heads-up display
/////////////////////////////

pub struct hu {
    /////////////////////////////
    // hu_stuff.c
    /////////////////////////////
    pub chat_char: char, // remove later.
    pub plr: *mut player_t,
    pub hu_font: [*mut patch_t; HU_FONTSIZE as usize],
    pub w_title: hu_textline_t,
    pub chat_on: bool,
    pub w_chat: hu_itext_t,
    pub always_off: bool,
    pub chat_dest: String,
    //pub w_inputbuffer: [hu_itext_t;MAXPLAYERS],
    pub message_on: bool,
    pub message_dontfuckwithme: bool,
    pub message_nottobefuckedwith: bool,

    pub w_message: hu_stext_t,
    pub message_counter: i32,

    pub headsupactive: bool,

    pub chatchars: String,
    pub head: i32,
    pub tail: i32,
}

impl hu {
    pub fn new() -> Self {
        Self {
            /////////////////////////////
            // hu_stuff.c
            /////////////////////////////
            chat_char: char::default(), // remove later.
            plr: ptr::null_mut(),
            hu_font: [ptr::null_mut(); HU_FONTSIZE as usize],
            w_title: hu_textline_t::new(),
            chat_on: false,
            w_chat: hu_itext_t::new(),
            always_off: false,
            chat_dest: String::new(),
            //w_inputbuffer: [hu_itext_t::new();MAXPLAYERS],
            message_on: false,
            message_dontfuckwithme: false,
            message_nottobefuckedwith: false,
            w_message: hu_stext_t::new(),
            message_counter: 0,
            headsupactive: false,
            chatchars: String::new(),
            head: 0,
            tail: 0,
        }
    }
}
