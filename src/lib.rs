pub trait Contextmanager<T, R> {
    fn enter(&mut self) {}

    fn exit(&mut self, result: &T) -> R;

    fn with(&mut self, closure: impl Fn(&mut Self) -> T) -> R {
        self.enter();
        let result = closure(self);
        self.exit(&result)
    }
}
