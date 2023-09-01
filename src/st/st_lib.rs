#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// st_lib.h
/////////////////////////////
//
// DESCRIPTION:
// 	The status bar widget code.
//

//
// Typedefs of widgets
//

// Number widget

pub struct st_number_t {
    // upper right-hand corner
    //  of the number (right-justified)
    pub x: i32,
    pub y: i32,

    // max # of digits in number
    pub width: i32,

    // last number value
    pub oldnum: i32,

    // pointer to current value
    pub num: *mut i32,

    // pointer to boolean stating
    //  whether to update number
    pub on: *mut bool,

    // list of patches for 0-9
    pub p: *mut *mut patch_t,

    // user data
    pub data: i32,
}

// Percent widget ("child" of number widget,
//  or, more precisely, contains a number widget.)
pub struct st_percent_t {
    // number information
    pub n: st_number_t,

    // percent sign graphic
    pub p: *mut patch_t,
}

// Multiple Icon widget
pub struct st_multicon_t {
    // center-justified location of icons
    pub x: i32,
    pub y: i32,

    // last icon number
    pub oldinum: i32,

    // pointer to current icon
    pub inum: *mut i32,

    // pointer to boolean stating
    //  whether to update icon
    pub on: *mut bool,

    // list of icons
    pub p: *mut *mut patch_t,

    // user data
    pub data: i32,
}

// Binary Icon widget

pub struct st_binicon_t {
    // center-justified location of icon
    pub x: i32,
    pub y: i32,

    // last icon value
    pub oldval: bool,

    // pointer to current icon status
    pub val: *mut bool,

    // pointer to boolean
    //  stating whether to update icon
    pub on: *mut bool,

    pub p: *mut patch_t, // icon
    pub data: i32,       // user data
}

/////////////////////////////
// st_lib.c
/////////////////////////////
//
// DESCRIPTION:
//	The status bar widget code.
//

//
// Hack display negative frags.
//  Loads and store the stminus lump.
//

pub fn STlib_init() {
    println!("STlib_init");
}

// ?
pub fn STlib_initNum(
    n: *mut st_number_t,
    x: i32,
    y: i32,
    pl: *mut *mut patch_t,
    num: *mut i32,
    on: *mut bool,
    width: i32,
) {
    println!("STlib_initNum");
}

//
// A fairly efficient way to draw a number
//  based on differences from the old number.
// Note: worth the trouble?
//
pub fn STlib_drawNum(n: *mut st_number_t, refresh: bool) {
    println!("STlib_drawNum");
}

//
pub fn STlib_updateNum(n: *mut st_number_t, refresh: bool) {
    println!("STlib_updateNum");
}

//
pub fn STlib_initPercent(
    p: *mut st_percent_t,
    x: i32,
    y: i32,
    pl: *mut *mut patch_t,
    num: *mut i32,
    on: *mut bool,
    percent: *mut patch_t,
) {
    println!("STlib_initPercent");
}

pub fn STlib_updatePercent(per: *mut st_percent_t, refresh: i32) {
    println!("STlib_updatePercent");
}

pub fn STlib_initMultIcon(
    i: *mut st_multicon_t,
    x: i32,
    y: i32,
    il: *mut *mut patch_t,
    inum: *mut i32,
    on: *mut bool,
) {
    println!("STlib_initMultIcon");
}

pub fn STlib_updateMultIcon(mi: *mut st_multicon_t, refresh: bool) {
    println!("STlib_updateMultIcon");
}

pub fn STlib_initBinIcon(
    b: *mut st_binicon_t,
    x: i32,
    y: i32,
    i: *mut patch_t,
    val: *mut bool,
    on: *mut bool,
) {
    println!("STlib_initBinIcon");
}

pub fn STlib_updateBinIcon(bi: *mut st_binicon_t, refresh: bool) {
    println!("STlib_updateBinIcon");
}
