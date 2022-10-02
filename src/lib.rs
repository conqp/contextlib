pub trait Contextmanager {
    fn enter(&mut self) {}

    fn exit(&mut self);

    fn with(&mut self, closure: impl Fn(&mut Self) -> ()) {
        self.enter();
        closure(self);
        self.exit();
    }
}
