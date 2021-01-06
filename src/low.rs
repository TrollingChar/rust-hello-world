// C (Context) -- тип надсистемы (не нужен чтоли?)
// R (Result)  -- что мы должны ей вернуть
// подсистемы здесь нет
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