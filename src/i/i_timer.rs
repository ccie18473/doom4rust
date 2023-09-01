#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// i_timer.h
/////////////////////////////
//
// DESCRIPTION:
//      System-specific timer interface
//

pub const TICRATE: i32 = 35;

/////////////////////////////
// i_timer.c
/////////////////////////////
//
// DESCRIPTION:
//      Timer functions.
//

pub fn I_GetTicks(doom: &mut modules) -> i32 {
    println!("I_GetTicks");

    return doom.i.timer.ticks() as i32;
}

pub fn I_GetTime(doom: &mut modules) -> i32 {
    println!("I_GetTime");

    let mut ticks: u32;

    ticks = I_GetTicks(doom) as u32;

    if doom.i.basetime == 0 {
        doom.i.basetime = ticks;
    }

    ticks -= doom.i.basetime;

    return (ticks as i32 * TICRATE) / 1000;
}

//
// Same as I_GetTime, but returns time in milliseconds
//
pub fn I_GetTimeMS(doom: &mut modules) -> i32 {
    println!("I_GetTimeMS");

    let ticks: u32;

    ticks = I_GetTicks(doom) as u32;

    if doom.i.basetime == 0 {
        doom.i.basetime = ticks;
    }

    return (ticks - doom.i.basetime) as i32;
}

// Sleep for a specified number of ms
pub fn I_Sleep(doom: &mut modules, ms: i32) {
    println!("I_Sleep");

    doom.i.timer.delay(ms as u32);
}

pub fn I_WaitVBL(count: i32) {
    println!("I_WaitVBL");

    //I_Sleep((count * 1000) / 70);
}

pub fn I_InitTimer() {
    println!("I_InitTimer");

    println!("I_InitTimer: Setting up timer.");
    //if (SDL_Init(SDL_INIT_TIMER) < 0)
    //{
    //      printf("SDL_Init failed: %s\n", SDL_GetError());
    //      atexit(SDL_Quit);
    //      exit(1);
    //}
}
