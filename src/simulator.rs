use std::{sync::{Mutex, atomic::{AtomicBool, Ordering}}, thread};

use rdev::{EventType, Key};

pub static DO_SHUTDOWN: AtomicBool = AtomicBool::new(false);
static SIMULATE: Mutex<Option<String>> = Mutex::new(None);

pub fn main() {
  log::info!("Simulator started");
  
  while DO_SHUTDOWN.load(Ordering::Relaxed) == false {
    thread::park();
    
    let simulate_string = SIMULATE.lock().unwrap().take();
    
    if let Some(text) = simulate_string {
      log::info!("Request to simulate: {text} received");
      for chr in text.chars() {
        let mut keys = match chr.to_ascii_uppercase() {
          'A' => [Some(Key::ShiftLeft), Some(Key::KeyA)],
          'B' => [Some(Key::ShiftLeft), Some(Key::KeyB)],
          'C' => [Some(Key::ShiftLeft), Some(Key::KeyC)],
          'D' => [Some(Key::ShiftLeft), Some(Key::KeyD)],
          'E' => [Some(Key::ShiftLeft), Some(Key::KeyE)],
          'F' => [Some(Key::ShiftLeft), Some(Key::KeyF)],
          'G' => [Some(Key::ShiftLeft), Some(Key::KeyG)],
          'H' => [Some(Key::ShiftLeft), Some(Key::KeyH)],
          'I' => [Some(Key::ShiftLeft), Some(Key::KeyI)],
          'J' => [Some(Key::ShiftLeft), Some(Key::KeyJ)],
          'K' => [Some(Key::ShiftLeft), Some(Key::KeyK)],
          'L' => [Some(Key::ShiftLeft), Some(Key::KeyL)],
          'M' => [Some(Key::ShiftLeft), Some(Key::KeyM)],
          'N' => [Some(Key::ShiftLeft), Some(Key::KeyN)],
          'O' => [Some(Key::ShiftLeft), Some(Key::KeyO)],
          'P' => [Some(Key::ShiftLeft), Some(Key::KeyP)],
          'Q' => [Some(Key::ShiftLeft), Some(Key::KeyQ)],
          'R' => [Some(Key::ShiftLeft), Some(Key::KeyR)],
          'S' => [Some(Key::ShiftLeft), Some(Key::KeyS)],
          'T' => [Some(Key::ShiftLeft), Some(Key::KeyT)],
          'U' => [Some(Key::ShiftLeft), Some(Key::KeyU)],
          'V' => [Some(Key::ShiftLeft), Some(Key::KeyV)],
          'W' => [Some(Key::ShiftLeft), Some(Key::KeyW)],
          'X' => [Some(Key::ShiftLeft), Some(Key::KeyX)],
          'Y' => [Some(Key::ShiftLeft), Some(Key::KeyY)],
          'Z' => [Some(Key::ShiftLeft), Some(Key::KeyZ)],
          _ => continue
        };
        
        if chr.is_ascii_lowercase() {
          keys[0] = None;
        }
        
        for key in keys.iter().flatten() {
          rdev::simulate(&EventType::KeyPress(*key)).unwrap();
        }
        
        sdl3::timer::delay(2);
        
        for key in keys.iter().rev().flatten() {
          rdev::simulate(&EventType::KeyRelease(*key)).unwrap();
        }
      }
    }
  }
  
  log::info!("Simulator ended");
}

pub fn simulate(text: String) {
  *SIMULATE.lock().unwrap() = Some(text);
}

