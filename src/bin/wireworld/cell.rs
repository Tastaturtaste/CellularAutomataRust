pub use cell_engine::cell::*;
use cell_engine::rgba::RGBA;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WireCell {
    Off,
    Wire,
    ElectronHead,
    ElectronTail,
}

use WireCell::*;
impl Cell for WireCell {
    fn to_rgba(&self) -> RGBA {
        match *self {
            Self::Off => RGBA::black(),
            Self::Wire => RGBA {
                0: [0xCB, 0xCB, 0x5C, 0xFF],
            },
            Self::ElectronHead => RGBA {
                0: [0xC5, 0x29, 0x29, 0xff],
            },
            Self::ElectronTail => RGBA {
                0: [0x29, 0x29, 0xC5, 0xff],
            },
        }
    }
    fn next(&self) -> Self {
        match *self {
            Off => Wire,
            Wire => ElectronHead,
            ElectronHead => ElectronTail,
            ElectronTail => Off,
        }
    }
}
