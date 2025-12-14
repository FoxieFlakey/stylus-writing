use std::fmt::Display;

use sdl3::render::{FPoint, FRect};

#[derive(Clone)]
pub struct Stroke {
  pub start: Point,
  pub end: Point
}

impl Stroke {
  pub fn length(&self) -> f32 {
    self.start.distance(&self.end)
  }
}

#[derive(Clone)]
pub struct Point {
  pub x: f32,
  pub y: f32
}

impl Into<FPoint> for Point {
  fn into(self) -> FPoint {
    FPoint { x: self.x, y: self.y }
  }
}

impl Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl Point {
  pub fn distance(&self, other: &Self) -> f32 {
    f32::sqrt(f32::powf(self.x - other.x, 2.0) + f32::powf(self.y - other.y, 2.0))
  }
}

#[derive(Clone)]
pub struct Rect {
  pub x1: f32,
  pub y1: f32,
  pub x2: f32,
  pub y2: f32
}

impl Rect {
  pub fn contains(&self, point: &Point) -> bool {
    let x1 = f32::min(self.x1, self.x2);
    let y1 = f32::min(self.y1, self.y2);
    let x2 = f32::max(self.x1, self.x2);
    let y2 = f32::max(self.y1, self.y2);
    
    point.x >= x1 && point.x <= x2 &&
    point.y >= y1 && point.y <= y2
  }
}

impl Into<FRect> for Rect {
  fn into(self) -> FRect {
    let x1 = f32::min(self.x1, self.x2);
    let y1 = f32::min(self.y1, self.y2);
    let x2 = f32::max(self.x1, self.x2);
    let y2 = f32::max(self.y1, self.y2);
    
    FRect {
      x: x1,
      y: y1,
      w: x2 - x1,
      h: y2 - y1
    }
  }
}
