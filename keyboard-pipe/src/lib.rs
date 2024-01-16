#![no_std]

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::pipe::Pipe;

// Pipe for transporting keystrokes from ESP-NOW to emulator core
const PIPE_BUF_SIZE: usize = 15;
pub static PIPE: Pipe<CriticalSectionRawMutex, PIPE_BUF_SIZE> = Pipe::new();
