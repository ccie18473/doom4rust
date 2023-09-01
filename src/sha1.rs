#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/////////////////////////////
// sha1.h
/////////////////////////////
//
// DESCRIPTION:
//     SHA-1 digest.
//

pub type sha1_context_t = sha1_context_s;
pub type sha1_digest_t = [u8; 20];

pub struct sha1_context_s {
    pub h0: u32,
    pub h1: u32,
    pub h2: u32,
    pub h3: u32,
    pub h4: u32,
    pub nblocks: u32,
    pub buf: [u8; 64],
    pub count: i32,
}

/////////////////////////////
// sha1.c
/////////////////////////////

pub fn SHA1_Init(hd: *mut sha1_context_t) {
    println!("SHA1_Init");
}

/****************
 * Transform the message X which consists of 16 32-bit-words
 */
pub fn Transform(hd: *mut sha1_context_t, data: *mut u8) {
    println!("Transform");
}

/* Update the message digest with the contents
 * of INBUF with length INLEN.
 */
pub fn SHA1_Update(hd: *mut sha1_context_t, inbuf: *mut u8, inlen: usize) {
    println!("SHA1_Update");
}

/* The routine final terminates the computation and
 * returns the digest.
 * The handle is prepared for a new cycle, but adding bytes to the
 * handle will the destroy the returned buffer.
 * Returns: 20 bytes representing the digest.
 */

pub fn SHA1_Final(digest: sha1_digest_t, hd: *mut sha1_context_t) {
    println!("SHA1_Final");
}

pub fn SHA1_UpdateInt32(context: *mut sha1_context_t, val: u32) {
    println!("SHA1_UpdateInt32");
}

pub fn SHA1_UpdateString(context: *mut sha1_context_t, str: &str) {
    println!("SHA1_UpdateString");
}
