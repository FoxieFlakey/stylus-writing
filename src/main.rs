#![feature(thread_sleep_until)]

use std::{io::stdout, sync::{Arc, Condvar, Mutex, atomic::Ordering}, thread, time::Duration};

use log::LevelFilter;
use sdl3::{event::{Event, WindowEvent}, keyboard::Keycode, pixels::Color};
use taffy::{AvailableSpace, FlexDirection, FlexWrap, Size, Style, TaffyTree, prelude::FromLength};

use crate::{button::Button, pixel_buffer::PixelBuffer, shapes::Rect, timer::Timer, window::Window, writing_canvas::WritingCanvas};

mod timer;
mod shapes;
mod global;
mod sdl_log;
mod writing_canvas;
mod processing_thread;
mod processor;
mod button;
mod window;
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
  
  let window = Window::new(
      "Writer",
      800,
      300,
      400,
      200,
      0,
      0,
      true
    )
    .map_err(|e| {
      log::error!("Error creating new window: {e}");
    })?;
  
  log::info!("Everything is initialized");
  println!("Hello, world!");
  
  let mut timer = Timer::new(Duration::from_millis(1000 / 60));
  window.set_canvas_size(800, 300);
  
  let mut clear_button = Button::new(Rect {
    x1: (window.get_canvas_width() - 100) as f32,
    y1: 20.0,
    x2: (window.get_canvas_width() - 20) as f32,
    y2: 80.0
  }, window.get_canvas().clone());
  
  let mut tree = TaffyTree::<()>::new();
  let writing_canvas_layout = tree.new_leaf(Style {
      min_size: Size::from_lengths(100.0, 100.0),
      flex_grow: 1.0,
      ..Default::default()
    }).unwrap();
  
  let clear_button_layout = tree.new_leaf(Style {
      size: Size::from_lengths(100.0, 60.0),
      ..Default::default()
    }).unwrap();
  
  let buttons_layout = tree.new_with_children(
    Style {
      gap: Size::from_length(10.0),
      flex_direction: FlexDirection::Column,
      flex_wrap: FlexWrap::Wrap,
      ..Default::default()
    },
    &[clear_button_layout]
  ).unwrap();
  
  let root = tree.new_with_children(
    Style {
      padding: taffy::Rect::length(10.0),
      gap: Size::from_length(10.0),
      size: Size::from_percent(1.0, 1.0),
      flex_direction: FlexDirection::Row,
      flex_wrap: FlexWrap::Wrap,
      ..Default::default()
    },
    &[
      writing_canvas_layout,
      buttons_layout
    ]
  ).unwrap();
  
  let mut writing_canvas = WritingCanvas::new(Rect {
      x1: 20.0,
      y1: 20.0, 
      x2: (window.get_canvas_width() - 120) as f32,
      y2: (window.get_canvas_height() - 20) as f32
    }, window.get_canvas().clone());
  
  let mut recompute_layout = |writing_canvas: &mut WritingCanvas, clear_button: &mut Button| -> () {
    tree.compute_layout(
      root,
      Size {
        width: AvailableSpace::Definite(window.get_width() as f32),
        height: AvailableSpace::Definite(window.get_height() as f32)
      }
    ).unwrap();
    
    window.set_canvas_size(window.get_width(), window.get_height());
    
    let root = tree.layout(root).unwrap();
    let root_x = root.location.x;
    let root_y = root.location.y;
    let new_layout = tree.layout(writing_canvas_layout).unwrap();
    writing_canvas.set_bound(Rect {
      x1: root_x + new_layout.content_box_x(),
      y1: root_y + new_layout.content_box_y(),
      x2: root_x + new_layout.content_box_x() + new_layout.content_box_width(),
      y2: root_y + new_layout.content_box_y() + new_layout.content_box_height()
    });
    
    let new_layout = tree.layout(clear_button_layout).unwrap();
    let parent = tree.layout(buttons_layout).unwrap();
    let parent_x = root_x + parent.location.x;
    let parent_y = root_y + parent.location.y;
    clear_button.set_bound(Rect {
      x1: parent_x + new_layout.content_box_x(),
      y1: parent_y + new_layout.content_box_y(),
      x2: parent_x + new_layout.content_box_x() + new_layout.content_box_width(),
      y2: parent_y + new_layout.content_box_y() + new_layout.content_box_height()
    });
  };
  
  recompute_layout(&mut writing_canvas, &mut clear_button);
  
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
        Event::Window { window_id, win_event: WindowEvent::Resized(_, _), .. } => {
          if window_id != window.get_window_id() {
            log::warn!("Unknown window events for unknown window?!");
            continue;
          }
          
          recompute_layout(&mut writing_canvas, &mut clear_button);
        }
        _ => ()
      }
    }
    
    if clear_button.is_pressed() {
      log::info!("Clearing writing canvas");
      writing_canvas.clear();
    }
    
    let mut canvas_borrow = window.get_canvas().borrow_mut();
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
    
    window.get_canvas().borrow_mut().present();
    timer.wait_tick(1);
  }
  
  processing_thread::DO_SHUTDOWN.store(true, Ordering::Relaxed);
  processing_thread_handle.thread().unpark();
  processing_thread_handle.join().unwrap();
  Ok(())
}

