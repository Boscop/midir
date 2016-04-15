#![cfg_attr(windows, feature(vec_push_all, alloc, heap_api))]

#[macro_use]
extern crate bitflags;

#[cfg(target_os="linux")]
extern crate libc;
#[cfg(all(target_os="linux", not(feature = "jack")))]
extern crate alsa_sys;
#[cfg(all(feature = "jack", not(target_os = "windows")))]
extern crate jack_sys;

#[cfg(target_os="windows")] extern crate winapi;
#[cfg(target_os="windows")] extern crate winmm as winmm_sys;
#[cfg(target_os="windows")] extern crate alloc;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ignore {
    None = 0x00,
    Sysex = 0x01,
    Time = 0x02,
    SysexAndTime = 0x03,
    ActiveSense = 0x04,
    SysexAndActiveSense = 0x05,
    TimeAndActiveSense = 0x06,
    All = 0x07
}

impl std::ops::BitOr for Ignore {
    type Output = Ignore;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        // this is safe because all combinations also exist as variants
        unsafe { std::mem::transmute(self as u8 | rhs as u8) }
    }
}

impl Ignore {
    #[inline(always)]
    pub fn contains(self, other: Ignore) -> bool {
        self as u8 & other as u8 != 0 
    }
}

// A MIDI structure used internally by the class to store incoming
// messages. Each message represents one and only one MIDI message.
#[derive(Debug, Clone)]
struct MidiMessage {
    bytes: Vec<u8>,
    timestamp: f64
}

impl MidiMessage {
    // TODO: probably not needed
    pub fn new() -> MidiMessage {
        MidiMessage {
            bytes: vec![],
            timestamp: 0.0
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MidiShortMessage {
	pub status: Status,
	pub data1: u8,
	pub data2: u8,
}
impl MidiShortMessage {
	fn to_i32(&self) -> i32 {
		((((self.data2 as i32) << 16) & 0xFF0000) |
		  (((self.data1 as i32) << 8) & 0xFF00) |
		  ((self.status as u8 as i32) & 0xFF)) as i32
	}
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    NoteOn = 0x90,
    NoteOff = 0x80,
    ControlChange = 0xB0,
    PitchBend = 0xE0,
    ProgramChange = 0xC0,
    Other(u8)
}

impl Into<u8> for Status {
    fn into(self) -> u8 {
        self as u8
    }
}

pub mod os; // include platform-specific behaviour

mod errors;
pub use errors::*;

mod common;
pub use common::*;

mod backend;
