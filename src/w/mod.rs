#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub mod w_checksum;
pub mod w_file;
pub mod w_file_stdc;
pub mod w_main;
pub mod w_wad;

pub use w_checksum::*;
pub use w_file::*;
pub use w_file_stdc::*;
pub use w_main::*;
pub use w_wad::*;

use crate::*;

/////////////////////////////
// W_* WAD file loading
/////////////////////////////

pub struct w<'a> {
    /////////////////////////////
    // w_checksum.c
    /////////////////////////////
    pub open_wadfiles: *mut *mut wad_file_t,
    pub num_open_wadfiles: i32,
    /////////////////////////////
    // w_file.c
    /////////////////////////////
    pub wad_file_classes: [*mut wad_file_class_t; 10],
    /////////////////////////////
    // w_file_stdc.c
    /////////////////////////////
    pub stdc_wad_file: wad_file_class_t,
    /////////////////////////////
    // w_wad.c
    /////////////////////////////
    // Location of each lump on disk.
    pub lumpinfo: *mut lumpinfo_t<'a>,
    pub numlumps: u32,
    // Hash table for fast lookups
    pub lumphash: *mut *mut lumpinfo_t<'a>,
}

impl<'a> w<'a> {
    pub fn new() -> Self {
        let mut stdc_wad_file: wad_file_class_t = wad_file_class_t {
            OpenFile: W_StdC_OpenFile,
            CloseFile: W_StdC_CloseFile,
            Read: W_StdC_Read,
        };
        Self {
            /////////////////////////////
            // w_checksum.c
            /////////////////////////////
            open_wadfiles: ptr::null_mut(),
            num_open_wadfiles: 0,
            /////////////////////////////
            // w_file.c
            /////////////////////////////
            wad_file_classes: [&mut stdc_wad_file; 10],
            /////////////////////////////
            // w_file_stdc.c
            /////////////////////////////
            stdc_wad_file,
            /////////////////////////////
            // w_wad.c
            /////////////////////////////
            lumpinfo: ptr::null_mut(),
            numlumps: 0,
            lumphash: ptr::null_mut(),
        }
    }
}
