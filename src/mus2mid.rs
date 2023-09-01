#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::*;

/////////////////////////////
// mus2mid.h
/////////////////////////////

/////////////////////////////
// mus2mid.c
/////////////////////////////

pub const NUM_CHANNELS: u8 = 16;

pub const MIDI_PERCUSSION_CHAN: u8 = 9;
pub const MUS_PERCUSSION_CHAN: u8 = 15;

// MUS event codes
pub enum musevent {
    mus_releasekey = 0x00,
    mus_presskey = 0x10,
    mus_pitchwheel = 0x20,
    mus_systemevent = 0x30,
    mus_changecontroller = 0x40,
    mus_scoreend = 0x60,
}

// MIDI event codes
pub enum midievent {
    midi_releasekey = 0x80,
    midi_presskey = 0x90,
    midi_aftertouchkey = 0xA0,
    midi_changecontroller = 0xB0,
    midi_changepatch = 0xC0,
    midi_aftertouchchannel = 0xD0,
    midi_pitchwheel = 0xE0,
}

// Structure to hold MUS file header
pub struct musheader {
    pub id: [u8; 4],
    pub scorelength: u16,
    pub scorestart: u16,
    pub primarychannels: u16,
    pub secondarychannels: u16,
    pub instrumentcount: u16,
}

// Write timestamp to a MIDI file.

pub fn WriteTime(time: u32, midioutput: *mut MEMFILE) -> bool {
    println!("WriteTime");

    return false;
}

// Write the end of track marker
pub fn WriteEndTrack(midioutput: *mut MEMFILE) -> bool {
    println!("WriteEndTrack");

    return false;
}

// Write a key press event
pub fn WritePressKey(channel: u8, key: u8, velocity: u8, midioutput: *mut MEMFILE) -> bool {
    println!("WritePressKey");

    return false;
}

// Write a key release event
pub fn WriteReleaseKey(channel: u8, key: u8, midioutput: *mut MEMFILE) -> bool {
    println!("WriteReleaseKey");

    return false;
}

// Write a pitch wheel/bend event
pub fn WritePitchWheel(channel: u8, wheel: i16, midioutput: *mut MEMFILE) -> bool {
    println!("WriteReleaseKey");

    return false;
}

// Write a patch change event
pub fn WriteChangePatch(channel: u8, patch: u8, midioutput: *mut MEMFILE) -> bool {
    println!("WriteChangePatch");

    return false;
}

// Write a valued controller change event

pub fn WriteChangeController_Valued(
    channel: u8,
    control: u8,
    value: u8,
    midioutput: *mut MEMFILE,
) -> bool {
    println!("WriteChangeController_Valued");

    return false;
}

// Write a valueless controller change event
pub fn WriteChangeController_Valueless(channel: u8, control: u8, midioutput: *mut MEMFILE) -> bool {
    println!("WriteChangeController_Valueless");

    return false;
}

// Allocate a free MIDI channel.

pub fn AllocateMIDIChannel() -> i32 {
    println!("AllocateMIDIChannel");

    return 0;
}

// Given a MUS channel number, get the MIDI channel number to use
// in the outputted file.

pub fn GetMIDIChannel(mus_channel: i32, midioutput: *mut MEMFILE) -> i32 {
    println!("GetMIDIChannel");

    return 0;
}

pub fn ReadMusHeader(file: *mut MEMFILE, header: *mut musheader) -> bool {
    println!("ReadMusHeader");

    return false;
}

// Read a MUS file from a stream (musinput) and output a MIDI file to
// a stream (midioutput).
//
// Returns 0 on success or 1 on failure.

pub fn mus2mid(musinput: *mut MEMFILE, midioutput: *mut MEMFILE) -> bool {
    println!("mus2mid");

    return false;
}
