pub trait Contextmanager<T, R> {
    fn enter(&mut self) {}

    fn exit(&mut self, result: &T) -> R;

    fn with(&mut self, closure: impl Fn(&mut Self) -> T) -> R {
        self.enter();
        let result = closure(self);
        self.exit(&result)
    }
}

#[derive(Debug)]
pub struct Suppress<E> {
    errors: Vec<E>,
}

pub fn suppress<T, E>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

impl<E> Suppress<E> {
    pub fn new(errors: impl Into<Vec<E>>) -> Self {
        Self { errors: errors.into() }
    }
}

impl<E, T> Contextmanager<Result<T, E>, Result<Option<T>, E>> for Suppress<E>
where
    E: Eq + Clone,
    T: Clone
{
    fn exit(&mut self, result: &Result<T, E>) -> Result<Option<T>, E> {
        match result {
            Err(error) => if self.errors.iter().any(|item| item == error) {
                Ok(None)
            } else {
                Err(error.clone())
            }
            Ok(value) => Ok(Some(value.clone()))
        }
    }
}
