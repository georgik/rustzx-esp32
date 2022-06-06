use rustzx_core::{zx::keys::ZXKey, zx::keys::CompoundKey};

pub enum Event {
    NoEvent,
    ZXKey(ZXKey, bool),
    ZXKeyWithModifier(ZXKey, ZXKey, bool),
    CompoundKey(CompoundKey, bool),
    // Kempston(KempstonKey, bool),
    // Sinclair(SinclairJoyNum, SinclairKey, bool),
    // MouseMove { x: i8, y: i8 },
    // MouseButton(KempstonMouseButton, bool),
    // MouseWheel(KempstonMouseWheelDirection),
    // SwitchFrameTrace,
    // ChangeJoyKeyboardLayer(bool),
    // ChangeSpeed(EmulationMode),
    // InsertTape,
    // StopTape,
    // QuickSave,
    // QuickLoad,
    // OpenFile(PathBuf),
    // Exit,
}