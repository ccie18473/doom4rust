#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

pub mod m_argv;
pub mod m_bbox;
pub mod m_cheat;
pub mod m_config;
pub mod m_controls;
pub mod m_fixed;
pub mod m_menu;
pub mod m_misc;
pub mod m_random;

pub use m_argv::*;
pub use m_bbox::*;
pub use m_cheat::*;
pub use m_config::*;
pub use m_controls::*;
pub use m_fixed::*;
pub use m_menu::*;
pub use m_misc::*;
pub use m_random::*;

/////////////////////////////
// M_* Miscellaneous (includes the menu)
/////////////////////////////

pub struct m<'a> {
    pub myargv: Vec<String>,
    pub myargc: usize,
    /////////////////////////////
    // m_menu.c
    /////////////////////////////
    pub itemOn: i16,           // menu item skull is on
    pub skullAnimCounter: i16, // skull animation counter
    pub whichSkull: i16,       // which skull to draw
    // current menudef
    pub currentMenu: *mut menu_t,
    //
    // defaulted values
    //
    pub mouseSensitivity: i32, // = 5;
    // Show messages has default, 0 = off, 1 = on
    pub showMessages: i32, // = 1;
    // Blocky mode, has default, 0 = high, 1 = normal
    pub detailLevel: i32,  // = 0;
    pub screenblocks: i32, // = 10;
    // temp for screenblocks (0-9)
    pub screenSize: i32,
    // -1 = no quicksave slot picked!
    pub quickSaveSlot: i32,
    // 1 = message to be printed
    pub messageToPrint: i32,
    // ...and here is the message string!
    pub messageString: &'a str,
    // message x & y
    pub messx: i32,
    pub messy: i32,
    pub messageLastMenuActive: i32,
    // timed message = no input from user
    pub messageNeedsInput: bool,
    pub inhelpscreens: bool,
    pub menuactive: bool,
}

impl<'a> m<'a> {
    pub fn new() -> Self {
        Self {
            myargv: vec![String::new()],
            myargc: 0,
            /////////////////////////////
            // m_menu.c
            /////////////////////////////
            itemOn: 0,
            skullAnimCounter: 0,
            whichSkull: 0,
            currentMenu: ptr::null_mut(),
            mouseSensitivity: 5,
            showMessages: 1,
            detailLevel: 0,
            screenblocks: 10,
            screenSize: 0,
            quickSaveSlot: 0,
            messageToPrint: 0,
            messageString: "",
            messx: 0,
            messy: 0,
            messageLastMenuActive: 0,
            messageNeedsInput: false,
            inhelpscreens: false,
            menuactive: false,
        }
    }
}
