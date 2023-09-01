#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// m_misc.h
/////////////////////////////
//
// DESCRIPTION:
//      Miscellaneous.
//

/////////////////////////////
// m_misc.c
/////////////////////////////
//
// DESCRIPTION:
//      Miscellaneous.
//

//
// Create a directory
//

pub fn M_MakeDirectory(path: &str) {
    println!("M_MakeDirectory");
    //mkdir(path, 0755);
}

// Check if a file exists

pub fn M_FileExists(filename: &str) -> bool {
    println!("M_FileExists");

    let fstream: Result<std::fs::File, std::io::Error>;

    fstream = File::open(filename);

    match fstream {
        Ok(file) => {
            println!("M_FileExists filename:{}", filename);
            return true;
        }
        Err(error) => {
            // If we can't open because the file is a directory, the
            // "file" exists at least!
            println!("M_FileExists filename:{}", filename);
            return false;
        }
    }
}

//
// Determine the length of an open file.
//

pub fn M_FileLength(handle: &mut File) -> u64 {
    println!("M_FileLength");

    let savedpos: u64;
    let length: u64;

    // save the current position in the file
    savedpos = handle.stream_position().unwrap();

    // jump to the end and find the length
    handle.seek(SeekFrom::End(0)).unwrap();

    length = handle.stream_position().unwrap();

    // go back to the old location
    handle.seek(SeekFrom::Start(savedpos)).unwrap();

    return length;
}

//
// M_WriteFile
//

pub fn M_WriteFile(name: &str, source: *mut libc::c_void, length: i32) -> bool {
    println!("M_WriteFile");

    return false;
}

//
// M_ReadFile
//

pub fn M_ReadFile(name: &str, buffer: *mut *mut u8) -> i32 {
    println!("M_ReadFile");

    return 0;
}

// Returns the path to a temporary file of the given name, stored
// inside the system temporary directory.
//
// The returned value must be freed with Z_Free after use.

pub fn M_TempFile(s: &str) -> &str {
    println!("M_TempFile");

    return s;
}

pub fn M_StrToInt(str: &str, result: *mut i32) -> bool {
    println!("M_StrToInt");

    return false;
}

pub fn M_ExtractFileBase(path: &str, dest: &str) {
    println!("M_ExtractFileBase");
}

//---------------------------------------------------------------------------
//
// PROC M_ForceUppercase
//
// Change string to uppercase.
//
//---------------------------------------------------------------------------

pub fn M_ForceUppercase(text: &str) {
    println!("M_ForceUppercase");
}

//
// M_StrCaseStr
//
// Case-insensitive version of strstr()
//

pub fn M_StrCaseStr(haystack: &str, needle: &str) -> &'static str {
    println!("M_StrCaseStr");

    return "";
}

//
// Safe version of strdup() that checks the string was successfully
// allocated.
//

pub fn M_StringDuplicate(orig: &str) -> &'static str {
    println!("M_StringDuplicate");

    return "";
}

//
// String replace function.
//

pub fn M_StringReplace(haystack: &str, needle: &str, replacement: &str) -> &'static str {
    println!("M_StringReplace");

    return "";
}

// Safe string copy function that works like OpenBSD's strlcpy().
// Returns true if the string was not truncated.

pub fn M_StringCopy(dest: &str, src: &str, dest_size: usize) -> bool {
    println!("M_StringCopy");

    return false;
}

// Safe string concat function that works like OpenBSD's strlcat().
// Returns true if string not truncated.

pub fn M_StringConcat(dest: &str, src: &str, dest_size: usize) -> bool {
    println!("M_StringConcat");

    return false;
}

// Returns true if 's' begins with the specified prefix.

pub fn M_StringStartsWith(s: &str, prefix: &str) -> bool {
    println!("M_StringConcat");

    return false;
}

// Returns true if 's' ends with the specified suffix.

pub fn M_StringEndsWith(s: &str, suffix: &str) -> bool {
    println!("M_StringEndsWith");

    return false;
}

// Return a newly-malloced string with all the strings given as arguments
// concatenated together.

pub fn M_StringJoin(s: &str) //BUG variadic
{
    println!("M_StringJoin");
}

// Safe, portable vsnprintf().
pub fn M_vsnprintf(buf: &str, buf_len: usize, s: &str, args: &str) //BUG va_list
{
    println!("M_vsnprintf");
}
// Safe, portable snprintf().
pub fn M_snprintf(buf: &str, buf_len: usize, s: &str) //BUG variadic
{
    println!("M_snprintf");
}
