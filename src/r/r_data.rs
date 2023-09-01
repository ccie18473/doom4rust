#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// r_data.h
/////////////////////////////
//
// DESCRIPTION:
//  Refresh module, data I/O, caching, retrieval of graphics
//  by name.
//

/////////////////////////////
// r_data.c
/////////////////////////////
//
// DESCRIPTION:
//	Preparation of data for rendering,
//	generation of lookups, caching, retrieval by name.
//

//
// Graphics.
// DOOM graphics for walls and sprites
// is stored in vertical runs of opaque pixels (posts).
// A column is composed of zero or more posts,
// a patch or sprite is composed of zero or more columns.
//

//
// Texture definition.
// Each texture is composed of one or more patches,
// with patches being lumps stored in the WAD.
// The lumps are referenced by number, and patched
// into the rectangular texture space using origin
// and possibly other attributes.
//
#[repr(C)]
pub struct mappatch_t {
    pub originx: i16,
    pub originy: i16,
    pub patch: i16,
    pub stepdir: i16,
    pub colormap: i16,
}

//
// Texture definition.
// A DOOM wall texture is a list of patches
// which are to be combined in a predefined order.
//
#[repr(C)]
pub struct maptexture_t {
    pub name: [u8; 8],
    pub masked: i32,
    pub width: i16,
    pub height: i16,
    pub obsolete: i32,
    pub patchcount: i16,
    pub patches: [mappatch_t; 1],
}

// A single patch from a texture definition,
//  basically a rectangular area within
//  the texture rectangle.
pub struct texpatch_t {
    // Block origin (allways UL),
    // which has allready accounted
    // for the internal origin of the patch.
    pub originx: i16,
    pub originy: i16,
    pub patch: i32,
}

// A maptexturedef_t describes a rectangular texture,
//  which is composed of one or more mappatch_t structures
//  that arrange graphic patches.

pub type texture_t = texture_s;

pub struct texture_s {
    // Keep name for switch changing, etc.
    pub name: [u8; 8],
    pub width: i16,
    pub height: i16,
    // Index in textures list
    pub index: i32,
    // Next in hash table chain
    pub next: *mut texture_t,
    // All the patches[patchcount]
    //  are drawn back to front into the cached texture.
    pub patchcount: i16,
    pub patches: [texpatch_t; 1],
}

//
// MAPTEXTURE_T CACHING
// When a texture is first needed,
//  it counts the number of composite columns
//  required in the texture and allocates space
//  for a column directory and any new columns.
// The directory will simply point inside other patches
//  if there is only one patch in a given column,
//  but any columns with multiple patches
//  will have new column_ts generated.
//

//
// R_DrawColumnInCache
// Clip and draw a column
//  from a patch into a cached post.
//
pub fn R_DrawColumnInCache(patch: *mut column_t, cache: *mut u8, originy: i32, cacheheight: i32) {
    println!("R_DrawColumnInCache");
}

//
// R_GenerateComposite
// Using the texture definition,
//  the composite texture is created from the patches,
//  and each column is cached.
//
pub fn R_GenerateComposite(texnum: i32) {
    println!("R_GenerateComposite");
}

//
// R_GenerateLookup
//
pub fn R_GenerateLookup(texnum: i32) {
    println!("R_GenerateLookup");
}

//
// R_GetColumn
//
pub fn R_GetColumn(tex: i32, col: i32) -> *mut u8 {
    println!("R_GetColumn");

    return ptr::null_mut();
}

pub fn GenerateTextureHashTable() {
    println!("GenerateTextureHashTable");
}

//
// R_InitTextures
// Initializes the texture list
//  with the textures from the world map.
//
pub fn R_InitTextures() {
    println!("R_InitTextures");
}

//
// R_InitFlats
//
pub fn R_InitFlats() {
    println!("R_InitFlats");
}

//
// R_InitSpriteLumps
// Finds the width and hoffset of all sprites in the wad,
//  so the sprite does not need to be cached completely
//  just for having the header info ready during rendering.
//
pub fn R_InitSpriteLumps() {
    println!("R_InitSpriteLumps");
}

//
// R_InitColormaps
//
pub fn R_InitColormaps() {
    println!("R_InitColormaps");
}

//
// R_InitData
// Locates all the lumps
//  that will be used by all views
// Must be called after W_Init.
//
pub fn R_InitData() {
    println!("R_InitData");

    R_InitTextures();
    print!(".");
    R_InitFlats();
    print!(".");
    R_InitSpriteLumps();
    print!(".");
    R_InitColormaps();
}

//
// R_FlatNumForName
// Retrieval, get a flat number for a flat name.
//
pub fn R_FlatNumForName(name: &str) -> i32 {
    println!("R_FlatNumForName");

    return 0;
}

//
// R_CheckTextureNumForName
// Check whether texture is available.
// Filter out NoTexture indicator.
//
pub fn R_CheckTextureNumForName(name: &str) -> i32 {
    println!("R_CheckTextureNumForName");

    return 0;
}

//
// R_TextureNumForName
// Calls R_CheckTextureNumForName,
//  aborts with error message.
//
pub fn R_TextureNumForName(name: &str) -> i32 {
    println!("R_TextureNumForName");

    return 0;
}

//
// R_PrecacheLevel
// Preloads all relevant graphics for the level.
//

pub fn R_PrecacheLevel() {
    println!("R_PrecacheLevel");
}
