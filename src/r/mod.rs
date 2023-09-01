#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod r_bsp;
pub mod r_data;
pub mod r_defs;
pub mod r_draw;
pub mod r_main;
pub mod r_plane;
pub mod r_segs;
pub mod r_sky;
pub mod r_things;

pub use r_bsp::*;
pub use r_data::*;
pub use r_defs::*;
pub use r_draw::*;
pub use r_main::*;
pub use r_plane::*;
pub use r_segs::*;
pub use r_sky::*;
pub use r_things::*;

use crate::*;

/////////////////////////////
// R_* Rendering engine
/////////////////////////////

pub struct r<'a> {
    /////////////////////////////
    // r_data.c
    /////////////////////////////
    pub firstflat: i32,
    pub lastflat: i32,
    pub numflats: i32,

    pub firstpatch: i32,
    pub lastpatch: i32,
    pub numpatches: i32,

    pub firstspritelump: i32,
    pub lastspritelump: i32,
    pub numspritelumps: i32,

    pub numtextures: i32,
    pub textures: *mut *mut texture_t,
    pub textures_hashtable: *mut *mut texture_t,

    pub texturewidthmask: *mut i32,
    // needed for texture pegging
    pub textureheight: *mut fixed_t,
    pub texturecompositesize: *mut i32,
    pub texturecolumnlump: *mut *mut i16,
    pub texturecolumnofs: *mut *mut u16,
    pub texturecomposite: *mut *mut u8,

    // for global animation
    pub flattranslation: *mut i32,
    pub texturetranslation: *mut i32,

    // needed for pre rendering
    pub spritewidth: *mut fixed_t,
    pub spriteoffset: *mut fixed_t,
    pub spritetopoffset: *mut fixed_t,

    pub colormaps: *mut lighttable_t,

    pub flatmemory: i32,
    pub texturememory: i32,
    pub spritememory: i32,
    /////////////////////////////
    // r_draw.c
    /////////////////////////////

    //
    // All drawing to the view buffer is accomplished in this file.
    // The other refresh files only know about ccordinates,
    //  not the architecture of the frame buffer.
    // Conveniently, the frame buffer is a linear one,
    //  and we need only the base address,
    //  and the total size == width*height*depth/8.,
    //
    pub viewimage: *mut u8,
    pub viewwidth: i32,
    pub scaledviewwidth: i32,
    pub viewheight: i32,
    pub viewwindowx: i32,
    pub viewwindowy: i32,
    pub ylookup: [*mut u8; MAXHEIGHT as usize],
    pub columnofs: [i32; MAXWIDTH as usize],

    // Color tables for different players,
    //  translate a limited part to another
    //  (color ramps used for  suit colors).
    //
    pub translations: [[u8; 3]; 256],

    // Backing buffer containing the bezel drawn around the screen and
    // surrounding background.
    pub background_buffer: *mut u8,

    //
    // R_DrawColumn
    // Source is the top of the column to scale.
    //
    pub dc_colormap: *mut u8,
    pub dc_x: i32,
    pub dc_yl: i32,
    pub dc_yh: i32,
    pub dc_iscale: i32,
    pub dc_texturemid: i32,

    // first pixel in a column (possibly virtual)
    pub dc_source: *mut u8,

    // just for profiling
    pub dccount: i32,

    pub ds_y: i32,
    pub ds_x1: i32,
    pub ds_x2: i32,

    pub ds_colormap: *mut lighttable_t,

    pub ds_xfrac: fixed_t,
    pub ds_yfrac: fixed_t,
    pub ds_xstep: fixed_t,
    pub ds_ystep: fixed_t,

    // start of a 64*64 tile image
    pub ds_source: *mut u8,

    // just for profiling
    pub dscount: i32,

    pub fuzzpos: i32, // = 0;

    pub dc_translation: *mut u8,
    pub translationtables: *mut u8,

    /////////////////////////////
    // r_main.c
    /////////////////////////////
    pub viewangleoffset: i32,
    // increment every time a check is made
    pub validcount: i32, // = 1;
    pub fixedcolormap: *mut lighttable_t,
    pub centerx: i32,
    pub centery: i32,

    pub centerxfrac: fixed_t,
    pub centeryfrac: fixed_t,
    pub projection: fixed_t,

    // just for profiling purposes
    pub framecount: i32,
    pub sscount: i32,
    pub linecount: i32,
    pub loopcount: i32,

    pub viewx: fixed_t,
    pub viewy: fixed_t,
    pub viewz: fixed_t,
    pub viewangle: angle_t,

    pub viewcos: fixed_t,
    pub viewsin: fixed_t,

    pub viewplayer: *mut player_t,

    // 0 = high, 1 = low
    pub detailshift: i32,

    //
    // precalculated math tables
    //
    pub clipangle: angle_t,

    // The viewangletox[viewangle + FINEANGLES/4] lookup
    // maps the visible view angles to screen X coordinates,
    // flattening the arc to a flat projection plane.
    // There will be many angles mapped to the same X.
    pub viewangletox: [i32; FINEANGLES as usize / 2],

    // The xtoviewangleangle[] table maps a screen pixel
    // to the lowest viewangle that maps back to x ranges
    // from clipangle to -clipangle.
    pub xtoviewangle: [angle_t; SCREENWIDTH as usize + 1],

    pub scalelight: [[*mut lighttable_t; LIGHTLEVELS]; MAXLIGHTSCALE],
    pub scalelightfixed: [*mut lighttable_t; MAXLIGHTSCALE],
    pub zlight: [[*mut lighttable_t; LIGHTLEVELS]; MAXLIGHTZ],

    // bumped light from gun blasts
    pub extralight: i32,

    pub setsizeneeded: bool,
    pub setblocks: i32,
    pub setdetail: bool,
    /////////////////////////////
    // r_things.c
    /////////////////////////////
    //
    // Sprite rotation 0 is facing the viewer,
    //  rotation 1 is one angle turn CLOCKWISE around the axis.
    // This is not the same as the angle,
    //  which increases counter clockwise (protractor).
    // There was a lot of stuff grabbed wrong, so I changed it...
    //
    pub pspritescale: fixed_t,
    pub pspriteiscale: fixed_t,

    pub spritelights: *mut *mut lighttable_t,

    // constant arrays
    //  used for psprite clipping and initializing clipping
    pub negonearray: [i16; SCREENWIDTH as usize],
    pub screenheightarray: [i16; SCREENWIDTH as usize],

    // variables used to look up
    //  and range check thing_t sprites patches
    pub sprites: *mut spritedef_t,
    pub numsprites: i32,

    pub sprtemp: [spriteframe_t; 29],
    pub maxframe: i32,
    pub spritename: &'a str,

    pub vissprites: [vissprite_t; MAXVISSPRITES as usize],
    pub vissprite_p: *mut vissprite_t,
    pub newvissprite: i32,
    pub overflowsprite: vissprite_t,
    pub mfloorclip: *mut i16,
    pub mceilingclip: *mut i16,

    pub spryscale: fixed_t,
    pub sprtopscreen: fixed_t,
    pub vsprsortedhead: vissprite_t,
    pub clipbot: [i16; SCREENWIDTH as usize],
    pub cliptop: [i16; SCREENWIDTH as usize],
}

impl<'a> r<'a> {
    pub fn new() -> Self {
        Self {
            /////////////////////////////
            // r_data.c
            /////////////////////////////
            firstflat: 0,
            lastflat: 0,
            numflats: 0,
            firstpatch: 0,
            lastpatch: 0,
            numpatches: 0,
            firstspritelump: 0,
            lastspritelump: 0,
            numspritelumps: 0,
            numtextures: 0,
            textures: ptr::null_mut(),
            textures_hashtable: ptr::null_mut(),
            texturewidthmask: ptr::null_mut(),
            textureheight: ptr::null_mut(),
            texturecompositesize: ptr::null_mut(),
            texturecolumnlump: ptr::null_mut(),
            texturecolumnofs: ptr::null_mut(),
            texturecomposite: ptr::null_mut(),
            flattranslation: ptr::null_mut(),
            texturetranslation: ptr::null_mut(),
            spritewidth: ptr::null_mut(),
            spriteoffset: ptr::null_mut(),
            spritetopoffset: ptr::null_mut(),
            colormaps: ptr::null_mut(),
            flatmemory: 0,
            texturememory: 0,
            spritememory: 0,
            /////////////////////////////
            // r_draw.c
            /////////////////////////////
            viewimage: ptr::null_mut(),
            viewwidth: 0,
            scaledviewwidth: 0,
            viewheight: 0,
            viewwindowx: 0,
            viewwindowy: 0,
            ylookup: [ptr::null_mut(); MAXHEIGHT as usize],
            columnofs: [0; MAXWIDTH as usize],
            translations: [[0; 3]; 256],
            background_buffer: ptr::null_mut(),
            dc_colormap: ptr::null_mut(),
            dc_x: 0,
            dc_yl: 0,
            dc_yh: 0,
            dc_iscale: 0,
            dc_texturemid: 0,
            dc_source: ptr::null_mut(),
            dccount: 0,
            ds_y: 0,
            ds_x1: 0,
            ds_x2: 0,
            ds_colormap: ptr::null_mut(),
            ds_xfrac: 0,
            ds_yfrac: 0,
            ds_xstep: 0,
            ds_ystep: 0,
            ds_source: ptr::null_mut(),
            dscount: 0,
            fuzzpos: 0,
            dc_translation: ptr::null_mut(),
            translationtables: ptr::null_mut(),
            /////////////////////////////
            // r_main.c
            /////////////////////////////
            viewangleoffset: 0,
            validcount: 1,
            fixedcolormap: ptr::null_mut(),
            centerx: 0,
            centery: 0,
            centerxfrac: 0,
            centeryfrac: 0,
            projection: 0,
            framecount: 0,
            sscount: 0,
            linecount: 0,
            loopcount: 0,
            viewx: 0,
            viewy: 0,
            viewz: 0,
            viewangle: 0,
            viewcos: 0,
            viewsin: 0,
            viewplayer: ptr::null_mut(),
            detailshift: 0,
            clipangle: 0,
            viewangletox: [0; FINEANGLES as usize / 2],
            xtoviewangle: [0; SCREENWIDTH as usize + 1],
            scalelight: [[ptr::null_mut(); LIGHTLEVELS]; MAXLIGHTSCALE],
            scalelightfixed: [ptr::null_mut(); MAXLIGHTSCALE],
            zlight: [[ptr::null_mut(); LIGHTLEVELS]; MAXLIGHTZ],
            extralight: 0,
            setsizeneeded: false,
            setblocks: 0,
            setdetail: false,
            /////////////////////////////
            // r_things.c
            /////////////////////////////
            pspritescale: 0,
            pspriteiscale: 0,
            spritelights: ptr::null_mut(),
            negonearray: [0; SCREENWIDTH as usize],
            screenheightarray: [0; SCREENWIDTH as usize],
            sprites: ptr::null_mut(),
            numsprites: 0,
            sprtemp: [spriteframe_t::new(); 29],
            maxframe: 0,
            spritename: "",
            vissprites: [vissprite_t::new(); MAXVISSPRITES as usize],
            vissprite_p: ptr::null_mut(),
            newvissprite: 0,
            overflowsprite: vissprite_t::new(),
            mfloorclip: ptr::null_mut(),
            mceilingclip: ptr::null_mut(),
            spryscale: 0,
            sprtopscreen: 0,
            vsprsortedhead: vissprite_t::new(),
            clipbot: [0; SCREENWIDTH as usize],
            cliptop: [0; SCREENWIDTH as usize],
        }
    }
}
