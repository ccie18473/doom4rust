#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// p_sight.c
/////////////////////////////
//
// DESCRIPTION:
//	LineOfSight/Visibility checks, uses REJECT Lookup Table.
//

//
// P_DivlineSide
// Returns side 0 (front), 1 (back), or 2 (on).
//
pub fn P_DivlineSide(x: fixed_t, y: fixed_t, node: *mut divline_t) -> i32 {
    println!("P_DivlineSide");

    return 0;
}

//
// P_InterceptVector2
// Returns the fractional intercept point
// along the first divline.
// This is only called by the addthings and addlines traversers.
//
pub fn P_InterceptVector2(v2: *mut divline_t, v1: *mut divline_t) -> fixed_t {
    println!("P_InterceptVector2");

    return 0;
}

//
// P_CrossSubsector
// Returns true
//  if strace crosses the given subsector successfully.
//
pub fn P_CrossSubsector(num: i32) -> bool {
    println!("P_CrossSubsector");

    return false;
}

//
// P_CrossBSPNode
// Returns true
//  if strace crosses the given node successfully.
//
pub fn P_CrossBSPNode(bspnum: i32) -> bool {
    println!("P_CrossBSPNode");

    return false;
}

//
// P_CheckSight
// Returns true
//  if a straight line between t1 and t2 is unobstructed.
// Uses REJECT.
//
pub fn P_CheckSight(t1: *mut mobj_t, t2: *mut mobj_t) -> bool {
    println!("P_CheckSight");

    return false;
}
