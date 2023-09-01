#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// hu_stuff.h
/////////////////////////////
//
// DESCRIPTION:  Head up display
//

//
// Globally visible constants.
//
pub const HU_FONTSTART: char = '!'; // the first font characters
pub const HU_FONTEND: char = '_'; // the last font characters

// Calculate # of glyphs in font.
pub const HU_FONTSIZE: u8 = HU_FONTEND as u8 - HU_FONTSTART as u8 + 1;

pub const HU_BROADCAST: u8 = 5;

pub const HU_MSGX: u8 = 0;
pub const HU_MSGY: u8 = 0;
pub const HU_MSGWIDTH: u8 = 64; // in characters
pub const HU_MSGHEIGHT: u8 = 1; // in lines

pub const HU_MSGTIMEOUT: i32 = 4 * TICRATE;

/////////////////////////////
// hu_stuff.c
/////////////////////////////
//
// DESCRIPTION:  Heads-up displays
//

//
// Locally used constants, shortcuts.
//
//pub const HU_TITLE	(mapnames[(gameepisode-1)*9+gamemap-1])
//pub const HU_TITLE2	(mapnames_commercial[gamemap-1])
//pub const HU_TITLEP	(mapnames_commercial[gamemap-1 + 32])
//pub const HU_TITLET	(mapnames_commercial[gamemap-1 + 64])
//pub const HU_TITLE_CHEX   (mapnames[gamemap - 1])
pub const HU_TITLEHEIGHT: u8 = 1;
pub const HU_TITLEX: u8 = 0;
//pub const HU_TITLEY	(167 - SHORT(hu_font[0]->height))

pub const HU_INPUTTOGGLE: char = 't';
pub const HU_INPUTX: u8 = HU_MSGX;
//pub const HU_INPUTY	(HU_MSGY + HU_MSGHEIGHT*(SHORT(hu_font[0]->height) +1))
pub const HU_INPUTWIDTH: u8 = 64;
pub const HU_INPUTHEIGHT: u8 = 1;

pub const chat_macros: [&str; 10] = [
    HUSTR_CHATMACRO0,
    HUSTR_CHATMACRO1,
    HUSTR_CHATMACRO2,
    HUSTR_CHATMACRO3,
    HUSTR_CHATMACRO4,
    HUSTR_CHATMACRO5,
    HUSTR_CHATMACRO6,
    HUSTR_CHATMACRO7,
    HUSTR_CHATMACRO8,
    HUSTR_CHATMACRO9,
];

pub const player_names: [&str; 4] = [
    HUSTR_PLRGREEN,
    HUSTR_PLRINDIGO,
    HUSTR_PLRBROWN,
    HUSTR_PLRRED,
];

//
// Builtin map names.
// The actual names can be found in DStrings.h.
//

pub const mapnames: [&str; 45] = // DOOM shareware/registered/retail (Ultimate) names.
    [
        HUSTR_E1M1, HUSTR_E1M2, HUSTR_E1M3, HUSTR_E1M4, HUSTR_E1M5, HUSTR_E1M6, HUSTR_E1M7,
        HUSTR_E1M8, HUSTR_E1M9, HUSTR_E2M1, HUSTR_E2M2, HUSTR_E2M3, HUSTR_E2M4, HUSTR_E2M5,
        HUSTR_E2M6, HUSTR_E2M7, HUSTR_E2M8, HUSTR_E2M9, HUSTR_E3M1, HUSTR_E3M2, HUSTR_E3M3,
        HUSTR_E3M4, HUSTR_E3M5, HUSTR_E3M6, HUSTR_E3M7, HUSTR_E3M8, HUSTR_E3M9, HUSTR_E4M1,
        HUSTR_E4M2, HUSTR_E4M3, HUSTR_E4M4, HUSTR_E4M5, HUSTR_E4M6, HUSTR_E4M7, HUSTR_E4M8,
        HUSTR_E4M9, "NEWLEVEL", "NEWLEVEL", "NEWLEVEL", "NEWLEVEL", "NEWLEVEL", "NEWLEVEL",
        "NEWLEVEL", "NEWLEVEL", "NEWLEVEL",
    ];

// List of names for levels in commercial IWADs
// (doom2.wad, plutonia.wad, tnt.wad).  These are stored in a
// single large array; WADs like pl2.wad have a MAP33, and rely on
// the layout in the Vanilla executable, where it is possible to
// overflow the end of one array into the next.

pub const mapnames_commercial: [&str; 96] = [
    // DOOM 2 map names.
    HUSTR_1, HUSTR_2, HUSTR_3, HUSTR_4, HUSTR_5, HUSTR_6, HUSTR_7, HUSTR_8, HUSTR_9, HUSTR_10,
    HUSTR_11, HUSTR_12, HUSTR_13, HUSTR_14, HUSTR_15, HUSTR_16, HUSTR_17, HUSTR_18, HUSTR_19,
    HUSTR_20, HUSTR_21, HUSTR_22, HUSTR_23, HUSTR_24, HUSTR_25, HUSTR_26, HUSTR_27, HUSTR_28,
    HUSTR_29, HUSTR_30, HUSTR_31, HUSTR_32, // Plutonia WAD map names.
    PHUSTR_1, PHUSTR_2, PHUSTR_3, PHUSTR_4, PHUSTR_5, PHUSTR_6, PHUSTR_7, PHUSTR_8, PHUSTR_9,
    PHUSTR_10, PHUSTR_11, PHUSTR_12, PHUSTR_13, PHUSTR_14, PHUSTR_15, PHUSTR_16, PHUSTR_17,
    PHUSTR_18, PHUSTR_19, PHUSTR_20, PHUSTR_21, PHUSTR_22, PHUSTR_23, PHUSTR_24, PHUSTR_25,
    PHUSTR_26, PHUSTR_27, PHUSTR_28, PHUSTR_29, PHUSTR_30, PHUSTR_31, PHUSTR_32,
    // TNT WAD map names.
    THUSTR_1, THUSTR_2, THUSTR_3, THUSTR_4, THUSTR_5, THUSTR_6, THUSTR_7, THUSTR_8, THUSTR_9,
    THUSTR_10, THUSTR_11, THUSTR_12, THUSTR_13, THUSTR_14, THUSTR_15, THUSTR_16, THUSTR_17,
    THUSTR_18, THUSTR_19, THUSTR_20, THUSTR_21, THUSTR_22, THUSTR_23, THUSTR_24, THUSTR_25,
    THUSTR_26, THUSTR_27, THUSTR_28, THUSTR_29, THUSTR_30, THUSTR_31, THUSTR_32,
];

pub const QUEUESIZE: u8 = 128;

pub fn HU_Init(doom: &mut modules) {
    println!("HU_Init");

    // load the heads-up font
    let mut j = HU_FONTSTART as usize;
    for i in 0..HU_FONTSIZE as usize {
        let buffer = format!("STCFN{:0>3}", j);
        j += 1;
        doom.hu.hu_font[i] =
            W_CacheLumpName(doom, &buffer, PURGE::PU_STATIC as i32) as *mut patch_t;
    }
}

pub fn HU_Stop() {
    println!("HU_Stop");
}

pub fn HU_Start() {
    println!("HU_Start");
}

pub fn HU_Drawer() {
    println!("HU_Drawer");
}

pub fn HU_Erase() {
    println!("HU_Erase");
}

pub fn HU_Ticker() {
    println!("HU_Ticker");
}

pub fn HU_queueChatChar(c: char) {
    println!("HU_queueChatChar");
}

pub fn HU_dequeueChatChar() -> char {
    println!("HU_dequeueChatChar");

    return 'z';
}

pub fn HU_Responder(ev: *mut event_t) -> bool {
    println!("HU_Responder");

    return false;
}
