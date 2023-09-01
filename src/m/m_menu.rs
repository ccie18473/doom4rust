#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// m_menu.h
/////////////////////////////
//
// DESCRIPTION:
//	DOOM selection menu, options, episode etc.
//	Sliders and icons. Kinda widget stuff.
//

/////////////////////////////
// m_menu.c
/////////////////////////////
//
// DESCRIPTION:
//	DOOM selection menu, options, episode etc.
//	Sliders and icons. Kinda widget stuff.
//

//
// M_ReadSaveStrings
//  read the strings from the savegame files
//

//
// MENU TYPEDEFS
//
pub struct menuitem_t {
    // 0 = no cursor here, 1 = ok, 2 = arrows ok
    pub status: i16,
    pub name: [u8; 10],
    // choice = menu item #.
    // if status = 2,
    //   choice=0:leftarrow,1:rightarrow
    pub routine: fn(choice: i32),
    // hotkey in menu
    pub alphaKey: u8,
}

pub type menu_s = menu_t;
pub struct menu_t {
    pub numitems: i16,              // # of menu items
    pub prevMenu: *mut menu_s,      // previous menu
    pub menuitems: *mut menuitem_t, // menu items
    pub routine: fn(),              // draw routine
    pub x: i16,
    pub y: i16,      // x,y of menu
    pub lastOn: i16, // last item user was on in menu
}

// graphic name of skulls
// warning: initializer-string for array of chars is too long
pub const skullName: [&str; 2] = ["M_SKULL1", "M_SKULL2"];

//
// M_QuitDOOM
//
pub const quitsounds: [i32; 8] = [
    sfxenum_t::sfx_pldeth as i32,
    sfxenum_t::sfx_dmpain as i32,
    sfxenum_t::sfx_popain as i32,
    sfxenum_t::sfx_slop as i32,
    sfxenum_t::sfx_telept as i32,
    sfxenum_t::sfx_posit1 as i32,
    sfxenum_t::sfx_posit3 as i32,
    sfxenum_t::sfx_sgtatk as i32,
];

pub const quitsounds2: [i32; 8] = [
    sfxenum_t::sfx_vilact as i32,
    sfxenum_t::sfx_getpow as i32,
    sfxenum_t::sfx_boscub as i32,
    sfxenum_t::sfx_slop as i32,
    sfxenum_t::sfx_skeswg as i32,
    sfxenum_t::sfx_kntdth as i32,
    sfxenum_t::sfx_bspact as i32,
    sfxenum_t::sfx_sgtatk as i32,
];

pub const cdetailNames: [&str; 2] = ["M_GDHIGH", "M_GDLOW"];
pub const msgNames: [&str; 2] = ["M_MSGOFF", "M_MSGON"];

pub fn M_ReadSaveStrings() {
    println!("M_ReadSaveStrings");
}

//
// M_LoadGame & Cie.
//
pub fn M_DrawLoad() {
    println!("M_DrawLoad");
}

//
// Draw border for the savegame description
//
pub fn M_DrawSaveLoadBorder(x: i32, y: i32) {
    println!("M_DrawSaveLoadBorder");
}

//
// User wants to load this game
//
pub fn M_LoadSelect(choice: i32) {
    println!("M_LoadSelect");
}

//
// Selected from DOOM menu
//
pub fn M_LoadGame(choice: i32) {
    println!("M_LoadGame");
}

//
//  M_SaveGame & Cie.
//
pub fn M_DrawSave() {
    println!("M_DrawSave");
}

//
// M_Responder calls this when user is finished
//
pub fn M_DoSave(slot: i32) {
    println!("M_DoSave");
}

//
// User wants to save. Start string input for M_Responder
//
pub fn M_SaveSelect(choice: i32) {
    println!("M_SaveSelect");
}

//
// Selected from DOOM menu
//
pub fn M_SaveGame(choice: i32) {
    println!("M_SaveGame");
}

pub fn M_QuickSaveResponse(key: i32) {
    println!("M_QuickSaveResponse");
}

pub fn M_QuickSave() {
    println!("M_QuickSave");
}

//
// M_QuickLoad
//
pub fn M_QuickLoadResponse(key: i32) {
    println!("M_QuickLoadResponse");
}

pub fn M_QuickLoad() {
    println!("M_QuickLoad");
}

//
// Read This Menus
// Had a "quick hack to fix romero bug"
//
pub fn M_DrawReadThis1() {
    println!("M_DrawReadThis1");
}

//
// Read This Menus - optional second page.
//
pub fn M_DrawReadThis2() {
    println!("M_DrawReadThis2");
}

//
// Change Sfx & Music volumes
//
pub fn M_DrawSound() {
    println!("M_DrawSound");
}

pub fn M_Sound(choice: i32) {
    println!("M_Sound");
}

pub fn M_SfxVol(choice: i32) {
    println!("M_SfxVol");
}

pub fn M_MusicVol(choice: i32) {
    println!("M_MusicVol");
}

//
// M_DrawMainMenu
//
pub fn M_DrawMainMenu() {
    println!("M_DrawMainMenu");
}

//
// M_NewGame
//
pub fn M_DrawNewGame() {
    println!("M_DrawNewGame");
}

pub fn M_NewGame(choice: i32) {
    println!("M_NewGame");
}

pub fn M_DrawEpisode() {
    println!("M_DrawEpisode");
}

pub fn M_VerifyNightmare(key: i32) {
    println!("M_VerifyNightmare");
}

pub fn M_ChooseSkill(choice: i32) {
    println!("M_ChooseSkill");
}

pub fn M_Episode(choice: i32) {
    println!("M_Episode");
}

//
// M_Options
//

pub fn M_DrawOptions() {
    println!("M_DrawOptions");
}

pub fn M_Options(choice: i32) {
    println!("M_Options");
}

//
//      Toggle messages on/off
//
pub fn M_ChangeMessages(choice: i32) {
    println!("M_ChangeMessages");
}

//
// M_EndGame
//
pub fn M_EndGameResponse(key: i32) {
    println!("M_EndGameResponse");
}

pub fn M_EndGame(choice: i32) {
    println!("M_EndGame");
}

//
// M_ReadThis
//
pub fn M_ReadThis(choice: i32) {
    println!("M_ReadThis");
}

pub fn M_ReadThis2(choice: i32) {
    println!("M_ReadThis2");
}

pub fn M_FinishReadThis(choice: i32) {
    println!("M_FinishReadThis");
}

pub fn M_QuitResponse(key: i32) {
    println!("M_QuitResponse");
}

pub fn M_SelectEndMessage() -> &'static str {
    println!("M_SelectEndMessage");

    return "";
}

pub fn M_QuitDOOM(choice: i32) {
    println!("M_QuitDOOM");
}

pub fn M_ChangeSensitivity(choice: i32) {
    println!("M_ChangeSensitivity");
}

pub fn M_ChangeDetail(choice: i32) {
    println!("M_ChangeDetail");
}

pub fn M_SizeDisplay(choice: i32) {
    println!("M_SizeDisplay");
}

//
//      Menu Functions
//
pub fn M_DrawThermo(x: i32, y: i32, thermWidth: i32, thermDot: i32) {
    println!("M_DrawThermo");
}

pub fn M_DrawEmptyCell(menu: *mut menu_t, item: i32) {
    println!("M_DrawEmptyCell");
}

pub fn M_DrawSelCell(menu: *mut menu_t, item: i32) {
    println!("M_DrawSelCell");
}

pub fn M_StartMessage(string: &str, routine: *mut libc::c_void, input: bool) {
    println!("M_StartMessage");
}

pub fn M_StopMessage() {
    println!("M_StopMessage");
}

//
// Find string width from hu_font chars
//
pub fn M_StringWidth(string: &str) -> i32 {
    println!("M_StringWidth");

    return 0;
}

//
//      Find string height from hu_font chars
//
pub fn M_StringHeight(string: &str) -> i32 {
    println!("M_StringHeight");

    return 0;
}

//
//      Write a string using the hu_font
//
pub fn M_WriteText(x: i32, y: i32, string: &str) {
    println!("M_WriteText");
}

// These keys evaluate to a "null" key in Vanilla Doom that allows weird
// jumping in the menus. Preserve this behavior for accuracy.

pub fn IsNullKey(key: i32) -> bool {
    println!("IsNullKey");

    return false;
}

//
// CONTROL PANEL
//

//
// M_Responder
//
pub fn M_Responder(ev: *mut event_t) -> bool {
    println!("M_Responder");

    return false;
}

//
// M_StartControlPanel
//
pub fn M_StartControlPanel() {
    println!("M_StartControlPanel");
}

//
// M_Drawer
// Called after the view has been rendered,
// but before it has been blitted.
//
pub fn M_Drawer() {
    println!("M_Drawer");
}

//
// M_ClearMenus
//
pub fn M_ClearMenus() {
    println!("M_ClearMenus");
}

//
// M_SetupNextMenu
//
pub fn M_SetupNextMenu(menudef: *mut menu_t) {
    println!("M_SetupNextMenu");
}

//
// M_Ticker
//
pub fn M_Ticker() {
    println!("M_Ticker");
}

//
// M_Init
//
pub fn M_Init() {
    println!("M_Init");
}
