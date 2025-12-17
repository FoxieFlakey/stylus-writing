use std::{sync::{Mutex, atomic::{AtomicBool, Ordering}}, thread};

use image::{ImageBuffer, Rgb};
use sdl3::pixels::PixelFormat;

use crate::processor::{Processor, paddle_ocr::PaddleOcrProcessor};

pub static DO_SHUTDOWN: AtomicBool = AtomicBool::new(false);
pub static CURRENTLY_RECOGNIZED: Mutex<Option<String>> = Mutex::new(None);

pub fn main() {
  log::info!("Processing thread started");
  
  let mut latest_update_id = 0;
  let mut processor = PaddleOcrProcessor::new();
  
  while DO_SHUTDOWN.load(Ordering::Relaxed) == false {
    thread::park();
    let Some((pixels, update_id)) = crate::get_pixels() else {
      // Buffer is not ready yet
      println!("Buffer not ready");
      continue;
    };
    
    if update_id == latest_update_id {
      continue;
    }
    latest_update_id = update_id;
    assert!(matches!(pixels.format, PixelFormat::RGB24));
    
    let image = ImageBuffer::<Rgb<u8>, &[u8]>::from_raw(u32::try_from(pixels.width).unwrap(), u32::try_from(pixels.height).unwrap(), &pixels.data).unwrap();
    let recognized = processor.detect(&image);
    log::info!("Text recognized: {}", recognized.trim());
    
    *CURRENTLY_RECOGNIZED.lock().unwrap() = Some(recognized);
  }
  
  log::info!("Processing thread stapped");
}
