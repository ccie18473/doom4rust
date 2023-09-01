#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/////////////////////////////
// deh_str.h
/////////////////////////////
//
// Dehacked string replacements
//

// Used to do dehacked text substitutions throughout the program

pub fn DEH_String(x: &str) -> &str {
    x
}
pub fn DEH_printf(s: &str) {
    print!("{}", s);
}
pub fn DEH_fprintf(s: &str) {
    print!("{}", s);
}
pub fn DEH_snprintf(s: &str) {
    print!("{}", s);
}
pub fn DEH_AddStringReplacement<'a>(mut x: &'a str, y: &'a str) {
    x = y;
}
