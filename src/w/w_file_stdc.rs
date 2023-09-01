#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::io::Read;

use super::*;

/////////////////////////////
// w_file_stdc.c
/////////////////////////////
//
// DESCRIPTION:
//	WAD I/O functions.
//

pub struct stdc_wad_file_t {
    pub wad: wad_file_t,
    pub fstream: *mut File,
}

pub fn W_StdC_OpenFile(doom: &mut modules, path: &str) -> *mut wad_file_t {
    println!("W_StdC_OpenFile");

    let result: *mut stdc_wad_file_t;

    let fstream: Result<std::fs::File, std::io::Error>;

    fstream = File::open(path);

    match fstream {
        Ok(mut file) => {
            // Create a new stdc_wad_file_t to hold the file handle.
            unsafe {
                result = Z_Malloc(
                    mem::size_of::<stdc_wad_file_t>() as i32,
                    PURGE::PU_STATIC as i32,
                    ptr::null_mut(),
                ) as *mut stdc_wad_file_t;
                (*result).wad.file_class = &mut doom.w.stdc_wad_file;
                (*result).wad.mapped = ptr::null_mut();
                (*result).wad.length = M_FileLength(&mut file) as u32;
                (*result).fstream = &mut file;

                return &mut (*result).wad;
            }
        }
        Err(error) => {
            return ptr::null_mut();
        }
    }
}

pub fn W_StdC_CloseFile(wad: *mut wad_file_t) {
    println!("W_StdC_CloseFile");
}

// Read data from the specified position in the file into the
// provided buffer.  Returns the number of bytes read.

pub fn W_StdC_Read(
    wad: *mut wad_file_t,
    offset: u32,
    buffer: &mut Vec<u8>,
    buffer_len: usize,
) -> usize {
    println!("W_StdC_Read");

    let stdc_wad: *mut stdc_wad_file_t;
    let result: usize;

    stdc_wad = wad as *mut stdc_wad_file_t;

    // Jump to the specified position in the file.
    //fseek(stdc_wad->fstream, offset, SEEK_SET);

    //BUG
    let mut fstream = File::open("doom1.wad").unwrap();
    unsafe { (*stdc_wad).fstream = &mut fstream };

    unsafe {
        (*(*stdc_wad).fstream)
            .seek(SeekFrom::Start(offset as u64))
            .unwrap();
    };

    // Read into the buffer.
    //result = fread(buffer, 1, buffer_len, stdc_wad->fstream);

    unsafe {
        buffer.resize(buffer_len, 0);
        result = (*(*stdc_wad).fstream).read(buffer).unwrap();
    };

    return result;
}
