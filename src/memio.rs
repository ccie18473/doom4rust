#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

/////////////////////////////
// memio.h
/////////////////////////////

pub type MEMFILE<'a> = _MEMFILE<'a>;

pub enum mem_rel_t {
    MEM_SEEK_SET,
    MEM_SEEK_CUR,
    MEM_SEEK_END,
}

/////////////////////////////
// memio.c
/////////////////////////////
//
// Emulates the IO functions in C stdio.h reading and writing to
// memory.
//

pub enum memfile_mode_t {
    MODE_READ,
    MODE_WRITE,
}

pub struct _MEMFILE<'a> {
    pub buf: &'a str,
    pub buflen: usize,
    pub alloced: usize,
    pub position: u32,
    pub mode: memfile_mode_t,
}

// Open a memory area for reading

pub fn mem_fopen_read<'a>(buf: *mut libc::c_void, buflen: usize) -> *mut MEMFILE<'a> {
    println!("mem_fopen_read");

    return ptr::null_mut();
}

// Read bytes

pub fn mem_fread(buf: *mut libc::c_void, size: usize, nmemb: usize, stream: *mut MEMFILE) -> usize {
    println!("mem_fread");

    return 0;
}

// Open a memory area for writing

pub fn mem_fopen_write<'a>() -> *mut MEMFILE<'a> {
    println!("mem_fopen_write");

    return ptr::null_mut();
}

// Write bytes to stream

pub fn mem_fwrite(
    ptr: *mut libc::c_void,
    size: usize,
    nmemb: usize,
    stream: *mut MEMFILE,
) -> usize {
    println!("mem_fwrite");

    return 0;
}

pub fn mem_get_buf(stream: *mut MEMFILE, buf: *mut *mut libc::c_void, buflen: *mut usize) {
    println!("mem_get_buf");
}

pub fn mem_fclose(stream: *mut MEMFILE) {
    println!("mem_fclose");
}

pub fn mem_ftell(stream: *mut MEMFILE) -> i64 {
    println!("mem_ftell");

    return 0;
}

pub fn mem_fseek(stream: *mut MEMFILE, position: i64, whence: mem_rel_t) -> i32 {
    println!("mem_fseek");

    return 0;
}
