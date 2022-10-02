use contextlib::{with, Contextmanager};
use std::time::{Duration, SystemTime};

#[derive(Debug)]
struct Timer {
    start: Option<SystemTime>,
    end: Option<SystemTime>,
}

impl Timer {
    fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub fn duration(&self) -> Option<Duration> {
        match self.end?.duration_since(self.start?) {
            Ok(duration) => Some(duration),
            Err(_) => None
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

#[test]
fn test_timer() {
    let mut timer = Timer::new();
    with(&mut timer, |this| {
        println!("Start: {:?}, end: {:?}", this.start, this.end,);
    });
    println!("Duration: {:?}", timer.duration());
}
