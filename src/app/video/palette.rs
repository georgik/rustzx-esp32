use rustzx_core::zx::video::colors::{ZXBrightness, ZXColor};
use rustzx_utils::palette::rgba::ORIGINAL as DEFAULT_PALETTE;

//type ColorRgba = [u8; 4];
type ColorRgba = [u8; 2];

const MAX_COLORS: usize = 16;

pub struct Palette {
    colors: [ColorRgba; MAX_COLORS],
}

pub const PALETTE_565: [[u8; 2]; 16] = [
    // normal
    0x0000_u16.to_be_bytes(),
    0x0019_u16.to_be_bytes(),
    0xC800_u16.to_be_bytes(),
    0xC819_u16.to_be_bytes(),
    0x0660_u16.to_be_bytes(),
    0x0679_u16.to_be_bytes(),
    0xCE60_u16.to_be_bytes(),
    0xCE79_u16.to_be_bytes(),
    // bright
    0x0000_u16.to_be_bytes(),
    0x001F_u16.to_be_bytes(),
    0xF800_u16.to_be_bytes(),
    0xF81F_u16.to_be_bytes(),
    0x07E0_u16.to_be_bytes(),
    0x00FFFF_u16.to_be_bytes(),
    0x07FF_u16.to_be_bytes(),
    0xFFFF_u16.to_be_bytes(),
];

impl Default for Palette {
    fn default() -> Self {
        Palette {
            colors: PALETTE_565,
        }
    }
}

impl Palette {
    pub fn get_rgba(&self, color: ZXColor, brightness: ZXBrightness) -> ColorRgba {
        let index = ((color as u8) + (brightness as u8) * 8) as usize;
        assert!(index < MAX_COLORS);
        self.colors[index]
    }
}
