#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// v_patch.h
/////////////////////////////
//
// DESCRIPTION:
//      Refresh/rendering module, shared data struct definitions.
//

// Patches.
// A patch holds one or more columns.
// Patches are used for sprites and all masked pictures,
// and we compose textures from the TEXTURE1/2 lists
// of patches.

#[repr(C)]
pub struct patch_t {
    pub width: i16, // bounding box size
    pub height: i16,
    pub leftoffset: i16, // pixels to the left of origin
    pub topoffset: i16,  // pixels below the origin
    //BUG was size 8 now MAXWIDTH
    pub columnofs: [i32; MAXWIDTH as usize], // only [width] used
                                             // the [0] is &columnofs[width]
}

// posts are runs of non masked source pixels
#[repr(C)]
pub struct post_t {
    pub topdelta: u8, // -1 is the last post in a column
    pub length: u8,   // length data bytes follows
}

// column_t is a list of 0 or more post_t, (byte)-1 terminated
pub type column_t = post_t;
