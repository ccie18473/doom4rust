#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::*;

/////////////////////////////
// w_wad.h
/////////////////////////////
//
// DESCRIPTION:
//	WAD I/O functions.
//

//
// TYPES
//

//
// WADFILE I/O related stuff.
//

pub type lumpinfo_t<'a> = lumpinfo_s<'a>;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct lumpinfo_s<'a> {
    pub name: [u8; 8],
    pub wad_file: *mut wad_file_t,
    pub position: i32,
    pub size: i32,
    pub cache: *mut libc::c_void,

    // Used for hash table lookups
    pub next: *mut lumpinfo_t<'a>,
}

impl<'a> lumpinfo_s<'a> {
    pub fn new() -> Self {
        Self {
            name: [0; 8],
            wad_file: ptr::null_mut(),
            position: 0,
            size: 0,
            cache: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}

/////////////////////////////
// w_wad.c
/////////////////////////////
//
// DESCRIPTION:
//	Handles WAD file header, directory, lump I/O.
//

#[derive(Debug, Serialize, Deserialize)]
#[repr(C)]
pub struct wadinfo_t {
    // Should be "IWAD" or "PWAD".
    pub identification: [u8; 4],
    pub numlumps: i32,
    pub infotableofs: i32,
}

impl wadinfo_t {
    pub fn new() -> Self {
        Self {
            identification: [0; 4],
            numlumps: 0,
            infotableofs: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[repr(C)]
pub struct filelump_t {
    pub filepos: i32,
    pub size: i32,
    pub name: [u8; 8],
}

// Lump names that are unique to particular game types. This lets us check
// the user is not trying to play with the wrong executable, eg.
// chocolate-doom -iwad hexen.wad.
#[repr(C)]
pub struct lump_t<'a> {
    pub mission: GameMission_t,
    pub lumpname: &'a str,
}

pub const unique_lumps: [lump_t; 4] = [
    lump_t {
        mission: GameMission_t::doom,
        lumpname: "POSSA1",
    },
    lump_t {
        mission: GameMission_t::heretic,
        lumpname: "IMPXA1",
    },
    lump_t {
        mission: GameMission_t::hexen,
        lumpname: "ETTNA1",
    },
    lump_t {
        mission: GameMission_t::strife,
        lumpname: "AGRDA1",
    },
];

// Hash function used for lump names.

pub fn W_LumpNameHash(s: &str) -> u32 {
    println!("W_LumpNameHash");

    let s = s.trim_matches(char::from(0));

    // This is the djb2 string hash function, modded to work on strings
    // that have a maximum length of 8.

    let mut result: u32 = 5381;
    let vec: Vec<char> = s.chars().collect();
    for c in vec {
        if c == '\0' {
            break;
        }
        result = ((result << 5) ^ result) ^ c.to_ascii_uppercase() as u32;
    }
    return result;
}

// Increase the size of the lumpinfo[] array to the specified size.
pub fn ExtendLumpInfo(doom: &mut modules, newnumlumps: i32) {
    println!("ExtendLumpInfo");

    let newlumpinfo: *mut lumpinfo_t;

    unsafe {
        newlumpinfo =
            libc::calloc(newnumlumps as usize, mem::size_of::<lumpinfo_t>()) as *mut lumpinfo_t
    };
    if newlumpinfo == ptr::null_mut() {
        println!("Couldn't realloc lumpinfo");
        I_Error();
    }

    // Copy over lumpinfo_t structures from the old array. If any of
    // these lumps have been cached, we need to update the user
    // pointers to the new location.
    let nlumps;
    if doom.w.numlumps < newnumlumps as u32 {
        nlumps = doom.w.numlumps;
    } else {
        nlumps = newnumlumps as u32;
    }
    unsafe {
        for i in 0..nlumps {
            //memcpy(&newlumpinfo[i], &lumpinfo[i], sizeof(lumpinfo_t));
            *newlumpinfo.add(i as usize) = *doom.w.lumpinfo.add(i as usize);

            if (*newlumpinfo.add(i as usize)).cache != ptr::null_mut() {
                Z_ChangeUser(
                    (*newlumpinfo.add(i as usize)).cache,
                    &mut (*newlumpinfo.add(i as usize)).cache,
                );
            }

            // We shouldn't be generating a hash table until after all WADs have
            // been loaded, but just in case...
            if (*doom.w.lumpinfo.add(i as usize)).next != ptr::null_mut() {
                let nextlumpnum =
                    (*doom.w.lumpinfo.add(i as usize)).next as usize - doom.w.lumpinfo as usize;
                (*newlumpinfo.add(i as usize)).next = newlumpinfo.add(nextlumpnum);
            }
        }

        // All done.
        libc::free(doom.w.lumpinfo as *mut libc::c_void);
    }
    doom.w.lumpinfo = newlumpinfo;
    doom.w.numlumps = newnumlumps as u32;
}

//
// LUMP BASED ROUTINES.
//

//
// W_AddFile
// All files are optional, but at least one file must be
//  found (PWAD, if all required lumps are present).
// Files with a .wad extension are wadlink files
//  with multiple lumps.
// Other files are single lumps with the base filename
//  for the lump name.

pub fn W_AddFile(doom: &mut modules, filename: &str) -> *mut wad_file_t {
    println!("W_AddFile");

    unsafe {
        let mut header: wadinfo_t;
        let mut lump_p: *mut lumpinfo_t;
        let wad_file: *mut wad_file_t;
        let length: i32;
        let startlump: i32;
        let mut fileinfo: *mut filelump_t;
        let mut filerover: *mut filelump_t;
        let mut newnumlumps: i32;
        let mut buffer: Vec<u8>;

        // open the file and add to directory
        wad_file = W_OpenFile(doom, filename);

        if wad_file == ptr::null_mut() {
            println!(" couldn't open {}", filename);
            return ptr::null_mut();
        }

        newnumlumps = doom.w.numlumps as i32;

        if filename[filename.len() - 3..].to_string() != "wad" {
            // single lump file

            // fraggle: Swap the filepos and size here.  The WAD directory
            // parsing code expects a little-endian directory, so will swap
            // them back.  Effectively we're constructing a "fake WAD directory"
            // here, as it would appear on disk.

            fileinfo = Z_Malloc(
                mem::size_of::<filelump_t>() as i32,
                PURGE::PU_STATIC as i32,
                ptr::null_mut(),
            ) as *mut filelump_t;
            (*fileinfo).filepos = LONG(0);
            (*fileinfo).size = LONG((*wad_file).length as i32);

            // Name the lump after the base of the filename (without the
            // extension).
            //BUG
            //M_ExtractFileBase(filename, (*fileinfo).name);
            newnumlumps += 1;
        } else {
            // WAD file
            let buffer_len: usize = mem::size_of::<wadinfo_t>();
            buffer = vec![0; buffer_len];
            W_Read(wad_file, 0, &mut buffer, buffer_len);
            //BUG
            header = bincode::deserialize(&buffer).unwrap();
            if header.identification != [b'I', b'W', b'A', b'D'] {
                // Homebrew levels?
                if header.identification != [b'P', b'W', b'A', b'D'] {
                    println!("Wad file {} doesn't have IWAD or PWAD id\n", filename);
                    I_Error();
                }

                // ???modifiedgame = true;
            }

            header.numlumps = LONG(header.numlumps);
            header.infotableofs = LONG(header.infotableofs);
            length = header.numlumps * mem::size_of::<filelump_t>() as i32;

            fileinfo =
                Z_Malloc(length, PURGE::PU_STATIC as i32, ptr::null_mut()) as *mut filelump_t;
            buffer = vec![0; length as usize];
            W_Read(
                wad_file,
                header.infotableofs as u32,
                &mut buffer,
                length as usize,
            );
            //(*fileinfo) = bincode::deserialize(&buffer).unwrap();
            fileinfo = buffer.as_mut_ptr() as *mut filelump_t;
            newnumlumps += header.numlumps;
        }

        // Increase size of numlumps array to accomodate the new file.
        startlump = doom.w.numlumps as i32;
        ExtendLumpInfo(doom, newnumlumps);
        lump_p = doom.w.lumpinfo.add(startlump as usize);
        filerover = fileinfo;

        for i in startlump..doom.w.numlumps as i32 {
            (*lump_p).wad_file = wad_file;
            (*lump_p).position = LONG((*filerover).filepos);
            (*lump_p).size = LONG((*filerover).size);
            (*lump_p).cache = ptr::null_mut();
            (*lump_p).name = (*filerover).name; //BUG 8 chars only

            lump_p = lump_p.add(1);
            filerover = filerover.add(1);
        }
        //BUG
        //Z_Free(fileinfo as *mut libc::c_void);

        if doom.w.lumphash != ptr::null_mut() {
            Z_Free(doom.w.lumphash as *mut libc::c_void);
            doom.w.lumphash = ptr::null_mut();
        }

        return wad_file;
    }
}

//
// W_NumLumps
//
pub fn W_NumLumps() -> i32 {
    println!("W_NumLumps");

    return 0;
}

//
// W_CheckNumForName
// Returns -1 if name not found.
//

pub fn W_CheckNumForName(doom: &mut modules, name: &str) -> i32 {
    println!("W_CheckNumForName");

    let mut lump_p: *mut lumpinfo_t;

    // Do we have a hash table yet?

    if doom.w.lumphash != ptr::null_mut() {
        let hash: u32;

        // We do! Excellent.
        hash = W_LumpNameHash(name) % doom.w.numlumps;
        //for (lump_p = lumphash[hash]; lump_p != NULL; lump_p = lump_p->next)
        unsafe {
            lump_p = *(doom.w.lumphash.add(hash as usize));
        }
        loop {
            if lump_p == ptr::null_mut() {
                break;
            }
            unsafe {
                let mut string = str::from_utf8(&(*lump_p).name).unwrap();
                string = string.trim_matches(char::from(0));
                if string == name {
                    let total = (lump_p as usize - doom.w.lumpinfo as usize) as i32;
                    let size = mem::size_of::<lumpinfo_t>() as i32;
                    return total / size;
                }
                lump_p = (*lump_p).next;
            }
        }
    } else {
        // We don't have a hash table generate yet. Linear search :-(
        //
        // scan backwards so patch lump files take precedence
        for i in (0..=doom.w.numlumps - 1).rev() {
            unsafe {
                let string = str::from_utf8(&(*doom.w.lumpinfo.add(i as usize)).name).unwrap();
                if string == name {
                    return i as i32;
                }
            }
        }
    }

    // TFB. Not found.
    //BUG
    if name == "d_introa" {
        return 231;
    }
    return -1;
}

//
// W_GetNumForName
// Calls W_CheckNumForName, but bombs out if not found.
//
pub fn W_GetNumForName(doom: &mut modules, name: &str) -> i32 {
    println!("W_GetNumForName");

    let i = W_CheckNumForName(doom, name);

    if i < 0 {
        println!("W_GetNumForName: {} not found!", name);
        I_Error();
    }

    return i;
}

//
// W_LumpLength
// Returns the buffer size needed to load the given lump.
//
pub fn W_LumpLength(doom: &mut modules, lump: u32) -> i32 {
    println!("W_LumpLength");

    if lump >= doom.w.numlumps {
        println!("W_LumpLength: {} >= numlumps", lump);
        I_Error();
    }

    let size: i32;

    unsafe { size = (*doom.w.lumpinfo.add(lump as usize)).size };

    return size;
}

//
// W_ReadLump
// Loads the lump into the given buffer,
//  which must be >= W_LumpLength().
//

//BUG/Behavior
//About as_mut_prt:
//
//The caller must ensure that the vector outlives the pointer this function returns,
//or else it will end up pointing to garbage. Modifying the vector may cause its buffer to be reallocated,
//which would also make any pointers to it invalid.

pub static mut dest_vec: Vec<u8> = Vec::new();

pub fn W_ReadLump(doom: &mut modules, lump: u32, dest: &mut *mut libc::c_void) {
    println!("W_ReadLump");

    let c: usize;
    let l: *mut lumpinfo_t;

    if lump >= doom.w.numlumps {
        println!("W_ReadLump: {} >= numlumps", lump);
        I_Error();
    }

    unsafe { l = doom.w.lumpinfo.add(lump as usize) };

    I_BeginRead();

    unsafe {
        c = W_Read(
            (*l).wad_file,
            (*l).position as u32,
            &mut dest_vec,
            (*l).size as usize,
        );
        *dest = dest_vec.as_mut_ptr() as *mut libc::c_void;
    };

    if c < unsafe { (*l).size as usize } {
        unsafe {
            println!(
                "W_ReadLump: only read {} of {} on lump {}",
                c,
                (*l).size,
                lump
            )
        };
        I_Error();
    }
    //I_EndRead ();
}

//
// W_CacheLumpNum
//
// Load a lump into memory and return a pointer to a buffer containing
// the lump data.
//
// 'tag' is the type of zone memory buffer to allocate for the lump
// (usually PU_STATIC or PU_CACHE).  If the lump is loaded as
// PU_STATIC, it should be released back using W_ReleaseLumpNum
// when no longer needed (do not use Z_ChangeTag).
//

pub fn W_CacheLumpNum(doom: &mut modules, lumpnum: i32, tag: i32) -> *mut libc::c_void {
    println!("W_CacheLumpNum");

    unsafe {
        let result: *mut u8;
        let lump: *mut lumpinfo_t;

        if lumpnum as u32 >= doom.w.numlumps {
            println!("W_CacheLumpNum: {} >= numlumps", lumpnum);
            I_Error();
        }

        //lump = &lumpinfo[lumpnum];
        lump = doom.w.lumpinfo.add(lumpnum as usize);

        // Get the pointer to return.  If the lump is in a memory-mapped
        // file, we can just return a pointer to within the memory-mapped
        // region.  If the lump is in an ordinary file, we may already
        // have it cached; otherwise, load it into memory.

        if (*(*lump).wad_file).mapped != ptr::null_mut() {
            // Memory mapped file, return from the mmapped region.
            //result = lump->wad_file->mapped + lump->position;
            result = (*(*lump).wad_file).mapped.add((*lump).position as usize);
        } else if (*lump).cache != ptr::null_mut() {
            // Already cached, so just switch the zone tag.
            result = (*lump).cache as *mut u8;
            Z_ChangeTag((*lump).cache, tag);
        } else {
            // Not yet loaded, so load it now
            (*lump).cache = Z_Malloc(W_LumpLength(doom, lumpnum as u32), tag, (*lump).cache);
            W_ReadLump(doom, lumpnum as u32, &mut (*lump).cache);
            result = (*lump).cache as *mut u8;
        }

        return result as *mut libc::c_void;
    }
}

//
// W_CacheLumpName
//
pub fn W_CacheLumpName(doom: &mut modules, name: &str, tag: i32) -> *mut libc::c_void {
    println!("W_CacheLumpName");

    let lumpnum = W_GetNumForName(doom, name);

    return W_CacheLumpNum(doom, lumpnum, tag);
}

//
// Release a lump back to the cache, so that it can be reused later
// without having to read from disk again, or alternatively, discarded
// if we run out of memory.
//
// Back in Vanilla Doom, this was just done using Z_ChangeTag
// directly, but now that we have WAD mmap, things are a bit more
// complicated ...
//

pub fn W_ReleaseLumpNum(lumpnum: i32) {
    println!("W_ReleaseLumpNum");
}

pub fn W_ReleaseLumpName(name: &str) {
    println!("W_ReleaseLumpName");
}

// Generate a hash table for fast lookups

pub fn W_GenerateHashTable(doom: &mut modules) {
    println!("W_GenerateHashTable");

    // Free the old hash table, if there is one

    if doom.w.lumphash != ptr::null_mut() {
        Z_Free(doom.w.lumphash as *mut libc::c_void);
    }
    // Generate hash table
    if doom.w.numlumps > 0 {
        doom.w.lumphash = Z_Malloc(
            mem::size_of::<*mut lumpinfo_t>() as i32 * doom.w.numlumps as i32,
            PURGE::PU_STATIC as i32,
            ptr::null_mut(),
        ) as *mut *mut lumpinfo_s;

        //memset(lumphash, 0, sizeof(lumpinfo_t *) * doom.w.numlumps);

        unsafe {
            for i in 0..doom.w.numlumps {
                *(doom.w.lumphash.add(i as usize)) = ptr::null_mut();
            }
        }
        unsafe {
            for i in 0..doom.w.numlumps {
                let hash: u32;
                let string = str::from_utf8(&(*doom.w.lumpinfo.add(i as usize)).name).unwrap();
                hash = W_LumpNameHash(string) % doom.w.numlumps;
                //BUG
                // Hook into the hash table
                (*doom.w.lumpinfo.add(i as usize)).next = *(doom.w.lumphash.add(hash as usize));
                *(doom.w.lumphash.add(hash as usize)) = doom.w.lumpinfo.add(i as usize);
            }
        }
    }

    // All done!
}

pub fn W_CheckCorrectIWAD(doom: &mut modules, mission: GameMission_t) {
    println!("W_CheckCorrectIWAD");

    let mut lumpnum: i32;

    for i in 0..unique_lumps.len() {
        if mission != unique_lumps[i].mission {
            lumpnum = W_CheckNumForName(doom, unique_lumps[i].lumpname);

            if lumpnum >= 0 {
                println!(
                    "\nYou are trying to use a %s IWAD file with the %s%s binary.\n
                This isn't going to work.\n
                You probably want to use the %s%s binary."
                );
                I_Error();
                //D_SuggestGameName(unique_lumps[i].mission, indetermined),
                //PROGRAM_PREFIX,
                //D_GameMissionString(mission),
                //PROGRAM_PREFIX,
                //D_GameMissionString(unique_lumps[i].mission),
            }
        }
    }
}
