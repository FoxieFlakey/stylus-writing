use sdl3::log::{Category, Priority};

pub fn init() {
  sdl3::log::set_output_function(|priority, category, msg| {
    let category = match category {
      Category::Application => "Application",
      Category::Assert => "Assert",
      Category::Audio => "Audio",
      Category::Custom => "Custom",
      Category::Error => "Error",
      Category::Gpu => "Gpu",
      Category::Input => "Input",
      Category::Render => "Render",
      Category::System => "System",
      Category::Test => "Test",
      Category::Unknown => "Unknown",
      Category::Video => "Video"
    };
    
    match priority {
      Priority::Critical => log::error!("[CRITICAL] [SDL/{category}] {msg}"),
      Priority::Debug => log::debug!("[SDL/{category}] {msg}"),
      Priority::Error => log::error!("[SDL/{category}] {msg}"),
      Priority::Info => log::info!("[SDL/{category}] {msg}"),
      Priority::Trace => log::trace!("[SDL/{category}] {msg}"),
      Priority::Verbose => log::trace!("[SDL/{category}] {msg}"),
      Priority::Warn => log::warn!("[SDL/{category}] {msg}"),
    }
  });
  
  sdl3::log::set_log_priorities(sdl3::log::Priority::Verbose);
}

