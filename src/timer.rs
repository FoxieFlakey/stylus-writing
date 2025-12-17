use std::{ops::Mul, thread, time::{Duration, Instant}};

pub struct Timer {
  last_tick: Instant,
  period: Duration
}

impl Timer {
  pub fn new(period: Duration) -> Self {
    Timer {
      last_tick: Instant::now(),
      period
    }
  }
  
  pub fn wait_tick(&mut self, count: u32) {
    self.last_tick += self.period.mul(count);
    thread::sleep_until(self.last_tick);
  }
}
