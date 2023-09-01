#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_maputl.c
/////////////////////////////
//
// DESCRIPTION:
//	Movement/collision utility functions,
//	as used by function in p_map.c.
//	BLOCKMAP Iterator functions,
//	and some PIT_* functions to use for iteration.
//

//
// P_AproxDistance
// Gives an estimation of distance (not exact)
//

pub fn P_AproxDistance(dx: fixed_t, dy: fixed_t) -> fixed_t {
    println!("P_AproxDistance");

    return 0;
}

//
// P_PointOnLineSide
// Returns 0 or 1
//
pub fn P_PointOnLineSide(x: fixed_t, y: fixed_t, line: *mut line_t) -> i32 {
    println!("P_PointOnLineSide");

    return 0;
}

//
// P_BoxOnLineSide
// Considers the line to be infinite
// Returns side 0 or 1, -1 if box crosses the line.
//
pub fn P_BoxOnLineSide(tmbox: *mut fixed_t, ld: *mut line_t) -> i32 {
    println!("P_BoxOnLineSide");

    return 0;
}

//
// P_PointOnDivlineSide
// Returns 0 or 1.
//
pub fn P_PointOnDivlineSide(x: fixed_t, y: fixed_t, line: *mut divline_t) -> i32 {
    println!("P_PointOnDivlineSide");

    return 0;
}

//
// P_MakeDivline
//
pub fn P_MakeDivline(li: *mut line_t, dl: *mut divline_t) {
    println!("P_MakeDivline");
}

//
// P_InterceptVector
// Returns the fractional intercept point
// along the first divline.
// This is only called by the addthings
// and addlines traversers.
//
pub fn P_InterceptVector(v2: *mut divline_t, v1: *mut divline_t) -> fixed_t {
    println!("P_InterceptVector");

    return 0;
}

//
// P_LineOpening
// Sets opentop and openbottom to the window
// through a two sided line.
// OPTIMIZE: keep this precalculated
//

pub fn P_LineOpening(linedef: *mut line_t) {
    println!("P_LineOpening");
}

//
// THING POSITION SETTING
//

//
// P_UnsetThingPosition
// Unlinks a thing from block map and sectors.
// On each position change, BLOCKMAP and other
// lookups maintaining lists ot things inside
// these structures need to be updated.
//
pub fn P_UnsetThingPosition(thing: *mut mobj_t) {
    println!("P_UnsetThingPosition");
}

//
// P_SetThingPosition
// Links a thing into both a block and a subsector
// based on it's x y.
// Sets thing->subsector properly
//
pub fn P_SetThingPosition(thing: *mut mobj_t) {
    println!("P_SetThingPosition");
}

//
// BLOCK MAP ITERATORS
// For each line/thing in the given mapblock,
// call the passed PIT_* function.
// If the function returns false,
// exit with false without checking anything else.
//

//
// P_BlockLinesIterator
// The validcount flags are used to avoid checking lines
// that are marked in multiple mapblocks,
// so increment validcount before the first call
// to P_BlockLinesIterator, then make one or more calls
// to it.
//
pub fn P_BlockLinesIterator(x: i32, y: i32 /*,boolean(*func)(line_t*)*/) -> bool {
    println!("P_BlockLinesIterator");

    return false;
}

//
// P_BlockThingsIterator
//
pub fn P_BlockThingsIterator(x: i32, y: i32 /*,boolean(*func)(mobj_t*)*/) -> bool {
    println!("P_BlockThingsIterator");

    return false;
}

//
// PIT_AddLineIntercepts.
// Looks for lines in the given block
// that intercept the given trace
// to add to the intercepts list.
//
// A line is crossed if its endpoints
// are on opposite sides of the trace.
// Returns true if earlyout and a solid line hit.
//
pub fn PIT_AddLineIntercepts(ld: *mut line_t) -> bool {
    println!("PIT_AddLineIntercepts");

    return false;
}

//
// PIT_AddThingIntercepts
//
pub fn PIT_AddThingIntercepts(thing: *mut mobj_t) -> bool {
    println!("PIT_AddThingIntercepts");

    return false;
}

//
// P_TraverseIntercepts
// Returns true if the traverser function returns true
// for all lines.
//
pub fn P_TraverseIntercepts(func: traverser_t, maxfrac: fixed_t) -> bool {
    println!("P_TraverseIntercepts");

    return false;
}

// Overwrite a specific memory location with a value.

pub fn InterceptsMemoryOverrun(location: i32, value: i32) {
    println!("InterceptsMemoryOverrun");
}

// Emulate overruns of the intercepts[] array.

pub fn InterceptsOverrun(num_intercepts: i32, intercept: *mut intercept_t) {
    println!("InterceptsOverrun");
}

//
// P_PathTraverse
// Traces a line from x1,y1 to x2,y2,
// calling the traverser function for each.
// Returns true if the traverser function returns true
// for all lines.
//
pub fn P_PathTraverse(
    x1: fixed_t,
    y1: fixed_t,
    x2: fixed_t,
    y2: fixed_t,
    flags: i32, /*,boolean (*trav) (intercept_t *)*/
) -> bool {
    println!("P_PathTraverse");

    return false;
}
