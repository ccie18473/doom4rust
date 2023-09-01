#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/////////////////////////////
// m_cheat.h
/////////////////////////////
//
// DESCRIPTION:
//	Cheat sequence checking.
//

//
// CHEAT SEQUENCE PACKAGE
//

// declaring a cheat

//#define CHEAT(value, parameters) \
//    { value, sizeof(value) - 1, parameters, 0, 0, "" }

pub const MAX_CHEAT_LEN: u8 = 25;
pub const MAX_CHEAT_PARAMS: u8 = 5;

pub struct cheatseq_t {
    // settings for this cheat
    pub sequence: String,
    pub sequence_len: usize,
    pub parameter_chars: i32,

    // state used during the game
    pub chars_read: usize,
    pub param_chars_read: i32,
    pub parameter_buf: String,
}

impl cheatseq_t {
    pub fn new() -> Self {
        Self {
            sequence: String::new(),
            sequence_len: 0,
            parameter_chars: 0,
            chars_read: 0,
            param_chars_read: 0,
            parameter_buf: String::new(),
        }
    }
}

/////////////////////////////
// m_cheat.c
/////////////////////////////
//
// DESCRIPTION:
//	Cheat sequence checking.
//

//
// CHEAT SEQUENCE PACKAGE
//

//
// Called in st_stuff module, which handles the input.
// Returns a 1 if the cheat was successful, 0 if failed.
//
pub fn cht_CheckCheat(cht: *mut cheatseq_t, key: u8) -> i32 {
    println!("cht_CheckCheat");

    return 0;
}

pub fn cht_GetParam(cht: *mut cheatseq_t, buffer: &str) {
    println!("cht_GetParam");
}
