#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// v_video.h
/////////////////////////////
//
// DESCRIPTION:
//	Gamma correction LUT.
//	Functions to draw patches (by post) directly to screen.
//	Functions to blit a block to the screen.
//
//
// VIDEO
//

pub const CENTERY: i32 = SCREENHEIGHT / 2;

// haleyjd 08/28/10: implemented for Strife support
// haleyjd 08/28/10: Patch clipping callback, implemented to support Choco
// Strife.
pub type vpatchclipfunc_t = fn(*mut patch_t, i32, i32) -> bool;

/////////////////////////////
// v_video.c
/////////////////////////////
//
// DESCRIPTION:
//	Gamma correction LUT stuff.
//	Functions to draw patches (by post) directly to screen.
//	Functions to blit a block to the screen.
//

// TODO: There are separate RANGECHECK defines for different games, but this
// is common code. Fix this.
pub const RANGECHECK: bool = false;

//
// SCREEN SHOTS
//
#[repr(C)]
pub struct pcx_t {
    pub manufacturer: char,
    pub version: char,
    pub encoding: char,
    pub bits_per_pixel: char,

    pub xmin: u16,
    pub ymin: u16,
    pub xmax: u16,
    pub ymax: u16,

    pub hres: u16,
    pub vres: u16,

    pub palette: [u8; 48],

    pub reserved: char,
    pub color_planes: char,
    pub bytes_per_line: u16,
    pub palette_type: u16,

    pub filler: [char; 58],
    pub data: char, // unbounded
}

pub const MOUSE_SPEED_BOX_WIDTH: u8 = 120;
pub const MOUSE_SPEED_BOX_HEIGHT: u8 = 9;

//
// V_MarkRect
//
pub fn V_MarkRect(doom: &mut modules, x: i32, y: i32, width: i32, height: i32) {
    println!("V_MarkRect");

    // If we are temporarily using an alternate screen, do not
    // affect the update box.

    if doom.v.dest_screen == doom.i.I_VideoBuffer {
        M_AddToBox(&mut doom.v.dirtybox, x, y);
        M_AddToBox(&mut doom.v.dirtybox, x + width - 1, y + height - 1);
    }
}

//
// V_CopyRect
//
pub fn V_CopyRect(
    srcx: i32,
    srcy: i32,
    source: *mut u8,
    width: i32,
    height: i32,
    destx: i32,
    desty: i32,
) {
    println!("V_CopyRect");
}

//
// V_SetPatchClipCallback
//
// haleyjd 08/28/10: Added for Strife support.
// By calling this function, you can setup runtime error checking for patch
// clipping. Strife never caused errors by drawing patches partway off-screen.
// Some versions of vanilla DOOM also behaved differently than the default
// implementation, so this could possibly be extended to those as well for
// accurate emulation.
//
pub fn V_SetPatchClipCallback(doom: &mut modules, func: vpatchclipfunc_t) {
    println!("V_SetPatchClipCallback");

    doom.v.patchclip_callback = func;
}

//
// V_DrawPatch
// Masks a column based masked pic to the screen.
//

pub fn V_DrawPatch(doom: &mut modules, mut x: i32, mut y: i32, patch: *mut patch_t) {
    println!("V_DrawPatch");

    let mut count: i32;
    let mut col: i32;
    let mut column: *mut column_t;
    let mut desttop: *mut u8;
    let mut dest: *mut u8;
    let mut source: *mut u8;
    let w: i32;

    unsafe {
        y -= SHORT((*patch).topoffset) as i32;
        x -= SHORT((*patch).leftoffset) as i32;

        // haleyjd 08/28/10: Strife needs silent error checking here.
        if doom.v.patchclip_callback != func {
            if !(doom.v.patchclip_callback)(patch, x, y) {
                return;
            }
        }

        if x < 0
            || x + SHORT((*patch).width) as i32 > SCREENWIDTH
            || y < 0
            || y + SHORT((*patch).height) as i32 > SCREENHEIGHT
        {
            println!("Bad V_DrawPatch x={} y={} patch.width={} patch.height={} topoffset={} leftoffset={}", x, y, (*patch).width, (*patch).height, (*patch).topoffset, (*patch).leftoffset);
            I_Error();
        }

        V_MarkRect(
            doom,
            x,
            y,
            SHORT((*patch).width) as i32,
            SHORT((*patch).height) as i32,
        );

        col = 0;
        desttop = doom.v.dest_screen.add((y * SCREENWIDTH + x) as usize);

        w = SHORT((*patch).width) as i32;

        while col < w {
            column = (patch as *mut u8).add(LONG((*patch).columnofs[col as usize]) as usize)
                as *mut column_t;

            // step through the posts in a column
            while (*column).topdelta != 0xff {
                source = (column as *mut u8).add(3);
                dest = desttop.add(((*column).topdelta as i32 * SCREENWIDTH) as usize);
                count = (*column).length as i32;

                while count > -3 {
                    *dest = *source;
                    source = source.add(1);
                    dest = dest.add(SCREENWIDTH as usize);

                    count -= 1;
                }
                column = (column as *mut u8).add((*column).length as usize + 4) as *mut column_t;
            }
            x += 1;
            col += 1;
            desttop = desttop.add(1);
        }
    }
}

//
// V_DrawPatchFlipped
// Masks a column based masked pic to the screen.
// Flips horizontally, e.g. to mirror face.
//

pub fn V_DrawPatchFlipped(x: i32, y: i32, patch: *mut patch_t) {
    println!("V_DrawPatchFlipped");
}

//
// V_DrawPatchDirect
// Draws directly to the screen on the pc.
//

pub fn V_DrawPatchDirect(x: i32, y: i32, patch: *mut patch_t) {
    println!("V_DrawPatchDirect");
}

//
// V_DrawTLPatch
//
// Masks a column based translucent masked pic to the screen.
//

pub fn V_DrawTLPatch(x: i32, y: i32, patch: *mut patch_t) {
    println!("V_DrawTLPatch");
}

//
// V_DrawXlaPatch
//
// villsa [STRIFE] Masks a column based translucent masked pic to the screen.
//

pub fn V_DrawXlaPatch(x: i32, y: i32, patch: *mut patch_t) {
    println!("V_DrawXlaPatch");
}

//
// V_DrawAltTLPatch
//
// Masks a column based translucent masked pic to the screen.
//

pub fn V_DrawAltTLPatch(x: i32, y: i32, patch: *mut patch_t) {
    println!("V_DrawAltTLPatch");
}

//
// V_DrawShadowedPatch
//
// Masks a column based masked pic to the screen.
//

pub fn V_DrawShadowedPatch(x: i32, y: i32, patch: *mut patch_t) {
    println!("V_DrawShadowedPatch");
}

//
// Load tint table from TINTTAB lump.
//

pub fn V_LoadTintTable() {
    println!("V_LoadTintTable");
}

//
// V_LoadXlaTable
//
// villsa [STRIFE] Load xla table from XLATAB lump.
//

pub fn V_LoadXlaTable() {
    println!("V_LoadXlaTable");
}

//
// V_DrawBlock
// Draw a linear block of pixels into the view buffer.
//

pub fn V_DrawBlock(x: i32, y: i32, width: i32, height: i32, src: *mut u8) {
    println!("V_DrawBlock");
}

pub fn V_DrawFilledBox(x: i32, y: i32, w: i32, h: i32, c: i32) {
    println!("V_DrawFilledBox");
}

pub fn V_DrawHorizLine(x: i32, y: i32, w: i32, c: i32) {
    println!("V_DrawHorizLine");
}

pub fn V_DrawVertLine(x: i32, y: i32, h: i32, c: i32) {
    println!("V_DrawVertLine");
}

pub fn V_DrawBox(x: i32, y: i32, w: i32, h: i32, c: i32) {
    println!("V_DrawBox");
}

//
// Draw a "raw" screen (lump containing raw data to blit directly
// to the screen)
//

pub fn V_DrawRawScreen(raw: *mut u8) {
    println!("V_DrawRawScreen");
}

//
// V_Init
//
pub fn V_Init() {
    println!("V_Init");
}

// Set the buffer that the code draws to.

pub fn V_UseBuffer(buffer: *mut u8) {
    println!("V_UseBuffer");
}

// Restore screen buffer to the i_video screen buffer.

pub fn V_RestoreBuffer(doom: &mut modules) {
    println!("V_RestoreBuffer");

    doom.v.dest_screen = doom.i.I_VideoBuffer;
}

//
// WritePCXfile
//

pub fn WritePCXfile(filename: String, data: *mut u8, width: i32, height: i32, palette: *mut u8) {
    println!("WritePCXfile");
}

//
// V_ScreenShot
//

pub fn V_ScreenShot(format: String) {
    println!("V_ScreenShot");
}

pub fn V_DrawMouseSpeedBox(speed: i32) {
    println!("V_DrawMouseSpeedBox");
}
