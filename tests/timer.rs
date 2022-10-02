use contextlib::Contextmanager;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct Timer {
    start: Option<SystemTime>,
    end: Option<SystemTime>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub fn start(&self) -> Option<SystemTime> {
        self.start
    }

    pub fn end(&self) -> Option<SystemTime> {
        self.end
    }

    pub fn duration(&self) -> Option<Duration> {
        match self.end?.duration_since(self.start?) {
            Ok(duration) => Some(duration),
            Err(_) => None,
        }
    }
}

impl Contextmanager for Timer {
    fn enter(&mut self) {
        self.start = Some(SystemTime::now());
    }

    fn exit(&mut self) {
        self.end = Some(SystemTime::now());
    }
}
