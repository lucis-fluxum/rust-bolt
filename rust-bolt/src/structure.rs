use crate::bolt::value::Marker;
use crate::serialize::Serialize;

pub const MARKER_TINY_STRUCTURE: u8 = 0xB0;
pub const MARKER_SMALL_STRUCTURE: u8 = 0xDC;
pub const MARKER_MEDIUM_STRUCTURE: u8 = 0xDD;

pub trait Structure: Marker + Serialize {
    fn get_signature(&self) -> u8;
}

// TODO: Create Structure enum to hold protocol Message types
