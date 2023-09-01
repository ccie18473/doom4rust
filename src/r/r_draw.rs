#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// r_draw.h
/////////////////////////////
//
// DESCRIPTION:
//	System specific interface stuff.
//

/////////////////////////////
// r_draw.c
/////////////////////////////
//
// DESCRIPTION:
//	The actual span/column drawing functions.
//	Here find the main potential for optimization,
//	 e.g. inline assembly, different algorithms.
//

// ?
pub const MAXWIDTH: i32 = 1120;
pub const MAXHEIGHT: i32 = 832;

// status bar height at bottom of screen
pub const SBARHEIGHT: u8 = 32;

pub const FUZZTABLE: u8 = 50;
pub const FUZZOFF: i32 = SCREENWIDTH;

//
// Spectre/Invisibility.
//

pub const fuzzoffset: [i32; FUZZTABLE as usize] = [
    FUZZOFF, -FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF,
    FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF,
    -FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF,
    -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, -FUZZOFF, -FUZZOFF,
    -FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF, FUZZOFF, -FUZZOFF, FUZZOFF,
];

//
// A column is a vertical slice/span from a wall texture that,
//  given the DOOM style restrictions on the view orientation,
//  will always have constant z depth.
// Thus a special case loop for very fast rendering can
//  be used. It has also been used with Wolfenstein 3D.
//
pub fn R_DrawColumn() {
    println!("R_DrawColumn");
}

pub fn R_DrawColumnLow() {
    println!("R_DrawColumnLow");
}

//
// Framebuffer postprocessing.
// Creates a fuzzy image by copying pixels
//  from adjacent ones to left and right.
// Used with an all black colormap, this
//  could create the SHADOW effect,
//  i.e. spectres and invisible players.
//
pub fn R_DrawFuzzColumn() {
    println!("R_DrawFuzzColumn");
}

// low detail mode version

pub fn R_DrawFuzzColumnLow() {
    println!("R_DrawFuzzColumnLow");
}

//
// R_DrawTranslatedColumn
// Used to draw player sprites
//  with the green colorramp mapped to others.
// Could be used with different translation
//  tables, e.g. the lighter colored version
//  of the BaronOfHell, the HellKnight, uses
//  identical sprites, kinda brightened up.
//

pub fn R_DrawTranslatedColumn() {
    println!("R_DrawTranslatedColumn");
}

pub fn R_DrawTranslatedColumnLow() {
    println!("R_DrawTranslatedColumnLow");
}

//
// R_InitTranslationTables
// Creates the translation tables to map
//  the green color ramp to gray, brown, red.
// Assumes a given structure of the PLAYPAL.
// Could be read from a lump instead.
//
pub fn R_InitTranslationTables() {
    println!("R_InitTranslationTables");
}

//
// R_DrawSpan
// With DOOM style restrictions on view orientation,
//  the floors and ceilings consist of horizontal slices
//  or spans with constant z depth.
// However, rotation around the world z axis is possible,
//  thus this mapping, while simpler and faster than
//  perspective correct texture mapping, has to traverse
//  the texture at an angle in all but a few cases.
// In consequence, flats are not stored by column (like walls),
//  and the inner loop has to step in texture space u and v.
//

//
// Draws the actual span.
pub fn R_DrawSpan() {
    println!("R_DrawSpan");
}

//
// Again..
//
pub fn R_DrawSpanLow() {
    println!("R_DrawSpanLow");
}

//
// R_InitBuffer
// Creats lookup tables that avoid
//  multiplies and other hazzles
//  for getting the framebuffer address
//  of a pixel to draw.
//
pub fn R_InitBuffer(width: i32, height: i32) {
    println!("R_InitBuffer");
}

//
// R_FillBackScreen
// Fills the back screen with a pattern
//  for variable screen sizes
// Also draws a beveled edge.
//
pub fn R_FillBackScreen() {
    println!("R_FillBackScreen");
}

//
// Copy a screen buffer.
//
pub fn R_VideoErase(ofs: u32, count: i32) {
    println!("R_VideoErase");
}

//
// R_DrawViewBorder
// Draws the border around the view
//  for different size windows?
//
pub fn R_DrawViewBorder() {
    println!("R_DrawViewBorder");
}
