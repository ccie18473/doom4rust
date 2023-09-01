#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// zone.h
/////////////////////////////
//
// DESCRIPTION:
//      Zone Memory Allocation, perhaps NeXT ObjectiveC inspired.
//	Remark: this was the only stuff that, according
//	 to John Carmack, might have been useful for
//	 Quake.
//

//
// ZONE MEMORY
// PU - purge tags.

pub enum PURGE {
    PU_STATIC = 1, // static entire execution time
    PU_SOUND,      // static while playing
    PU_MUSIC,      // static while playing
    PU_FREE,       // a free block
    PU_LEVEL,      // static until level exited
    PU_LEVSPEC,    // a special thinker in a level

    // Tags >= PU_PURGELEVEL are purgable whenever needed.
    PU_PURGELEVEL,
    PU_CACHE,

    // Total number of different tag types
    PU_NUM_TAGS,
}

//
// This is used to get the local FILE:LINE info from CPP
// prior to really call the function in question.
//
pub fn Z_ChangeTag(p: *mut libc::c_void, t: i32) {
    println!("Z_ChangeTag");

    Z_ChangeTag2(p, t, file!(), line!())
}

/////////////////////////////
// zone.c
/////////////////////////////
//
// DESCRIPTION:
//	Zone Memory Allocation. Neat.
//

//
// ZONE MEMORY ALLOCATION
//
// There is never any space between memblocks,
//  and there will never be two contiguous free memblocks.
// The rover can be left pointing at a non-empty block.
//
// It is of no value to free a cachable block,
//  because it will get overwritten automatically if needed.
//

//pub const MEM_ALIGN: i32 =  sizeof(pub fn *);
pub const ZONEID: i32 = 0x1d4a11;

pub struct memblock_t {
    pub size: i32, // including the header and possibly tiny fragments
    pub user: *mut *mut libc::c_void,
    pub tag: i32, // PU_FREE if this is free
    pub id: i32,  // should be ZONEID
    pub next: *mut memblock_s,
    pub prev: *mut memblock_s,
}

impl memblock_t {
    pub fn new() -> Self {
        Self {
            size: 0,
            user: ptr::null_mut(),
            tag: 0,
            id: 0,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }
    }
}

type memblock_s = memblock_t;

pub struct memzone_t {
    // total bytes malloced, including header
    pub size: i32,

    // start / end cap for linked list
    pub blocklist: memblock_t,

    pub rover: *mut memblock_t,
}

impl memzone_t {
    pub fn new() -> Self {
        Self {
            size: 0,
            blocklist: memblock_t::new(),
            rover: ptr::null_mut(),
        }
    }
}

pub const MINFRAGMENT: u8 = 64;

//
// Z_ClearZone
//
pub fn Z_ClearZone(zone: Vec<memzone_t>) {
    println!("Z_ClearZone");
}

//
// Z_Init
//
pub fn Z_Init(doom: &mut modules) {
    println!("Z_Init");
}

//
// Z_Free
//
pub fn Z_Free(ptr: *mut libc::c_void) {
    println!("Z_Free");

    unsafe {
        libc::free(ptr);
    }
}

//
// Z_Malloc
// You can pass a NULL user if the tag is < PU_PURGELEVEL.
//

pub fn Z_Malloc(size: i32, tag: i32, user: *mut libc::c_void) -> *mut libc::c_void {
    println!("Z_Malloc");

    let result: *mut libc::c_void;
    //BUG
    // malloc 2 * size
    unsafe { result = libc::malloc(2 * size as usize) };

    return result;
}

//
// Z_FreeTags
//
pub fn Z_FreeTags(lowtag: i32, hightag: i32) {
    println!("Z_FreeTags");
}

//
// Z_DumpHeap
// Note: TFileDumpHeap( stdout ) ?
//
pub fn Z_DumpHeap(lowtag: i32, hightag: i32) {
    println!("Z_DumpHeap");
}

//
// Z_FileDumpHeap
//
pub fn Z_FileDumpHeap(f: *mut File) {
    println!("Z_FileDumpHeap");
}

//
// Z_CheckHeap
//
pub fn Z_CheckHeap() {
    println!("Z_CheckHeap");
}

//
// Z_ChangeTag
//
pub fn Z_ChangeTag2(ptr: *mut libc::c_void, tag: i32, file: &str, line: u32) {
    println!("Z_ChangeTag2");
}

pub fn Z_ChangeUser(ptr: *mut libc::c_void, user: *mut *mut libc::c_void) {
    println!("Z_ChangeUser");
}

//
// Z_FreeMemory
//
pub fn Z_FreeMemory() -> i32 {
    println!("Z_FreeMemory");

    return 0;
}

pub fn Z_ZoneSize() -> u32 {
    println!("Z_ZoneSize");

    return 0;
}
