use std::{cell::RefCell, rc::Rc};

use sdl3::{render::Canvas, video::{Window as SDLWindow, WindowBuildError}};

use crate::global;

pub struct Window {
  window: SDLWindow,
  canvas: Rc<RefCell<Canvas<SDLWindow>>>
}

impl Window {
  pub fn new(
    name: &str,
    width: u32,
    height: u32,
    min_width: u32,
    min_height: u32,
    max_width: u32,
    max_height: u32,
    can_resize: bool
  ) -> Result<Window, WindowBuildError> {
    let mut window = global::get_video().window(name, width, height);
    window.hidden();
    if can_resize {
      window.resizable();
    }
    let mut window = window.build()?;
    
    let canvas = Rc::new(RefCell::new(window.clone().into_canvas()));
    
    window.set_minimum_size(min_width, min_height).unwrap();
    window.set_maximum_size(max_width, max_height).unwrap();
    window.show();
    if window.sync() == false {
      log::warn!("Error syncing window: ignoring");
    }
    
    Ok(Self {
      window,
      canvas
    })
  }
  
  pub fn get_canvas(&self) -> &Rc<RefCell<Canvas<SDLWindow>>> {
    &self.canvas
  }
  
  pub fn get_width(&self) -> u32 {
    self.window.size().0
  }
  
  pub fn get_height(&self) -> u32 {
    self.window.size().1
  }
}

