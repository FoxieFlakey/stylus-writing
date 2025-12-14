#![feature(thread_sleep_until)]

use std::{cell::RefCell, io::stdout, rc::Rc, sync::{Arc, Condvar, Mutex, atomic::Ordering}, thread, time::Duration};

use log::LevelFilter;
use sdl3::{event::Event, keyboard::Keycode, pixels::Color};

use crate::{button::Button, pixel_buffer::PixelBuffer, shapes::Rect, timer::Timer, writing_canvas::WritingCanvas};

mod timer;
mod shapes;
mod global;
mod sdl_log;
mod writing_canvas;
mod processing_thread;
mod button;
mod pixel_buffer;

fn init_sdl() -> Result<(), ()> {
  global::SDL.set(Some(
    sdl3::init()
      .map_err(|e| {
        log::error!("Error initializing SDL3: {e}");
      })?
  ));
  
  global::VIDEO.set(Some(
    global::get_sdl()
      .video()
      .map_err(|e| {
        log::error!("Error initializing SDL3 video: {e}");
      })?
  ));
  
  global::EVENTS.set(Some(
    global::get_sdl()
      .event()
      .map_err(|e| {
        log::error!("Error initializing SDL3 events: {e}");
      })?
  ));
  
  Ok(())
}

// .0 => the pixel buffer
// .1 => number of updates to the writing canvas
static CURRENT_PIXELS: Mutex<(Option<Arc<PixelBuffer>>, u64)> = Mutex::new((None, 0));
static CURRENT_PIXELS_COND: Condvar = Condvar::new();

// .0 => the pixel buffer
// .1 => number of updates or "generation id" of current buffer to track new changes
pub fn get_pixels() -> Option<(Arc<PixelBuffer>, u64)> {
  let current = CURRENT_PIXELS.lock().unwrap();
  let Some(buf) = current.0.clone() else {
    // Buffer is not ready yet
    return None;
  };
  
  Some((buf, current.1))
}

fn main() -> Result<(), ()> {
  simple_logging::log_to(stdout(), LevelFilter::max());
  sdl_log::init();
  init_sdl()?;
  
  let mut event_pump = global::get_sdl().event_pump()
    .map_err(|e| {
      log::error!("Error creating SDL event pump: {e}");
    })?;
  
  const WIDTH: u32 = 800;
  const HEIGHT: u32 = 300;
  
  let canvas = global::get_video()
    .window_and_renderer("Writer", WIDTH, HEIGHT)
    .map_err(|e| {
      log::error!("Error creating new window: {e}");
    })?;
  
  log::info!("Everything is initialized");
  println!("Hello, world!");
  
  let canvas = Rc::new(RefCell::new(canvas));
  let mut timer = Timer::new(Duration::from_millis(1000 / 60));
  
  let mut clear_button = Button::new(Rect {
    x1: (WIDTH - 100) as f32,
    y1: 20.0,
    x2: (WIDTH - 20) as f32,
    y2: 80.0
  }, canvas.clone());
  
  let mut writing_canvas = WritingCanvas::new(Rect {
      x1: 20.0,
      y1: 20.0, 
      x2: (WIDTH - 120) as f32,
      y2: (HEIGHT - 20) as f32
    }, canvas.clone());
  
  let processing_thread_handle = thread::spawn(processing_thread::main);
  'main_loop: loop {
    let old_count = writing_canvas.get_update_count();
    
    clear_button.reset();
    for event in event_pump.poll_iter() {
      match event {
        Event::PenDown { x, y, which, .. } => {
          writing_canvas.pen_down(x, y, which);
          clear_button.pen_down(x, y);
        }
        Event::PenUp { x, y, which, .. } => {
          writing_canvas.pen_up(x, y, which);
          clear_button.pen_up(x, y);
        }
        Event::PenMotion { x, y, which, .. } => {
          writing_canvas.pen_motion(x, y, which);
        }
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'main_loop,
        Event::Quit { .. } => break 'main_loop,
        _ => ()
      }
    }
    
    if clear_button.is_pressed() {
      log::info!("Clearing writing canvas");
      writing_canvas.clear();
    }
    
    let mut canvas_borrow = canvas.borrow_mut();
    canvas_borrow.set_draw_color(Color::RGB(0x55, 0x55, 0x55));
    canvas_borrow.clear();
    drop(canvas_borrow);
    
    writing_canvas.draw();
    clear_button.draw();
    
    if writing_canvas.get_update_count() > old_count {
      if let Ok(mut current_pixels) = CURRENT_PIXELS.try_lock() {
        writing_canvas.with_pixels(|bytes, width, height, pitch, pixel_format| {
          let mut data = Vec::new();
          data.extend_from_slice(bytes);
          current_pixels.0 = Some(
            Arc::new(PixelBuffer::new(
              data,
              usize::try_from(pitch).unwrap(),
              usize::try_from(width).unwrap(),
              usize::try_from(height).unwrap(),
              pixel_format
            ))
          );
        });
        
        current_pixels.1 = writing_canvas.get_update_count();
        
        processing_thread_handle.thread().unpark();
        CURRENT_PIXELS_COND.notify_all();
      }
    }
    
    canvas.borrow_mut().present();
    timer.wait_tick(1);
  }
  
  processing_thread::DO_SHUTDOWN.store(true, Ordering::Relaxed);
  processing_thread_handle.thread().unpark();
  processing_thread_handle.join().unwrap();
  Ok(())
}

