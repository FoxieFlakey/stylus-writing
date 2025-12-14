use std::{fs::File, io::Write, sync::atomic::{AtomicBool, Ordering}, thread, time::Duration};

use image::{ImageBuffer, Rgb, codecs::png::PngEncoder};
use leptess::LepTess;
use sdl3::pixels::PixelFormat;

pub static DO_SHUTDOWN: AtomicBool = AtomicBool::new(false);

pub fn main() {
  log::info!("Processing thread started");
  
  let mut latest_update_id = 0;
  let mut api = LepTess::new(None, "eng").unwrap();
  let mut image_data = Vec::new();
  
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
    
    let rgb_image = ImageBuffer::<Rgb<u8>, &[u8]>::from_raw(u32::try_from(pixels.width).unwrap(), u32::try_from(pixels.height).unwrap(), &pixels.data).unwrap();
    image_data.clear();
    rgb_image.write_with_encoder(PngEncoder::new(&mut image_data)).unwrap();
    
    api.set_image_from_mem(&image_data).unwrap();
    
    let recognized = api.get_utf8_text().unwrap();
    log::info!("Text recognized: {}", recognized.trim());
  }
  
  log::info!("Processing thread stapped");
}
