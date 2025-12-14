use std::cell::RefCell;

thread_local! {
  pub static SDL: RefCell<Option<sdl3::Sdl>> = RefCell::new(None);
  pub static VIDEO: RefCell<Option<sdl3::VideoSubsystem>> = RefCell::new(None);
  pub static EVENTS: RefCell<Option<sdl3::EventSubsystem>> = RefCell::new(None);
}

pub fn get_sdl() -> sdl3::Sdl {
  SDL.with(|x| {
    x.borrow().clone()
  }).expect("SDL not initialized or called from wrong thread")
}

pub fn get_video() -> sdl3::VideoSubsystem {
  VIDEO.with(|x| {
    x.borrow().clone()
  }).expect("SDL not initialized or called from wrong thread")
}

pub fn get_events() -> sdl3::EventSubsystem {
  EVENTS.with(|x| {
    x.borrow().clone()
  }).expect("SDL not initialized or called from wrong thread")
}
