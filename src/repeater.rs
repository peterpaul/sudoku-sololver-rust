pub struct Repeater<T> {
    f: Box<dyn Fn() -> T + 'static>,
}

impl<T> Repeater<T> {
    pub fn new(f: Box<dyn Fn() -> T + 'static>) -> Repeater<T> {
        Repeater {
            f
        }
    }
}

impl<T> Iterator for Repeater<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        Some((*self.f)())
    }
}
