use std::{cell::RefCell, rc::Rc};

use sdl3::{pixels::Color, render::Canvas, video::Window};

use crate::shapes::{Rect, Point};

pub struct Button {
  canvas: Rc<RefCell<Canvas<Window>>>,
  bound: Rect,
  is_down: bool,
  is_pressed: bool
}

impl Button {
  pub fn new(bound: Rect, canvas: Rc<RefCell<Canvas<Window>>>) -> Self {
    Self {
      is_pressed: false,
      is_down: false,
      bound,
      canvas
    }
  }
  
  pub fn reset(&mut self) {
    self.is_pressed = false;
  }
  
  pub fn draw(&self) {
    let mut canvas = self.canvas.borrow_mut();
    canvas.set_draw_color(Color::RGB(0xBB, 0xBB, 0xBB));
    let _ = canvas.fill_rect(Some(self.bound.clone().into()))
      .map_err(|e| log::warn!("error calling canvas.fill_rect {e}"));
  }
  
  pub fn is_pressed(&self) -> bool {
    self.is_pressed
  }
  
  pub fn set_bound(&mut self, bound: Rect) {
    self.bound = bound;
  }
  
  pub fn pen_up(&mut self, _x: f32, _y: f32) {
    if self.is_down {
      self.is_pressed = true;
      self.is_down = false;
    }
  }
  
  pub fn pen_down(&mut self, x: f32, y: f32) {
    if !self.bound.contains(&Point { x, y }) {
      return;
    }
    
    self.is_down = true;
  }
}

