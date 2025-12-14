use sdl3::pixels::PixelFormat;

pub struct PixelBuffer {
  pub data: Vec<u8>,
  pub pitch: usize,
  pub width: usize,
  pub height: usize,
  pub format: PixelFormat
}

impl PixelBuffer {
  pub fn new(data: Vec<u8>, pitch: usize, width: usize, height: usize, format: PixelFormat) -> Self {
    Self {
      data,
      format,
      height,
      pitch,
      width
    }
  }
}

