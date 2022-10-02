use contextlib::Contextmanager;

#[derive(Debug)]
pub struct Counter {
    calls: i64,
}

impl Counter {
    pub fn new() -> Self {
        Self { calls: 0 }
    }

    pub fn calls(&self) -> i64 {
        self.calls
    }
}

impl Contextmanager for Counter {
    fn enter(&mut self) {
        self.calls += 1;
    }

    fn exit(&mut self) {
        self.calls -= 1;
    }
}
