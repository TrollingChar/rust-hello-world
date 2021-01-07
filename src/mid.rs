pub struct Object<R> {
    fns: Fns<R>,
    state: Box<dyn State<R>>,
}


// почему R должно быть 'static?
impl<R: 'static> Object<R> {
    pub fn new(fns: Fns<R>) -> Self {
        Self {
            fns,
            state: Low::boxed(LowFns {
                nop: move || Box::new(move |o| (o.fns.nop)())
            }),
        }
    }
    pub fn boxed(fns: Fns<R>) -> Box<Self> {
        Box::new(Self::new(fns))
    }
    pub fn update(&mut self) -> R {
        let f = self.state.update();
        f(self)
    }
}


pub struct Fns<R> {
    pub nop: fn() -> R,
}


trait State<R> {
    fn update(&mut self) -> Box<dyn FnOnce(&mut Object<R>) -> R>;
}


type Low<R> = crate::low::Object<Box<dyn FnOnce(&mut Object<R>) -> R>>;
type LowFns<R> = crate::low::Fns<Box<dyn FnOnce(&mut Object<R>) -> R>>;


impl<R> State<R> for Low<R> {
    fn update(&mut self) -> Box<dyn FnOnce(&mut Object<R>) -> R> {
        Low::update(self)
    }
}