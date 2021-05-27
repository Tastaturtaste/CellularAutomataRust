pub struct RGBA(pub [u8; 4]);

impl RGBA {
    pub fn black() -> RGBA {
        RGBA {
            0: [0x00, 0x00, 0x00, 0x00],
        }
    }
    pub fn white() -> RGBA {
        RGBA {
            0: [0xFF, 0xFF, 0xFF, 0xFF],
        }
    }
    pub fn red() -> RGBA {
        RGBA {
            0: [0xFF, 0x00, 0x00, 0xFF],
        }
    }
    pub fn green() -> RGBA {
        RGBA {
            0: [0x00, 0xFF, 0x00, 0xFF],
        }
    }
    pub fn blue() -> RGBA {
        RGBA {
            0: [0x00, 0x00, 0xFF, 0xFF],
        }
    }
    pub fn get_raw(&self) -> [u8; 4] {
        self.0
    }
    pub fn r(&mut self) -> &mut u8 {
        &mut self.0[0]
    }
    pub fn g(&mut self) -> &mut u8 {
        &mut self.0[1]
    }
    pub fn b(&mut self) -> &mut u8 {
        &mut self.0[2]
    }
    pub fn a(&mut self) -> &mut u8 {
        &mut self.0[3]
    }
}
