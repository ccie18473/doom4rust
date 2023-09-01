#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// r_defs.h
/////////////////////////////
//
// DESCRIPTION:
//      Refresh/rendering module, shared data struct definitions.
//

// Silhouette, needed for clipping Segs (mainly)
// and sprites representing things.
pub const SIL_NONE: u8 = 0;
pub const SIL_BOTTOM: u8 = 1;
pub const SIL_TOP: u8 = 2;
pub const SIL_BOTH: u8 = 3;

pub const MAXDRAWSEGS: i32 = 256;

//
// INTERNAL MAP TYPES
//  used by play and refresh
//

//
// Your plain vanilla vertex.
// Note: transformed values not buffered locally,
//  like some DOOM-alikes ("wt", "WebView") did.
//
pub struct vertex_t {
    pub x: i32,
    pub y: i32,
}

// Forward of LineDefs, for Sectors.

// Each sector has a degenmobj_t in its center
//  for sound origin purposes.
// I suppose this does not handle sound from
//  moving objects (doppler), because
//  position is prolly just buffered, not
//  updated.
pub struct degenmobj_t {
    pub thinker: thinker_t, // not used for anything
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

//
// The SECTORS record, at runtime.
// Stores things/mobjs.
//
pub struct sector_t {
    pub floorheight: i32,
    pub ceilingheight: i32,
    pub floorpic: u8,
    pub ceilingpic: u8,
    pub lightlevel: u8,
    pub special: u8,
    pub tag: u8,

    // 0 = untraversed, 1,2 = sndlines -1
    pub soundtraversed: i32,

    // thing that made a sound (or null)
    pub soundtarget: *mut mobj_t,

    // mapblock bounding box for height changes
    pub blockbox: [i32; 4],

    // origin for any sounds played by the sector
    pub soundorg: degenmobj_t,

    // if == validcount, already checked
    pub validcount: i32,

    // list of mobjs in sector
    pub thinglist: *mut mobj_t,

    // thinker_t for reversable actions
    pub specialdata: *mut libc::c_void,

    pub linecount: i32,
    pub lines: *mut *mut line_s, // [linecount] size
}

//
// The SideDef.
//

pub struct side_t {
    // add this to the calculated texture column
    pub textureoffset: i32,

    // add this to the calculated texture top
    pub rowoffset: i32,

    // Texture indices.
    // We do not maintain names here.
    pub toptexture: i16,
    pub bottomtexture: i16,
    pub midtexture: i16,

    // Sector the SideDef is facing.
    pub sector: *mut sector_t,
}

//
// Move clipping aid for LineDefs.
//
pub enum slopetype_t {
    ST_HORIZONTAL,
    ST_VERTICAL,
    ST_POSITIVE,
    ST_NEGATIVE,
}

pub type line_s = line_t;
pub struct line_t {
    // Vertices, from v1 to v2.
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,

    // Precalculated v2 - v1 for side checking.
    pub dx: i32,
    pub dy: i32,

    // Animation related.
    pub flags: i16,
    pub special: i16,
    pub tag: i16,

    // Visual appearance: SideDefs.
    //  sidenum[1] will be -1 if one sided
    pub sidenum: [i16; 2],

    // Neat. Another bounding box, for the extent
    //  of the LineDef.
    pub bbox: [i32; 4],

    // To aid move clipping.
    pub slopetype: slopetype_t,

    // Front and back sector.
    // Note: redundant? Can be retrieved from SideDefs.
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,

    // if == validcount, already checked
    pub validcount: i32,

    // thinker_t for reversable actions
    pub specialdata: *mut libc::c_void,
}

//
// A SubSector.
// References a Sector.
// Basically, this is a list of LineSegs,
//  indicating the visible walls that define
//  (all or some) sides of a convex BSP leaf.
//

pub type subsector_s = subsector_t;
pub struct subsector_t {
    pub sector: *mut sector_t,
    pub numlines: i16,
    pub firstline: i16,
}

//
// The LineSeg.
//
pub struct seg_t {
    pub v1: *mut vertex_t,
    pub v2: *mut vertex_t,

    pub offset: i32,

    pub angle: u32,

    pub sidedef: *mut side_t,
    pub linedef: *mut line_t,

    // Sector references.
    // Could be retrieved from linedef, too.
    // backsector is NULL for one sided lines
    pub frontsector: *mut sector_t,
    pub backsector: *mut sector_t,
}

//
// BSP node.
//
pub struct node_t {
    // Partition line.
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,

    // Bounding box for each child.
    pub bbox: [[i32; 2]; 4],

    // If NF_SUBSECTOR its a subsector.
    pub children: [u16; 2],
}

// PC direct to screen pointers
//B UNUSED - keep till detailshift in r_draw.c resolved
//extern byte*	destview;
//extern byte*	destscreen;

//
// OTHER TYPES
//

// This could be wider for >8 bit display.
// Indeed, true color support is posibble
//  precalculating 24bpp lightmap/colormap LUT.
//  from darkening PLAYPAL to all black.
// Could even us emore than 32 levels.

pub type lighttable_t = byte;

//
// ?
//

pub type drawseg_s = drawseg_t;
pub struct drawseg_t {
    pub curline: seg_t,
    pub x1: i32,
    pub x2: i32,

    pub scale1: i32,
    pub scale2: i32,
    pub scalestep: i32,

    // 0=none, 1=bottom, 2=top, 3=both
    pub silhouette: i32,

    // do not clip sprites above this
    pub bsilheight: i32,

    // do not clip sprites below this
    pub tsilheight: i32,

    // Pointers to lists for sprite clipping,
    //  all three adjusted so [x1] is first value.
    pub sprtopclip: *mut u8,
    pub sprbottomclip: *mut u8,
    pub maskedtexturecol: *mut u8,
}

// A vissprite_t is a thing
//  that will be drawn during a refresh.
// I.e. a sprite object that is partly visible.

pub type vissprite_s = vissprite_t;

#[derive(Copy, Clone)]
pub struct vissprite_t {
    // Doubly linked list.
    pub prev: *mut vissprite_s,
    pub next: *mut vissprite_s,

    pub x1: i32,
    pub x2: i32,

    // for line side calculation
    pub gx: i32,
    pub gy: i32,

    // global bottom / top for silhouette clipping
    pub gz: i32,
    pub gzt: i32,

    // horizontal position of x1
    pub startfrac: i32,

    pub scale: i32,

    // negative if flipped
    pub xiscale: i32,

    pub texturemid: i32,
    pub patch: i32,

    // for color translation and shadow draw,
    //  maxbright frames as well
    pub colormap: *mut u8,

    pub mobjflags: i32,
}

impl vissprite_t {
    pub fn new() -> Self {
        Self {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            x1: 0,
            x2: 0,
            gx: 0,
            gy: 0,
            gz: 0,
            gzt: 0,
            startfrac: 0,
            scale: 0,
            xiscale: 0,
            texturemid: 0,
            patch: 0,
            colormap: ptr::null_mut(),
            mobjflags: 0,
        }
    }
}

//
// Sprites are patches with a special naming convention
//  so they can be recognized by R_InitSprites.
// The base name is NNNNFx or NNNNFxFx, with
//  x indicating the rotation, x = 0, 1-7.
// The sprite and frame specified by a thing_t
//  is range checked at run time.
// A sprite is a patch_t that is assumed to represent
//  a three dimensional object and may have multiple
//  rotations pre drawn.
// Horizontal flipping is used to save space,
//  thus NNNNF2F5 defines a mirrored patch.
// Some sprites will only have one picture used
// for all views: NNNNF0
//
#[derive(Copy, Clone)]
pub struct spriteframe_t {
    // If false use 0 for any position.
    // Note: as eight entries are available,
    //  we might as well insert the same name eight times.
    pub rotate: bool,

    // Lump to use for view angles 0-7.
    pub lump: [u8; 8],

    // Flip bit (1 = flip) to use for view angles 0-7.
    pub flip: [u8; 8],
}

impl spriteframe_t {
    pub fn new() -> Self {
        Self {
            rotate: false,
            lump: [0; 8],
            flip: [0; 8],
        }
    }
}

//
// A sprite definition:
//  a number of animation frames.
//
pub struct spritedef_t {
    pub numframes: i32,
    pub spriteframes: *mut spriteframe_t,
}

//
// Now what is a visplane, anyway?
//
pub struct visplane_t {
    pub height: i32,
    pub picnum: i32,
    pub lightlevel: i32,
    pub minx: i32,
    pub maxx: i32,

    // leave pads for [minx-1]/[maxx+1]
    pub pad1: u8,
    // Here lies the rub for all
    //  dynamic resize/change of resolution.
    pub top: [u8; SCREENWIDTH as usize],
    pub pad2: u8,
    pub pad3: u8,
    // See above.
    pub bottom: [u8; SCREENWIDTH as usize],
    pub pad4: u8,
}
