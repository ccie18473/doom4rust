#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// w_checksum.h
/////////////////////////////
//
// DESCRIPTION:
//       Generate a checksum of the WAD directory.
//

/////////////////////////////
// w_checksum.c
/////////////////////////////
//
// DESCRIPTION:
//       Generate a checksum of the WAD directory.
//

pub fn GetFileNumber(handle: wad_file_t) -> i32 {
    println!("GetFileNumber");

    return 0;
}

pub fn ChecksumAddLump(sha1_context: *mut sha1_context_t, lump: *mut lumpinfo_t) {
    println!("ChecksumAddLump");
}

pub fn W_Checksum(digest: sha1_digest_t) {
    println!("W_Checksum");
}
