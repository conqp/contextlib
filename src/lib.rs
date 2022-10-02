pub trait Contextmanager {
    fn enter(&mut self) {}
    fn exit(&mut self);
}

pub fn with<T: Contextmanager>(contextmanager: &mut T, run: impl Fn(&mut T) -> ()) {
    contextmanager.enter();
    run(contextmanager);
    contextmanager.exit();
}
