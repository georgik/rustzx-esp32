use rustzx_core::zx::keys::ZXKey;

use crate::zx_event::Event;

/// returns ZX Spectum key form scancode of None if not found
pub fn ascii_code_to_zxkey(ascii_code: u8, pressed: bool) -> Option<Event> {
    let zxkey_event = match ascii_code {
        // Control keys
        0x0A => Some(ZXKey::Enter),
        0x0D => Some(ZXKey::Enter),
        // 0x13 => Some(ZXKey::Enter),
        // Temporary Enter
        // 0x40 => Some(ZXKey::Enter),

        0x20 => Some(ZXKey::Space),

        // Numbers 0-9
        0x30 => Some(ZXKey::N0),
        0x31 => Some(ZXKey::N1),
        0x32 => Some(ZXKey::N2),
        0x33 => Some(ZXKey::N3),
        0x34 => Some(ZXKey::N4),
        0x35 => Some(ZXKey::N5),
        0x36 => Some(ZXKey::N6),
        0x37 => Some(ZXKey::N7),
        0x38 => Some(ZXKey::N8),
        0x39 => Some(ZXKey::N9),

        // Lower-case letters - a-z
        0x61 => Some(ZXKey::A),
        0x62 => Some(ZXKey::B),
        0x63 => Some(ZXKey::C),
        0x64 => Some(ZXKey::D),
        0x65 => Some(ZXKey::E),
        0x66 => Some(ZXKey::F),
        0x67 => Some(ZXKey::G),
        0x68 => Some(ZXKey::H),
        0x69 => Some(ZXKey::I),
        0x6A => Some(ZXKey::J),
        0x6B => Some(ZXKey::K),
        0x6C => Some(ZXKey::L),
        0x6D => Some(ZXKey::M),
        0x6E => Some(ZXKey::N),
        0x6F => Some(ZXKey::O),
        0x70 => Some(ZXKey::P),
        0x71 => Some(ZXKey::Q),
        0x72 => Some(ZXKey::R),
        0x73 => Some(ZXKey::S),
        0x74 => Some(ZXKey::T),
        0x75 => Some(ZXKey::U),
        0x76 => Some(ZXKey::V),
        0x77 => Some(ZXKey::W),
        0x78 => Some(ZXKey::X),
        0x79 => Some(ZXKey::Y),
        0x7A => Some(ZXKey::Z),

        _ => None,
    };

    return zxkey_event.map(|k| Event::ZXKey(k, pressed))
}


/// returns ZX Spectum key form scancode of None if not found
pub fn ascii_code_to_modifier(ascii_code: u8, pressed: bool) -> Option<Event> {
    let zxkey_event = match ascii_code {
        // Symbols
        0x21 => Some((ZXKey::SymShift, ZXKey::N1)),    // !
        0x22 => Some((ZXKey::SymShift, ZXKey::P)),     // "
        0x23 => Some((ZXKey::SymShift, ZXKey::N3)),    // #
        0x24 => Some((ZXKey::SymShift, ZXKey::N4)),    // $
        0x25 => Some((ZXKey::SymShift, ZXKey::N5)),    // %
        0x26 => Some((ZXKey::SymShift, ZXKey::N6)),    // &
        0x27 => Some((ZXKey::SymShift, ZXKey::N7)),    // '
        0x28 => Some((ZXKey::SymShift, ZXKey::N8)),    // (
        0x29 => Some((ZXKey::SymShift, ZXKey::N9)),    // )
        0x2A => Some((ZXKey::SymShift, ZXKey::B)),     // *
        0x2B => Some((ZXKey::SymShift, ZXKey::K)),     // +
        0x2C => Some((ZXKey::SymShift, ZXKey::N)),     // ,
        0x2D => Some((ZXKey::SymShift, ZXKey::J)),     // -
        0x2E => Some((ZXKey::SymShift, ZXKey::M)),     // .
        0x2F => Some((ZXKey::SymShift, ZXKey::V)),     // /

        0x3A => Some((ZXKey::SymShift, ZXKey::Z)),     // :
        0x3B => Some((ZXKey::SymShift, ZXKey::O)),     // ;
        0x3C => Some((ZXKey::SymShift, ZXKey::R)),     // <
        0x3D => Some((ZXKey::SymShift, ZXKey::L)),     // =
        0x3E => Some((ZXKey::SymShift, ZXKey::T)),     // >
        0x3F => Some((ZXKey::SymShift, ZXKey::C)),     // ?
        0x40 => Some((ZXKey::SymShift, ZXKey::N2)),    // @

        // Upper-case letters A-Z
        0x41 => Some((ZXKey::Shift, ZXKey::A)),
        0x42 => Some((ZXKey::Shift, ZXKey::B)),
        0x43 => Some((ZXKey::Shift, ZXKey::C)),
        0x44 => Some((ZXKey::Shift, ZXKey::D)),
        0x45 => Some((ZXKey::Shift, ZXKey::E)),
        0x46 => Some((ZXKey::Shift, ZXKey::F)),
        0x47 => Some((ZXKey::Shift, ZXKey::G)),
        0x48 => Some((ZXKey::Shift, ZXKey::H)),
        0x49 => Some((ZXKey::Shift, ZXKey::I)),
        0x4A => Some((ZXKey::Shift, ZXKey::J)),
        0x4B => Some((ZXKey::Shift, ZXKey::K)),
        0x4C => Some((ZXKey::Shift, ZXKey::L)),
        0x4D => Some((ZXKey::Shift, ZXKey::M)),
        0x4E => Some((ZXKey::Shift, ZXKey::N)),
        0x4F => Some((ZXKey::Shift, ZXKey::O)),
        0x50 => Some((ZXKey::Shift, ZXKey::P)),
        0x51 => Some((ZXKey::Shift, ZXKey::Q)),
        0x52 => Some((ZXKey::Shift, ZXKey::R)),
        0x53 => Some((ZXKey::Shift, ZXKey::S)),
        0x54 => Some((ZXKey::Shift, ZXKey::T)),
        0x55 => Some((ZXKey::Shift, ZXKey::U)),
        0x56 => Some((ZXKey::Shift, ZXKey::V)),
        0x57 => Some((ZXKey::Shift, ZXKey::W)),
        0x58 => Some((ZXKey::Shift, ZXKey::X)),
        0x59 => Some((ZXKey::Shift, ZXKey::Y)),
        0x5A => Some((ZXKey::Shift, ZXKey::Z)),

        _ => None,
    };

    zxkey_event.map(|(k, k2)| Event::ZXKeyWithModifier(k, k2, pressed))
}