#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

// Called to set the source and destination buffers before doing the
// scale.

pub fn I_InitScale(_src_buffer: *mut u8, _dest_buffer: *mut u8, _dest_pitch: i32) {
    println!("I_InitScale");
}

//
// Pixel doubling scale-up functions.
//

// 1x scale doesn't really do any scaling: it just copies the buffer
// a line at a time for when pitch != SCREENWIDTH (!native_surface)

pub fn I_Scale1x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Scale1x");

    return false;
}

// 2x scale (640x400)

pub fn I_Scale2x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Scale2x");

    return false;
}

// 3x scale (960x600)

pub fn I_Scale3x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Scale3x");

    return false;
}

// 4x scale (1280x800)

pub fn I_Scale4x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Scale4x");

    return false;
}

// 5x scale (1600x1000)

pub fn I_Scale5x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Scale5x");

    return false;
}

// Search through the given palette, finding the nearest color that matches
// the given color.

pub fn FindNearestColor(palette: *mut u8, r: i32, g: i32, b: i32) -> i32 {
    println!("FindNearestColor");

    return 0;
}

// Create a stretch table.  This is a lookup table for blending colors.
// pct specifies the bias between the two colors: 0 = all y, 100 = all x.
// NB: This is identical to the lookup tables used in other ports for
// translucency.

pub fn GenerateStretchTable(palette: *mut u8, pct: i32) -> *mut u8 {
    println!("GenerateStretchTable");

    return ptr::null_mut();
}

// Called at startup to generate the lookup tables for aspect ratio
// correcting scale up.

pub fn I_InitStretchTables(palette: *mut u8) {
    println!("I_InitStretchTables");
}

// Create 50%/50% table for 800x600 squash mode

pub fn I_InitSquashTable(palette: *mut u8) {
    println!("I_InitSquashTable");
}

// Destroy the scaling lookup tables. This should only ever be called
// if switching to a completely different palette from the normal one
// (in which case the mappings no longer make any sense).

pub fn I_ResetScaleTables(palette: *mut u8) {
    println!("I_ResetScaleTables");
}

//
// Aspect ratio correcting scale up functions.
//
// These double up pixels to stretch the screen when using a 4:3
// screen mode.
//

pub fn WriteBlendedLine1x(dest: *mut u8, src1: *mut u8, src2: *mut u8, stretch_table: *mut u8) {
    println!("WriteBlendedLine1x");
}

// 1x stretch (320x240)

pub fn I_Stretch1x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Stretch1x");

    return false;
}

pub fn WriteLine2x(dest: *mut u8, src: *mut u8) {
    println!("WriteLine2x");
}

pub fn WriteBlendedLine2x(dest: *mut u8, src1: *mut u8, src2: *mut u8, stretch_table: *mut u8) {
    println!("WriteBlendedLine2x");
}

// 2x stretch (640x480)

pub fn I_Stretch2x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Stretch2x");

    return false;
}

pub fn WriteLine3x(dest: *mut u8, src: *mut u8) {
    println!("WriteLine3x");
}

pub fn WriteBlendedLine3x(dest: *mut u8, src1: *mut u8, src2: *mut u8, stretch_table: *mut u8) {
    println!("WriteBlendedLine3x");
}

// 3x stretch (960x720)

pub fn I_Stretch3x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Stretch3x");

    return false;
}

pub fn WriteLine4x(dest: *mut u8, src: *mut u8) {
    println!("WriteLine4x");
}

pub fn WriteBlendedLine4x(dest: *mut u8, src1: *mut u8, src2: *mut u8, stretch_table: *mut u8) {
    println!("WriteBlendedLine4x");
}

// 4x stretch (1280x960)

pub fn I_Stretch4x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Stretch4x");

    return false;
}

pub fn WriteLine5x(dest: *mut u8, src: *mut u8) {
    println!("WriteLine5x");
}

// 5x stretch (1600x1200)

pub fn I_Stretch5x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Stretch5x");

    return false;
}

//
// Aspect ratio correcting "squash" functions.
//
// These do the opposite of the "stretch" functions above: while the
// stretch functions increase the vertical dimensions, the squash
// functions decrease the horizontal dimensions for the same result.
//
// The same blend tables from the stretch functions are reused; as
// a result, the dimensions are *slightly* wrong (eg. 320x200 should
// squash to 266x200, but actually squashes to 256x200).
//

//
// 1x squashed scale (256x200)
//

pub fn WriteSquashedLine1x(dest: *mut u8, src: *mut u8) {
    println!("WriteSquashedLine1x");
}

// 1x squashed (256x200)

pub fn I_Squash1x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Squash1x");

    return false;
}

//
// 2x squashed scale (512x400)
//

pub fn WriteSquashedLine2x(dest: *mut u8, src: *mut u8) {
    println!("WriteSquashedLine2x");
}

// 2x squash (512x400)

pub fn I_Squash2x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Squash2x");

    return false;
}

pub fn WriteSquashedLine3x(dest: *mut u8, src: *mut u8) {
    println!("WriteSquashedLine3x");
}

//
// 3x scale squashed (800x600)
//
// This is a special case that uses the half_stretch_table (50%) rather
// than the normal stretch_tables(20,40%), to scale up to 800x600
// exactly.
//

pub fn I_Squash3x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Squash3x");

    return false;
}

pub fn WriteSquashedLine4x(dest: *mut u8, src: *mut u8) {
    println!("WriteSquashedLine4x");
}

//
// 4x squashed (1024x800)
//

pub fn I_Squash4x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Squash4x");

    return false;
}

pub fn WriteSquashedLine5x(dest: *mut u8, src: *mut u8) {
    println!("WriteSquashedLine5x");
}

//
// 5x squashed (1280x1000)
//

pub fn I_Squash5x(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    println!("I_Squash5x");

    return false;
}
