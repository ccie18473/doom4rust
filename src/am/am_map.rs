#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// am_map.h
/////////////////////////////
//
// DESCRIPTION:
//  AutoMap module.
//

// Used by ST StatusBar stuff.
pub const AM_MSGHEADER: u32 = (('a' as u32) << 24) + (('m' as u32) << 16);
pub const AM_MSGENTERED: u32 = AM_MSGHEADER | (('e' as u32) << 8);
pub const AM_MSGEXITED: u32 = AM_MSGHEADER | (('x' as u32) << 8);

/////////////////////////////
// am_map.c
/////////////////////////////
//
// DESCRIPTION:  the automap code
//

// For use if I do walls with outsides/insides
pub const REDS: i32 = 256 - 5 * 16;
pub const REDRANGE: i32 = 16;
pub const BLUES: i32 = 256 - 4 * 16 + 8;
pub const BLUERANGE: i32 = 8;
pub const GREENS: i32 = 7 * 16;
pub const GREENRANGE: i32 = 16;
pub const GRAYS: i32 = 6 * 16;
pub const GRAYSRANGE: i32 = 16;
pub const BROWNS: i32 = 4 * 16;
pub const BROWNRANGE: i32 = 16;
pub const YELLOWS: i32 = 256 - 32 + 7;
pub const YELLOWRANGE: i32 = 1;
pub const BLACK: i32 = 0;
pub const WHITE: i32 = 256 - 47;

// Automap colors
pub const BACKGROUND: i32 = BLACK;
pub const YOURCOLORS: i32 = WHITE;
pub const YOURRANGE: i32 = 0;
pub const WALLCOLORS: i32 = REDS;
pub const WALLRANGE: i32 = REDRANGE;
pub const TSWALLCOLORS: i32 = GRAYS;
pub const TSWALLRANGE: i32 = GRAYSRANGE;
pub const FDWALLCOLORS: i32 = BROWNS;
pub const FDWALLRANGE: i32 = BROWNRANGE;
pub const CDWALLCOLORS: i32 = YELLOWS;
pub const CDWALLRANGE: i32 = YELLOWRANGE;
pub const THINGCOLORS: i32 = GREENS;
pub const THINGRANGE: i32 = GREENRANGE;
pub const SECRETWALLCOLORS: i32 = WALLCOLORS;
pub const SECRETWALLRANGE: i32 = WALLRANGE;
pub const GRIDCOLORS: i32 = GRAYS + GRAYSRANGE / 2;
pub const GRIDRANGE: i32 = 0;
pub const XHAIRCOLORS: i32 = GRAYS;

// drawing stuff

pub const AM_NUMMARKPOINTS: usize = 10;

/*
// scale on entry
pub const INITSCALEMTOF (.2*FRACUNIT)
// how much the automap moves window per tic in frame-buffer coordinates
// moves 140 pixels in 1 second
pub const F_PANINC	4
// how much zoom-in per tic
// goes to 2x in 1 second
pub const M_ZOOMIN        ((int) (1.02*FRACUNIT))
// how much zoom-out per tic
// pulls out to 0.5x in 1 second
pub const M_ZOOMOUT       ((int) (FRACUNIT/1.02))

// translates between frame-buffer and map distances
pub const FTOM(x) FixedMul(((x)<<16),scale_ftom)
pub const MTOF(x) (FixedMul((x),scale_mtof)>>16)
// translates between frame-buffer and map coordinates
pub const CXMTOF(x)  (f_x + MTOF((x)-m_x))
pub const CYMTOF(y)  (f_y + (f_h - MTOF((y)-m_y)))

// the following is crap
pub const LINE_NEVERSEE ML_DONTDRAW

*/
pub struct fpoint_t {
    pub x: i32,
    pub y: i32,
}

pub struct fline_t {
    pub a: fpoint_t,
    pub b: fpoint_t,
}
#[derive(Copy, Clone)]
pub struct mpoint_t {
    pub x: i32,
    pub y: i32,
}
impl mpoint_t {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

pub struct mline_t {
    pub a: mpoint_t,
    pub b: mpoint_t,
}

pub struct islope_t {
    pub slp: i32,
    pub islp: i32,
}

/*
//
// The vector graphics for the automap.
//  A line drawing of the player pointing right,
//   starting from the middle.
//
pub const R ((8*PLAYERRADIUS)/7)
mline_t player_arrow[] = {
    { { -R+R/8, 0 }, { R, 0 } }, // -----
    { { R, 0 }, { R-R/2, R/4 } },  // ----->
    { { R, 0 }, { R-R/2, -R/4 } },
    { { -R+R/8, 0 }, { -R-R/8, R/4 } }, // >---->
    { { -R+R/8, 0 }, { -R-R/8, -R/4 } },
    { { -R+3*R/8, 0 }, { -R+R/8, R/4 } }, // >>--->
    { { -R+3*R/8, 0 }, { -R+R/8, -R/4 } }
};
#undef R

pub const R ((8*PLAYERRADIUS)/7)
mline_t cheat_player_arrow[] = {
    { { -R+R/8, 0 }, { R, 0 } }, // -----
    { { R, 0 }, { R-R/2, R/6 } },  // ----->
    { { R, 0 }, { R-R/2, -R/6 } },
    { { -R+R/8, 0 }, { -R-R/8, R/6 } }, // >----->
    { { -R+R/8, 0 }, { -R-R/8, -R/6 } },
    { { -R+3*R/8, 0 }, { -R+R/8, R/6 } }, // >>----->
    { { -R+3*R/8, 0 }, { -R+R/8, -R/6 } },
    { { -R/2, 0 }, { -R/2, -R/6 } }, // >>-d--->
    { { -R/2, -R/6 }, { -R/2+R/6, -R/6 } },
    { { -R/2+R/6, -R/6 }, { -R/2+R/6, R/4 } },
    { { -R/6, 0 }, { -R/6, -R/6 } }, // >>-dd-->
    { { -R/6, -R/6 }, { 0, -R/6 } },
    { { 0, -R/6 }, { 0, R/4 } },
    { { R/6, R/4 }, { R/6, -R/7 } }, // >>-ddt->
    { { R/6, -R/7 }, { R/6+R/32, -R/7-R/32 } },
    { { R/6+R/32, -R/7-R/32 }, { R/6+R/10, -R/7 } }
};
#undef R

pub const R (FRACUNIT)
mline_t triangle_guy[] = {
    { { (fixed_t)(-.867*R), (fixed_t)(-.5*R) }, { (fixed_t)(.867*R ), (fixed_t)(-.5*R) } },
    { { (fixed_t)(.867*R ), (fixed_t)(-.5*R) }, { (fixed_t)(0      ), (fixed_t)(R    ) } },
    { { (fixed_t)(0      ), (fixed_t)(R    ) }, { (fixed_t)(-.867*R), (fixed_t)(-.5*R) } }
};
#undef R

pub const R (FRACUNIT)
mline_t thintriangle_guy[] = {
    { { (fixed_t)(-.5*R), (fixed_t)(-.7*R) }, { (fixed_t)(R    ), (fixed_t)(0    ) } },
    { { (fixed_t)(R    ), (fixed_t)(0    ) }, { (fixed_t)(-.5*R), (fixed_t)(.7*R ) } },
    { { (fixed_t)(-.5*R), (fixed_t)(.7*R ) }, { (fixed_t)(-.5*R), (fixed_t)(-.7*R) } }
};
#undef R
*/

// Calculates the slope and slope according to the x-axis of a line
// segment in map coordinates (with the upright y-axis n' all) so
// that it can be used with the brain-dead drawing stuff.

pub fn AM_getIslope(ml: *mut mline_t, is: *mut islope_t) {
    println!("AM_getIslope");
}

//
//
//
pub fn AM_activateNewScale() {
    println!("AM_activateNewScale");
}

//
//
//
pub fn AM_saveScaleAndLoc() {
    println!("AM_saveScaleAndLoc");
}

//
//
//
pub fn AM_restoreScaleAndLoc() {
    println!("AM_restoreScaleAndLoc");
}

//
// adds a marker at the current location
//
pub fn AM_addMark() {
    println!("AM_addMark");
}

//
// Determines bounding box of all vertices,
// sets global variables controlling zoom range.
//
pub fn AM_findMinMaxBoundaries() {
    println!("AM_findMinMaxBoundaries");
}

//
//
//
pub fn AM_changeWindowLoc() {
    println!("AM_changeWindowLoc");
}

//
//
//
pub fn AM_initVariables() {
    println!("AM_initVariables");
}

//
//
//
pub fn AM_loadPics() {
    println!("AM_loadPics");
}

pub fn AM_unloadPics() {
    println!("AM_unloadPics");
}

pub fn AM_clearMarks() {
    println!("AM_clearMarks");
}

//
// should be called at the start of every level
// right now, i figure it out myself
//
pub fn AM_LevelInit() {
    println!("AM_LevelInit");
}

//
//
//
pub fn AM_Stop() {
    println!("AM_Stop");
}

//
//
//
pub fn AM_Start() {
    println!("AM_Start");
}

//
// set the window scale to the maximum size
//
pub fn AM_minOutWindowScale() {
    println!("AM_minOutWindowScale");
}

//
// set the window scale to the minimum size
//
pub fn AM_maxOutWindowScale() {
    println!("AM_maxOutWindowScale");
}

//
// Handle events (user inputs) in automap mode
//
pub fn AM_Responder(ev: *mut event_t) -> bool {
    println!("AM_Responder");

    return false;
}

//
// Zooming
//
pub fn AM_changeWindowScale() {
    println!("AM_changeWindowScale");
}

//
//
//
pub fn AM_doFollowPlayer() {
    println!("AM_doFollowPlayer");
}

//
//
//
pub fn AM_updateLightLev() {
    println!("AM_updateLightLev");
}

//
// Updates on Game Tick
//
pub fn AM_Ticker() {
    println!("AM_Ticker");
}

//
// Clear automap frame buffer.
//
pub fn AM_clearFB(color: i32) {
    println!("AM_clearFB");
}

//
// Automap clipping of lines.
//
// Based on Cohen-Sutherland clipping algorithm but with a slightly
// faster reject and precalculated slopes.  If the speed is needed,
// use a hash algorithm to handle  the common cases.
//
pub fn AM_clipMline(ml: *mut mline_t, fl: *mut fline_t) -> bool {
    println!("AM_clipMline");

    return true;
}

//
// Classic Bresenham w/ whatever optimizations needed for speed
//
pub fn AM_drawFline(fl: *mut fline_t, color: i32) {
    println!("AM_drawFline");
}

//
// Clip lines, draw visible part sof lines.
//
pub fn AM_drawMline(ml: *mut mline_t, color: i32) {
    println!("AM_drawMline");
}

//
// Draws flat (floor/ceiling tile) aligned grid lines.
//
pub fn AM_drawGrid(color: i32) {
    println!("AM_drawGrid");
}

//
// Determines visible lines, draws them.
// This is LineDef based, not LineSeg based.
//
pub fn AM_drawWalls() {
    println!("AM_drawWalls");
}

//
// Rotation in 2D.
// Used to rotate player arrow line character.
//
pub fn AM_rotate(x: *mut i32, y: *mut i32, a: angle_t) {
    println!("AM_rotate");
}

pub fn AM_drawLineCharacter(
    lineguy: *mut mline_t,
    lineguylines: i32,
    scale: i32,
    angle: angle_t,
    color: i32,
    x: i32,
    y: i32,
) {
    println!("AM_drawLineCharacter");
}

pub fn AM_drawPlayers() {
    println!("AM_drawPlayers");
}

pub fn AM_drawThings(colors: i32, colorrange: i32) {
    println!("AM_drawThings");
}

pub fn AM_drawMarks() {
    println!("AM_drawMarks");
}

pub fn AM_drawCrosshair(color: i32) {
    println!("AM_drawCrosshair");
}

pub fn AM_Drawer() {
    println!("AM_Drawer");
}
