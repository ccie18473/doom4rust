#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

/////////////////////////////
// statdump.h
/////////////////////////////

/////////////////////////////
// statdump.c
/////////////////////////////
/*
Functions for presenting the information captured from the statistics
 buffer to a file.
 */

/* Par times for E1M1-E1M9. */
pub const doom1_par_times: [i32; 9] = [30, 75, 120, 90, 165, 180, 180, 30, 165];

/* Par times for MAP01-MAP09. */
pub const doom2_par_times: [i32; 9] = [30, 90, 120, 120, 90, 150, 120, 120, 270];

pub fn StatCopy(stats: *mut wbstartstruct_t) {
    println!("StatCopy");
}

pub fn StatDump(doom: &mut modules) {
    println!("StatDump");
}

/////////////////////////////
// Statdump
/////////////////////////////

pub struct statdump {}

impl statdump {
    pub fn new() -> Self {
        Self {}
    }
}
