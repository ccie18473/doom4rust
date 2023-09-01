#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// hu_lib.h
/////////////////////////////
//
// DESCRIPTION:  none
//

// font stuff
pub const HU_CHARERASE: u8 = KEY_BACKSPACE;

pub const HU_MAXLINES: usize = 4;
pub const HU_MAXLINELENGTH: i32 = 80;

//
// Typedefs of widgets
//

// Text Line widget
//  (parent of Scrolling Text and Input Text widgets)
pub struct hu_textline_t {
    // left-justified position of scrolling text window
    pub x: i32,
    pub y: i32,

    pub f: *mut *mut patch_t, // font
    pub sc: i32,              // start character
    pub l: String,            //[HU_MAXLINELENGTH+1];	// line of text
    pub len: i32,             // current line length

    // whether this line needs to be udpated
    pub needsupdate: i32,
}

impl hu_textline_t {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            f: ptr::null_mut(),
            sc: 0,
            l: String::new(),
            len: 0,
            needsupdate: 0,
        }
    }
}

// Scrolling Text window widget
//  (child of Text Line widget)
pub struct hu_stext_t {
    //pub l: [hu_textline_t; HU_MAXLINES], // text lines to draw
    pub h: i32,  // height in lines
    pub cl: i32, // current line number

    // pointer to boolean stating whether to update window
    pub on: *mut bool,
    pub laston: bool, // last value of *->on.
}

impl hu_stext_t {
    pub fn new() -> Self {
        Self {
            //l: [hu_textline_t::new(); HU_MAXLINES],
            h: 0,
            cl: 0,
            on: ptr::null_mut(),
            laston: false,
        }
    }
}

// Input Text Line widget
//  (child of Text Line widget)
pub struct hu_itext_t {
    pub l: hu_textline_t, // text line to input on

    // left margin past which I am not to delete characters
    pub lm: i32,

    // pointer to boolean stating whether to update window
    pub on: *mut bool,
    pub laston: bool, // last value of *->on;
}

impl hu_itext_t {
    pub fn new() -> Self {
        Self {
            l: hu_textline_t::new(),
            lm: 0,
            on: ptr::null_mut(),
            laston: false,
        }
    }
}
/////////////////////////////
// hu_lib.c
/////////////////////////////
//
// DESCRIPTION:  heads-up text and input code
//

// boolean : whether the screen is always erased
//pub const noterased: i32 = viewwindowx;

pub fn HUlib_init() {}

pub fn HUlib_clearTextLine(t: *mut hu_textline_t) {}

pub fn HUlib_initTextLine(t: *mut hu_textline_t, x: i32, y: i32, f: *mut *mut patch_t, sc: i32) {}

pub fn HUlib_addCharToTextLine(t: *mut hu_textline_t, ch: char) -> bool {
    return false;
}

pub fn HUlib_delCharFromTextLine(t: *mut hu_textline_t) -> bool {
    return false;
}

pub fn HUlib_drawTextLine(l: *mut hu_textline_t, drawcursor: bool) {}

// sorta called by HU_Erase and just better darn get things straight
pub fn HUlib_eraseTextLine(l: *mut hu_textline_t) {}

pub fn HUlib_initSText(
    s: *mut hu_stext_t,
    x: i32,
    y: i32,
    h: i32,
    font: *mut *mut patch_t,
    startchar: i32,
    on: *mut bool,
) {
}

pub fn HUlib_addLineToSText(s: *mut hu_stext_t) {}

pub fn HUlib_addMessageToSText(s: *mut hu_stext_t, prefix: String, msg: String) {}

pub fn HUlib_drawSText(s: *mut hu_stext_t) {}

pub fn HUlib_eraseSText(s: *mut hu_stext_t) {}

pub fn HUlib_initIText(
    it: *mut hu_itext_t,
    x: i32,
    y: i32,
    font: *mut *mut patch_t,
    startchar: i32,
    on: *mut bool,
) {
}

// The following deletion routines adhere to the left margin restriction
pub fn HUlib_delCharFromIText(it: *mut hu_itext_t) {}

pub fn HUlib_eraseLineFromIText(it: *mut hu_itext_t) {}

// Resets left margin as well
pub fn HUlib_resetIText(it: *mut hu_itext_t) {}

pub fn HUlib_addPrefixToIText(it: *mut hu_itext_t, str: String) {}

// wrapper function for handling general keyed input.
// returns true if it ate the key
pub fn HUlib_keyInIText(it: *mut hu_itext_t, ch: char) -> bool {
    return false;
}

pub fn HUlib_drawIText(it: *mut hu_itext_t) {}
