#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// st_stuff.h
/////////////////////////////
//
// DESCRIPTION:
//	Status bar code.
//	Does the face/direction indicator animatin.
//	Does palette indicators as well (red pain/berserk, bright pickup)
//

// Size of statusbar.
// Now sensitive for scaling.
pub const ST_HEIGHT: i32 = 32;
pub const ST_WIDTH: i32 = SCREENWIDTH;
pub const ST_Y: i32 = SCREENHEIGHT - ST_HEIGHT;

// States for status bar code.
pub enum st_stateenum_t {
    AutomapState,
    FirstPersonState,
}

// States for the chat code.
pub enum st_chatstateenum_t {
    StartChatState,
    WaitDestState,
    GetChatState,
}

/////////////////////////////
// st_stuff.c
/////////////////////////////
//
// DESCRIPTION:
//	Status bar code.
//	Does the face/direction indicator animatin.
//	Does palette indicators as well (red pain/berserk, bright pickup)
//

//
// STATUS BAR CODE
//

pub fn ST_refreshBackground() {
    println!("ST_refreshBackground");
}

// Respond to keyboard input events,
//  intercept cheats.
pub fn ST_Responder(ev: *mut event_t) -> bool {
    println!("ST_Responder");

    return false;
}

pub fn ST_calcPainOffset() -> i32 {
    println!("ST_calcPainOffset");

    return 0;
}

//
// This is a not-very-pretty routine which handles
//  the face states and their timing.
// the precedence of expressions is:
//  dead > evil grin > turned head > straight ahead
//
pub fn ST_updateFaceWidget() {
    println!("ST_updateFaceWidget");
}

pub fn ST_updateWidgets() {
    println!("ST_updateWidgets");
}

pub fn ST_Ticker() {
    println!("ST_Ticker");
}

pub fn ST_doPaletteStuff() {
    println!("ST_doPaletteStuff");
}

pub fn ST_drawWidgets(refresh: bool) {
    println!("ST_drawWidgets");
}

pub fn ST_doRefresh() {
    println!("ST_doRefresh");
}

pub fn ST_diffDraw() {
    println!("ST_diffDraw");
}

pub fn ST_Drawer(fullscreen: bool, refresh: bool) {
    println!("ST_Drawer");
}

//BUG
pub type load_callback_t = fn(doom: &mut modules, lumpname: &str, variable: *mut *mut patch_t);

// Iterates through all graphics to be loaded or unloaded, along with
// the variable they use, invoking the specified callback function.

pub fn ST_loadUnloadGraphics(callback: load_callback_t) {
    println!("ST_loadUnloadGraphics");
}

pub fn ST_loadCallback(doom: &mut modules, lumpname: &str, variable: *mut *mut patch_t) {
    println!("ST_loadCallback");

    unsafe { *variable = W_CacheLumpName(doom, lumpname, PURGE::PU_STATIC as i32) as *mut patch_t };
}

pub fn ST_loadGraphics() {
    println!("ST_loadGraphics");

    ST_loadUnloadGraphics(ST_loadCallback);
}

pub fn ST_loadData(doom: &mut modules) {
    println!("ST_loadData");

    doom.st.lu_palette = W_GetNumForName(doom, DEH_String("PLAYPAL"));
    ST_loadGraphics();
}

pub fn ST_unloadCallback(lumpname: &str, variable: *mut *mut patch_t) {
    println!("ST_unloadCallback");
}

pub fn ST_unloadGraphics() {
    println!("ST_unloadGraphics");
}

pub fn ST_unloadData() {
    println!("ST_unloadData");

    ST_unloadGraphics();
}

pub fn ST_initData() {
    println!("ST_initData");
}

pub fn ST_createWidgets() {
    println!("ST_createWidgets");
}

pub fn ST_Start() {
    println!("ST_Start");
}

pub fn ST_Stop() {
    println!("ST_Stop");
}

pub fn ST_Init(doom: &mut modules) {
    println!("ST_Init");

    ST_loadData(doom);
    doom.st.st_backing_screen = Z_Malloc(
        ST_WIDTH * ST_HEIGHT,
        PURGE::PU_STATIC as i32,
        ptr::null_mut(),
    ) as *mut u8;
}
