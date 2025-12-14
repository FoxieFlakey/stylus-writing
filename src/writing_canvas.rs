use std::{cell::RefCell, rc::Rc};

use sdl3::{pixels::{Color, PixelFormat}, render::Canvas, video::Window};

use crate::shapes::{Rect, Stroke, Point};

pub struct WritingCanvas {
  bound: Rect,
  update_count: u64,
  stroke_distance_threshold: f32,
  // .0 = which pen
  // .1 = whether the pen is out or in the bound
  current_pen: Option<(u32, bool)>,
  canvas: Rc<RefCell<Canvas<Window>>>,
  all_strokes: Vec<Stroke>
}

impl WritingCanvas {
  pub fn new(bound: Rect, canvas: Rc<RefCell<Canvas<Window>>>) -> Self {
    Self {
      bound,
      canvas,
      update_count: 0,
      current_pen: None,
      stroke_distance_threshold: 3.0,
      all_strokes: Vec::new()
    }
  }
  
  pub fn with_pixels<R, F: FnOnce(&[u8], u32, u32, u32, PixelFormat) -> R>(&self, func: F) -> R {
    let canvas = self.canvas.borrow();
    let surface = canvas
      .read_pixels(Some(self.bound.clone().into()))
      .unwrap();
    
    let width = surface.width();
    let height = surface.height();
    let pitch = surface.pitch();
    let pixel_format = surface.pixel_format();
    
    surface.with_lock(|bytes| {
      func(bytes, width, height, pitch, pixel_format)
    })
  }
  
  pub fn get_stroke_count(&self) -> usize {
    self.all_strokes.len()
  } 
  
  pub fn get_update_count(&self) -> u64 {
    self.update_count
  }
  
  pub fn pen_down(&mut self, x: f32, y: f32, pen: u32) {
    if !self.bound.contains(&Point { x, y }) {
      return;
    }
    
    if let Some(current_pen) = self.current_pen {
      if current_pen.0 == pen {
        log::warn!("A same pen down'ed more than once");
      }
      return;
    }
    
    self.current_pen = Some((pen, true));
    self.all_strokes.push(Stroke {
      start: Point { x, y },
      end: Point { x, y }
    });
    self.update_count += 1;
  }
  
  pub fn pen_up(&mut self, x: f32, y: f32, pen: u32) {
    if let Some(current_pen) = self.current_pen {
      if current_pen.0 != pen {
        return;
      }
    }
    
    self.current_pen = None;
    if !self.bound.contains(&Point { x, y }) {
      return;
    }
    
    self.all_strokes.last_mut().unwrap().end = Point { x, y };
    self.update_count += 1;
  }
  
  pub fn pen_motion(&mut self, x: f32, y: f32, pen: u32) {
    let Some(current_pen) = self.current_pen.as_mut() else {
      return;
    };
    
    if current_pen.0 != pen {
      return;
    }
    
    if !self.bound.contains(&Point { x, y }) {
      // Pen went outside of the bound
      // TODO: Clip it instead
      current_pen.1 = false;
      return;
    }
    
    if current_pen.1 == false {
      // Pen was out of bound, start new stroke
      current_pen.1 = true;
      self.all_strokes.push(Stroke {
        start: Point { x, y },
        end: Point { x, y }
      });
    }
    
    let latest_stroke = self.all_strokes.last_mut().unwrap();
    latest_stroke.end = Point { x, y };
    self.update_count += 1;
    
    if latest_stroke.length() >= self.stroke_distance_threshold {
      self.all_strokes.push(Stroke {
        start: Point { x, y },
        end: Point { x, y }
      });
    }
  }
  
  pub fn clear(&mut self) {
    self.all_strokes.clear();
  }
  
  pub fn draw(&self) {
    let mut canvas = self.canvas.borrow_mut();
    canvas.set_draw_color(Color::RGB(0x88, 0x88, 0x88));
    let _ = canvas.fill_rect(Some(self.bound.clone().into()))
      .map_err(|e| log::warn!("error calling canvas.fill_rect {e}"));
    
    canvas.set_draw_color(Color::BLACK);
    for stroke in self.all_strokes.iter() {
      let _ = canvas.draw_line(stroke.start.clone(), stroke.end.clone())
        .map_err(|e| log::warn!("error calling canvas.draw_line: {e}"));
    }
  }
}

