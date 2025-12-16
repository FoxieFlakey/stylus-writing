use image::{ImageBuffer, Rgb};

pub mod leptess;

pub trait Processor {
  fn detect(&mut self, image: &ImageBuffer<Rgb<u8>, &[u8]>) -> String;
}

