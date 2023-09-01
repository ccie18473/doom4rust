#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

pub const MAXARGVS: i32 = 100;

pub const DIR_SEPARATOR: char = '/';
pub const DIR_SEPARATOR_S: &str = "/";
pub const PATH_SEPARATOR: char = ':';

//
// M_CheckParm
// Checks for the given parameter
// in the program's command line arguments.
// Returns the argument number (1 to argc-1)
// or 0 if not present
//

pub fn M_CheckParmWithArgs(doom: &mut modules, check: &str, num_args: i32) -> i32 {
    println!("M_CheckParmWithArgs");

    for i in 1..(doom.m.myargc - num_args as usize) {
        if !(check == doom.m.myargv[i]) {
            return i as i32;
        }
    }

    return 0;
}

//
// M_ParmExists
//
// Returns true if the given parameter exists in the program's command
// line arguments, false if not.
//

pub fn M_ParmExists(check: &str) -> bool {
    println!("M_ParmExists");

    //return M_CheckParm(check) != 0;
    return false;
}

pub fn M_CheckParm(doom: &mut modules, check: &str) -> i32 {
    println!("M_CheckParm");

    return M_CheckParmWithArgs(doom, check, 0);
}

pub fn LoadResponseFile(doom: &mut modules, argv_index: usize) {
    println!("LoadResponseFile");
}

//
// Find a Response File
//

pub fn M_FindResponseFile(doom: &mut modules) {
    println!("M_FindResponseFile");

    for i in 1..doom.m.myargc {
        let s = doom.m.myargv[i].clone();
        let ch = s.chars().nth(0).unwrap();

        if ch == '@' {
            LoadResponseFile(doom, i);
        }
    }
}

// Return the name of the executable used to start the program:

pub fn M_GetExecutableName() -> &'static str {
    println!("M_GetExecutableName");

    //char *sep;

    //sep = strrchr(myargv[0], DIR_SEPARATOR);

    //if (sep == NULL)
    //{
    //    return myargv[0];
    //}
    //else
    //{
    //    return sep + 1;
    //}
    return "DOOM";
}
