#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// w_file.h
/////////////////////////////
//
// DESCRIPTION:
//	WAD I/O functions.
//

pub type _wad_file_s = wad_file_t;

#[repr(C)]
pub struct wad_file_class_t {
    // Open a file for reading.
    pub OpenFile: fn(doom: &mut modules, path: &str) -> *mut wad_file_t,
    // Close the specified file.
    pub CloseFile: fn(file: *mut wad_file_t),
    // Read data from the specified position in the file into the
    // provided buffer.  Returns the number of bytes read.
    pub Read:
        fn(file: *mut wad_file_t, offset: u32, buffer: &mut Vec<u8>, buffer_len: usize) -> usize,
}

#[repr(C)]
pub struct wad_file_t {
    // Class of this file.
    pub file_class: *mut wad_file_class_t,

    // If this is NULL, the file cannot be mapped into memory.  If this
    // is non-NULL, it is a pointer to the mapped file.
    pub mapped: *mut u8,

    // Length of the file, in bytes.
    pub length: u32,
}

impl wad_file_t {
    pub fn new() -> Self {
        Self {
            file_class: ptr::null_mut(),
            mapped: ptr::null_mut(),
            length: 0,
        }
    }
}

/////////////////////////////
// w_file.c
/////////////////////////////
//
// DESCRIPTION:
//	WAD I/O functions.
//

pub fn W_OpenFile(doom: &mut modules, path: &str) -> *mut wad_file_t {
    println!("W_OpenFile");

    let mut result: *mut wad_file_t;

    //
    // Use the OS's virtual memory subsystem to map WAD files
    // directly into memory.
    //

    if M_CheckParm(doom, "-mmap") == 0 {
        return (doom.w.stdc_wad_file.OpenFile)(doom, path);
    }

    // Try all classes in order until we find one that works

    result = ptr::null_mut();

    for i in 0..doom.w.wad_file_classes.len() {
        unsafe { result = ((*doom.w.wad_file_classes[i]).OpenFile)(doom, path) };

        if result != ptr::null_mut() {
            break;
        }
    }
    return result;
}

pub fn W_CloseFile(wad: *mut wad_file_t) {
    println!("W_CloseFile");
}

pub fn W_Read(wad: *mut wad_file_t, offset: u32, buffer: &mut Vec<u8>, buffer_len: usize) -> usize {
    println!("W_Read");
    unsafe {
        return ((*(*wad).file_class).Read)(wad, offset, buffer, buffer_len);
    }
}
