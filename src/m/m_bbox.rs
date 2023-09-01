#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// m_bbox.h
/////////////////////////////
//
// DESCRIPTION:
//    Nil.
//

// Bounding box coordinate storage.
pub enum BOX {
    BOXTOP,
    BOXBOTTOM,
    BOXLEFT,
    BOXRIGHT,
} // bbox coordinates

/////////////////////////////
// m_bbox.c
/////////////////////////////
//
// DESCRIPTION:
//	Main loop menu stuff.
//	Random number LUT.
//	Default Config File.
//	PCX Screenshots.
//

pub fn M_ClearBox(BOX: &mut [i32; 4]) {
    println!("M_ClearBox");

    BOX[BOX::BOXTOP as usize] = libc::INT_MIN;
    BOX[BOX::BOXRIGHT as usize] = libc::INT_MIN;
    BOX[BOX::BOXBOTTOM as usize] = libc::INT_MAX;
    BOX[BOX::BOXLEFT as usize] = libc::INT_MAX;
}

pub fn M_AddToBox(BOX: &mut [i32; 4], x: fixed_t, y: fixed_t) {
    println!("M_AddToBox");

    if x < BOX[BOX::BOXLEFT as usize] {
        BOX[BOX::BOXLEFT as usize] = x;
    } else if x > BOX[BOX::BOXRIGHT as usize] {
        BOX[BOX::BOXRIGHT as usize] = x;
    }
    if y < BOX[BOX::BOXBOTTOM as usize] {
        BOX[BOX::BOXBOTTOM as usize] = y;
    } else if y > BOX[BOX::BOXTOP as usize] {
        BOX[BOX::BOXTOP as usize] = y;
    }
}
