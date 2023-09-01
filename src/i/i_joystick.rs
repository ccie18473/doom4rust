#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// i_joystick.c
/////////////////////////////
//
// DESCRIPTION:
//       SDL Joystick code.
//

pub fn I_ShutdownJoystick() {
    println!("I_ShutdownJoystick");
}

pub fn IsValidAxis(axis: i32) -> bool {
    println!("IsValidAxis");

    return false;
}

pub fn I_InitJoystick() {
    println!("I_InitJoystick");
}

pub fn IsAxisButton(physbutton: i32) -> bool {
    println!("IsAxisButton");

    return false;
}

// Get the state of the given virtual button.

pub fn ReadButtonState(vbutton: i32) -> i32 {
    println!("ReadButtonState");

    return 0;
}

// Get a bitmask of all currently-pressed buttons

pub fn GetButtonsState() -> i32 {
    println!("GetButtonsState");

    return 0;
}

// Read the state of an axis, inverting if necessary.

pub fn GetAxisState(axis: i32, invert: i32) -> i32 {
    println!("GetAxisState");

    return 0;
}

pub fn I_UpdateJoystick() {
    println!("I_UpdateJoystick");
}

pub fn I_BindJoystickVariables() {
    println!("I_BindJoystickVariables");
}
