// R: 'a => Object<R>: 'a
pub struct Object<R> {
    fns: Fns<R>,
}


impl<R> Object<R> {
    pub fn new(fns: Fns<R>) -> Self {
        Self { fns }
    }
    pub fn boxed(fns: Fns<R>) -> Box<Self> {
        Box::new(Self::new(fns))
    }
    pub fn update(&mut self) -> R {
        (self.fns.nop)()
    }
}


pub struct Fns<R> {
    pub nop: fn() -> R,
}