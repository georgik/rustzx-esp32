
use rustzx_core::zx::keys::ZXKey;

use crate::zx_event::Event;

pub(crate) fn usb_code_to_zxkey(keycode: u8, pressed:bool) -> Option<Event> {
    let zxkey_event:Option<ZXKey> = match keycode {
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

    return zxkey_event.map(|k| Event::ZXKey(k, pressed))
}

pub (crate) fn usb_code_to_modifier(keycode: u8, pressed: bool) -> Option<Event> {
    let zxkey_event:Option<(ZXKey, ZXKey)> = match keycode {
        42 => Some((ZXKey::Shift, ZXKey::N0)),

        80 => Some((ZXKey::Shift, ZXKey::N5)), // Left
        81 => Some((ZXKey::Shift, ZXKey::N6)), // Down
        82 => Some((ZXKey::Shift, ZXKey::N7)), // Up
        79 => Some((ZXKey::Shift, ZXKey::N8)), // Right

        // // ========= Row 2 (the numbers) =========
        45 => Some((ZXKey::SymShift, ZXKey::J)), // -
        86 => Some((ZXKey::SymShift, ZXKey::J)), // - NumPad
        87 => Some((ZXKey::SymShift, ZXKey::K)), // + NumPad

        // // ========= Row 4 (ASDF) =========
        51 => Some((ZXKey::SymShift, ZXKey::O)), // ;
        // // KeyCode::Oem3 => Some((ZXKey::SymShift, ZXKey::Z)), // :
        // KeyCode::Oem3 => Some((ZXKey::SymShift, ZXKey::N7)), // '

        // // ========= Row 5 (ZXCV) =========
        54 => Some((ZXKey::SymShift, ZXKey::N)), // ,
        55 => Some((ZXKey::SymShift, ZXKey::M)), // .
        56 => Some((ZXKey::SymShift, ZXKey::V)), // /
        _ => None,
    };
    zxkey_event.map(|(k, k2)| Event::ZXKeyWithModifier(k, k2, pressed))
}