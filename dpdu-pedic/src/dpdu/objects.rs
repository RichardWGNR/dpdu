use std::collections::HashMap;
use std::sync::LazyLock;
use map_macro::{hash_map_e, hash_set};

pub static BUSES: LazyLock<HashMap<&'static str, u32>> = LazyLock::new(|| hash_map_e! {
    "ISO_11898_2_DWCAN" => BUS_ISO_11898_2_DWCAN
});

pub static PROTOCOLS: LazyLock<HashMap<&'static str, u32>> = LazyLock::new(|| hash_map_e! {
    "ISO_11898_RAW" => PROTOCOL_ISO_11898_RAW
});

pub static PINS: LazyLock<HashMap<&'static str, u32>> = LazyLock::new(|| hash_map_e! {
    "HI" => PIN_HI,
    "LOW" => PIN_LOW
});

//pub static RESOURCES: LazyLock<HashMap<&'static str, u32>> = LazyLock::new(|| hash_map_e! {
//    ""
//});

// Buses.
pub const BUS_ISO_11898_2_DWCAN: u32 = 1;

// Protocols.
pub const PROTOCOL_ISO_11898_RAW: u32 = 1;

// Pins.
pub const PIN_HI: u32 = 1;
pub const PIN_LOW: u32 = 2;