pub fn uart_code_to_usb_key(keycode: u8) -> Option<(u8, u8)> {
    match keycode {
        0x08 => Some((0x00, 0x2a)), // Backspace
        0x0d => Some((0x00, 0x28)), // Enter
        0x20 => Some((0x00, 0x2c)), // Space
        0x31 => Some((0x00, 0x1e)), // 1
        0x32 => Some((0x00, 0x1f)), // 2
        0x33 => Some((0x00, 0x20)), // 3
        0x34 => Some((0x00, 0x21)), // 4
        0x35 => Some((0x00, 0x22)), // 5
        0x36 => Some((0x00, 0x23)), // 6
        0x37 => Some((0x00, 0x24)), // 7
        0x38 => Some((0x00, 0x25)), // 8
        0x39 => Some((0x00, 0x26)), // 9
        0x30 => Some((0x00, 0x27)), // 0
        0x71 => Some((0x00, 0x14)), // q
        0x77 => Some((0x00, 0x1a)), // w
        0x65 => Some((0x00, 0x08)), // e
        0x72 => Some((0x00, 0x15)), // r
        0x74 => Some((0x00, 0x17)), // t
        0x79 => Some((0x00, 0x1c)), // y
        0x75 => Some((0x00, 0x18)), // u
        0x69 => Some((0x00, 0x0c)), // i
        0x6f => Some((0x00, 0x12)), // o
        0x70 => Some((0x00, 0x13)), // p
        0x61 => Some((0x00, 0x04)), // a
        0x73 => Some((0x00, 0x16)), // s
        0x64 => Some((0x00, 0x07)), // d
        0x66 => Some((0x00, 0x09)), // f
        0x67 => Some((0x00, 0x0a)), // g
        0x68 => Some((0x00, 0x0b)), // h
        0x6a => Some((0x00, 0x0d)), // j
        0x6b => Some((0x00, 0x0e)), // k
        0x6c => Some((0x00, 0x0f)), // l
        0x7a => Some((0x00, 0x1d)), // z
        0x78 => Some((0x00, 0x1b)), // x
        0x63 => Some((0x00, 0x06)), // c
        0x76 => Some((0x00, 0x19)), // v
        0x62 => Some((0x00, 0x05)), // b
        0x6e => Some((0x00, 0x11)), // n
        0x6d => Some((0x00, 0x10)), // m
        0x2c => Some((0x00, 0x33)), // ,
        0x2e => Some((0x00, 0x34)), // .
        0x2f => Some((0x00, 0x35)), // /
        0x3b => Some((0x00, 0x2b)), // ;
        0x27 => Some((0x00, 0x2f)), // '
        0x5b => Some((0x00, 0x2f)), // [
        0x5d => Some((0x00, 0x30)), // ]
        0x5c => Some((0x00, 0x31)), // \
        0x2d => Some((0x00, 0x2d)), // -
        0x3d => Some((0x00, 0x2e)), // =
        0x60 => Some((0x00, 0x35)), // `
        0x21 => Some((0x02, 0x1e)), // !
        0x40 => Some((0x02, 0x1f)), // @
        0x23 => Some((0x02, 0x20)), // #
        0x24 => Some((0x02, 0x21)), // $
        0x25 => Some((0x02, 0x22)), // %
        0x5e => Some((0x02, 0x23)), // ^
        0x26 => Some((0x02, 0x24)), // &
        0x2a => Some((0x02, 0x25)), // *
        0x28 => Some((0x02, 0x26)), // (
        0x29 => Some((0x02, 0x27)), // )
        0x5f => Some((0x02, 0x2d)), // _
        0x2b => Some((0x02, 0x2e)), // +
        0x7e => Some((0x02, 0x35)), // ~
        0x51 => Some((0x02, 0x14)), // Q
        0x57 => Some((0x02, 0x1a)), // W
        0x45 => Some((0x02, 0x08)), // E
        0x52 => Some((0x02, 0x15)), // R
        0x54 => Some((0x02, 0x17)), // T
        0x59 => Some((0x02, 0x1c)), // Y
        0x55 => Some((0x02, 0x18)), // U
        0x49 => Some((0x02, 0x0c)), // I
        0x4f => Some((0x02, 0x12)), // O
        0x50 => Some((0x02, 0x13)), // P
        0x41 => Some((0x02, 0x04)), // A
        0x53 => Some((0x02, 0x16)), // S
        0x44 => Some((0x02, 0x07)), // D
        0x46 => Some((0x02, 0x09)), // F
        0x47 => Some((0x02, 0x0a)), // G
        0x48 => Some((0x02, 0x0b)), // H
        0x4a => Some((0x02, 0x0d)), // J
        0x4b => Some((0x02, 0x0e)), // K
        0x4c => Some((0x02, 0x0f)), // L
        0x5a => Some((0x02, 0x1d)), // Z
        0x58 => Some((0x02, 0x1b)), // X
        0x43 => Some((0x02, 0x06)), // C
        0x56 => Some((0x02, 0x19)), // V
        0x42 => Some((0x02, 0x05)), // B
        0x4e => Some((0x02, 0x11)), // N
        0x4d => Some((0x02, 0x10)), // M
        0x3c => Some((0x02, 0x33)), // <
        0x3e => Some((0x02, 0x34)), // >
        0x3f => Some((0x02, 0x35)), // ?
        0x3a => Some((0x02, 0x2b)), // :
        0x22 => Some((0x02, 0x34)), // "
        0x7b => Some((0x02, 0x2f)), // {
        0x7d => Some((0x02, 0x30)), // }
        0x7c => Some((0x02, 0x31)), // |

        _ => None,
    }
}

pub fn uart_composite_code_to_usb_key(code1: u8, code2: u8, code3: u8) -> Option<(u8, u8)> {
    match (code1, code2, code3) {
        (0x1b, 0x5b, 0x41) => Some((0x0, 0x52)), // Up
        (0x1b, 0x5b, 0x42) => Some((0x0, 0x51)), // Down
        (0x1b, 0x5b, 0x43) => Some((0x0, 0x4f)), // Right
        (0x1b, 0x5b, 0x44) => Some((0x0, 0x50)), // Left

        _ => None,
    }
}
