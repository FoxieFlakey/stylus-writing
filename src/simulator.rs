use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}}, thread};

use rdev::{EventType, Key};
use x11rb::{connection::Connection, protocol::xproto::{ChangeWindowAttributesAux, ConnectionExt, EventMask}};

pub static DO_SHUTDOWN: AtomicBool = AtomicBool::new(false);
static SIMULATE: Mutex<Option<SimulateAction>> = Mutex::new(None);

enum SimulateAction {
  Enter,
  Space,
  DelWord,
  String(String)
}

pub fn main() {
  log::info!("Simulator started");
  let (conn, screen_num) = x11rb::connect(None).unwrap();
  let screen = &conn.setup().roots[screen_num];
  let root_window = screen.root;
  conn.change_window_attributes(
    root_window,
    &ChangeWindowAttributesAux::new()
      .event_mask(EventMask::FOCUS_CHANGE)
  ).unwrap();
  conn.flush().unwrap();
  
  let prev_window = Arc::new(Mutex::new(None));
  {
    let prev2 = prev_window.clone();
    thread::spawn(move || {
      loop {
        match conn.wait_for_event().unwrap() {
          x11rb::protocol::Event::FocusOut(data) => {
            *prev2.lock().unwrap() = Some(data.event);
          },
          _ => ()
        }
      }
    });
  }
  
  while DO_SHUTDOWN.load(Ordering::Relaxed) == false {
    thread::park();
    
    let simulate_string = SIMULATE.lock().unwrap().take();
    
    if let Some(action) = simulate_string {
      rdev::simulate(&EventType::KeyPress(Key::Alt)).unwrap();
      rdev::simulate(&EventType::KeyPress(Key::Tab)).unwrap();
      sdl3::timer::delay(50);
      rdev::simulate(&EventType::KeyRelease(Key::Tab)).unwrap();
      rdev::simulate(&EventType::KeyRelease(Key::Alt)).unwrap();
      sdl3::timer::delay(50);
      
      match action {
        SimulateAction::DelWord => {
          rdev::simulate(&EventType::KeyPress(Key::ControlLeft)).unwrap();
          rdev::simulate(&EventType::KeyPress(Key::Backspace)).unwrap();
          sdl3::timer::delay(5);
          rdev::simulate(&EventType::KeyRelease(Key::Backspace)).unwrap();
          rdev::simulate(&EventType::KeyRelease(Key::ControlLeft)).unwrap();
        }
        SimulateAction::Enter => {
          rdev::simulate(&EventType::KeyPress(Key::Return)).unwrap();
          sdl3::timer::delay(5);
          rdev::simulate(&EventType::KeyRelease(Key::Return)).unwrap();
        }
        SimulateAction::Space => {
          rdev::simulate(&EventType::KeyPress(Key::Space)).unwrap();
          sdl3::timer::delay(5);
          rdev::simulate(&EventType::KeyRelease(Key::Space)).unwrap();
        }
        SimulateAction::String(text) => {
          log::info!("Request to simulate: {text} received");
          
          for chr in text.chars() {
            let mut keys = match chr.to_ascii_uppercase() {
              ' ' => [None, Some(Key::Space)],
              '0' => [None, Some(Key::Num0)],
              '1' => [None, Some(Key::Num1)],
              '2' => [None, Some(Key::Num2)],
              '3' => [None, Some(Key::Num3)],
              '4' => [None, Some(Key::Num4)],
              '5' => [None, Some(Key::Num5)],
              '6' => [None, Some(Key::Num6)],
              '7' => [None, Some(Key::Num7)],
              '8' => [None, Some(Key::Num8)],
              '9' => [None, Some(Key::Num9)],
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
            
            if chr.is_ascii_lowercase() && chr.is_ascii_alphabetic() {
              keys[0] = None;
            }
            
            for key in keys.iter().flatten() {
              rdev::simulate(&EventType::KeyPress(*key)).unwrap();
            }
            
            sdl3::timer::delay(5);
            
            for key in keys.iter().rev().flatten() {
              rdev::simulate(&EventType::KeyRelease(*key)).unwrap();
            }
          }
        }
      }
    }
  }
  
  log::info!("Simulator ended");
}

pub fn simulate(text: String) {
  *SIMULATE.lock().unwrap() = Some(SimulateAction::String(text));
}

pub fn simulate_delword() {
  *SIMULATE.lock().unwrap() = Some(SimulateAction::DelWord);
}

pub fn simulate_enter() {
  *SIMULATE.lock().unwrap() = Some(SimulateAction::Enter);
}

pub fn simulate_space() {
  *SIMULATE.lock().unwrap() = Some(SimulateAction::Space);
}

