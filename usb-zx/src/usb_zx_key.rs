use rustzx_core::zx::keys::ZXKey;

use crate::zx_event::Event;

pub fn usb_code_to_zxkey(pressed: bool, modifier: u8, keycode: u8) -> Option<Event> {
    if modifier == 0 {
        let zxkey_event: Option<ZXKey> = match keycode {
            44 => Some(ZXKey::Space),

            30 => Some(ZXKey::N1),
            31 => Some(ZXKey::N2),
            32 => Some(ZXKey::N3),
            33 => Some(ZXKey::N4),
            34 => Some(ZXKey::N5), // 5
            35 => Some(ZXKey::N6),
            36 => Some(ZXKey::N7),
            37 => Some(ZXKey::N8),
            38 => Some(ZXKey::N9),
            39 => Some(ZXKey::N0),

            20 => Some(ZXKey::Q),
            26 => Some(ZXKey::W),
            8 => Some(ZXKey::E),
            21 => Some(ZXKey::R),
            23 => Some(ZXKey::T),
            28 => Some(ZXKey::Y),
            24 => Some(ZXKey::U),
            12 => Some(ZXKey::I),
            18 => Some(ZXKey::O),
            19 => Some(ZXKey::P),

            4 => Some(ZXKey::A),
            22 => Some(ZXKey::S),
            7 => Some(ZXKey::D),
            9 => Some(ZXKey::F),
            10 => Some(ZXKey::G),
            11 => Some(ZXKey::H),
            13 => Some(ZXKey::J),
            14 => Some(ZXKey::K),
            15 => Some(ZXKey::L),

            29 => Some(ZXKey::Z),
            27 => Some(ZXKey::X),
            6 => Some(ZXKey::C),
            25 => Some(ZXKey::V),
            5 => Some(ZXKey::B),
            17 => Some(ZXKey::N),
            16 => Some(ZXKey::M),

            // KeyCode::LShift => Some(ZXKey::Shift),
            // KeyCode::RShift => Some(ZXKey::Shift),

            // KeyCode::LAlt => Some(ZXKey::SymShift),
            // KeyCode::RAlt2 => Some(ZXKey::SymShift),
            // KeyCode::LControl => Some(ZXKey::SymShift),
            // KeyCode::RControl => Some(ZXKey::SymShift),
            40 => Some(ZXKey::Enter), // Enter

            _ => None,
        };

        if zxkey_event.is_some() {
            return zxkey_event.map(|k| Event::ZXKey(k, pressed));
        }
    }

    let zxkey_event: Option<(ZXKey, ZXKey)> = match (modifier, keycode) {
        (0, 42) => Some((ZXKey::Shift, ZXKey::N0)), // Backspace

        (0, 80) => Some((ZXKey::Shift, ZXKey::N5)), // Left
        (0, 81) => Some((ZXKey::Shift, ZXKey::N6)), // Down
        (0, 82) => Some((ZXKey::Shift, ZXKey::N7)), // Up
        (0, 79) => Some((ZXKey::Shift, ZXKey::N8)), // Right

        (2, 20) => Some((ZXKey::Shift, ZXKey::Q)),
        (2, 26) => Some((ZXKey::Shift, ZXKey::W)),
        (2, 8) => Some((ZXKey::Shift, ZXKey::E)),
        (2, 21) => Some((ZXKey::Shift, ZXKey::R)),
        (2, 23) => Some((ZXKey::Shift, ZXKey::T)),
        (2, 28) => Some((ZXKey::Shift, ZXKey::Y)),
        (2, 24) => Some((ZXKey::Shift, ZXKey::U)),
        (2, 12) => Some((ZXKey::Shift, ZXKey::I)),
        (2, 18) => Some((ZXKey::Shift, ZXKey::O)),
        (2, 19) => Some((ZXKey::Shift, ZXKey::P)),

        (2, 4) => Some((ZXKey::Shift, ZXKey::A)),
        (2, 22) => Some((ZXKey::Shift, ZXKey::S)),
        (2, 7) => Some((ZXKey::Shift, ZXKey::D)),
        (2, 9) => Some((ZXKey::Shift, ZXKey::F)),
        (2, 10) => Some((ZXKey::Shift, ZXKey::G)),
        (2, 11) => Some((ZXKey::Shift, ZXKey::H)),
        (2, 13) => Some((ZXKey::Shift, ZXKey::J)),
        (2, 14) => Some((ZXKey::Shift, ZXKey::K)),
        (2, 15) => Some((ZXKey::Shift, ZXKey::L)),

        (2, 29) => Some((ZXKey::Shift, ZXKey::Z)),
        (2, 27) => Some((ZXKey::Shift, ZXKey::X)),
        (2, 6) => Some((ZXKey::Shift, ZXKey::C)),
        (2, 25) => Some((ZXKey::Shift, ZXKey::V)),
        (2, 5) => Some((ZXKey::Shift, ZXKey::B)),
        (2, 17) => Some((ZXKey::Shift, ZXKey::N)),
        (2, 16) => Some((ZXKey::Shift, ZXKey::M)),

        // // ========= Row 2 (the numbers) =========
        (0, 45) => Some((ZXKey::SymShift, ZXKey::J)), // -
        (0, 86) => Some((ZXKey::SymShift, ZXKey::J)), // - NumPad
        (0, 87) => Some((ZXKey::SymShift, ZXKey::K)), // + NumPad

        // // ========= Row 4 (ASDF) =========
        (0, 51) => Some((ZXKey::SymShift, ZXKey::O)), // ;
        (2, 52) => Some((ZXKey::SymShift, ZXKey::P)), // "
        // // KeyCode::Oem3 => Some((ZXKey::SymShift, ZXKey::Z)), // :
        // KeyCode::Oem3 => Some((ZXKey::SymShift, ZXKey::N7)), // '

        // // ========= Row 5 (ZXCV) =========
        (0, 54) => Some((ZXKey::SymShift, ZXKey::N)), // ,
        (0, 55) => Some((ZXKey::SymShift, ZXKey::M)), // .
        (0, 56) => Some((ZXKey::SymShift, ZXKey::V)), // /
        _ => None,
    };
    zxkey_event.map(|(k, k2)| Event::ZXKeyWithModifier(k, k2, pressed))
}
