use std::cell::Cell;

use super::ClockSource;
use crate::Result;

pub struct ManualClock(Cell<u64>);

impl<'a> ClockSource for ManualClock {
    type Time = u64;
    type Delta = u64;
    fn now(&mut self) -> Result<Self::Time> {
        Ok(self.0.get())
    }
}

impl ManualClock {
    pub fn new(t: u64) -> ManualClock {
        ManualClock(Cell::new(t))
    }
    pub fn set_time(&self, t: u64) {
        self.0.set(t)
    }
}
