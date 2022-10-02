use contextlib::{with, Contextmanager};

#[derive(Debug)]
struct TestCTM {
    name: String,
    calls: i64,
}

impl TestCTM {
    fn new() -> Self {
        Self {
            name: "Test context manager".to_string(),
            calls: 0,
        }
    }
}

impl Contextmanager for TestCTM {
    fn enter(&mut self) {
        assert_eq!(0, self.calls);
        self.calls += 1;
        assert_eq!(1, self.calls);
    }

    fn exit(&mut self) {
        assert_eq!(1, self.calls);
        self.calls -= 1;
        assert_eq!(0, self.calls);
    }
}

#[test]
fn test_contextmanager() {
    with(&mut TestCTM::new(), |this| {
        assert_eq!(1, this.calls);
        this.name = "New name".to_string();
    })
}
