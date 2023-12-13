
use rustzx_core::zx::keys::ZXKey;

use crate::zx_event::Event;

use pc_keyboard::{layouts, HandleControl, ScancodeSet2, KeyEvent, KeyCode};
/// returns ZX Spectum key form scancode of None if not found
// pub fn ascii_code_to_zxkey(ascii_code: u8, pressed: bool) -> Option<Event> {

pub(crate) fn pc_code_to_zxkey(keycode: KeyCode, pressed:bool) -> Option<Event> {
    let zxkey_event:Option<ZXKey> = match keycode {
        KeyCode::Spacebar => Some(ZXKey::Space),
        KeyCode::Key1 => Some(ZXKey::N1),
        KeyCode::Key2 => Some(ZXKey::N2),
        KeyCode::Key3 => Some(ZXKey::N3),
        KeyCode::Key4 => Some(ZXKey::N4),
        KeyCode::Key5 => Some(ZXKey::N5),
        KeyCode::Key6 => Some(ZXKey::N6),
        KeyCode::Key7 => Some(ZXKey::N7),
        KeyCode::Key8 => Some(ZXKey::N8),
        KeyCode::Key9 => Some(ZXKey::N9),
        KeyCode::Key0 => Some(ZXKey::N0),
        KeyCode::Q => Some(ZXKey::Q),
        KeyCode::W => Some(ZXKey::W),
        KeyCode::E => Some(ZXKey::E),
        KeyCode::R => Some(ZXKey::R),
        KeyCode::T => Some(ZXKey::T),
        KeyCode::Y => Some(ZXKey::Y),
        KeyCode::U => Some(ZXKey::U),
        KeyCode::I => Some(ZXKey::I),
        KeyCode::O => Some(ZXKey::O),
        KeyCode::P => Some(ZXKey::P),
        KeyCode::A => Some(ZXKey::A),
        KeyCode::S => Some(ZXKey::S),
        KeyCode::D => Some(ZXKey::D),
        KeyCode::F => Some(ZXKey::F),
        KeyCode::G => Some(ZXKey::G),
        KeyCode::H => Some(ZXKey::H),
        KeyCode::J => Some(ZXKey::J),
        KeyCode::K => Some(ZXKey::K),
        KeyCode::L => Some(ZXKey::L),
        KeyCode::Z => Some(ZXKey::Z),
        KeyCode::X => Some(ZXKey::X),
        KeyCode::C => Some(ZXKey::C),
        KeyCode::V => Some(ZXKey::V),
        KeyCode::B => Some(ZXKey::B),
        KeyCode::N => Some(ZXKey::N),
        KeyCode::M => Some(ZXKey::M),

        KeyCode::LShift => Some(ZXKey::Shift),
        KeyCode::RShift => Some(ZXKey::Shift),

        _ => None,
    };

    return zxkey_event.map(|k| Event::ZXKey(k, pressed))
}

// pub fn pc_code_to_modifier(ascii_code: u8, pressed: bool) -> Option<Event> {
//     let zxkey_event:Option<ZXKey> = match keycode {
//         KeyCode::

//     };
//     zxkey_event.map(|(k, k2)| Event::ZXKeyWithModifier(k, k2, pressed))
// }