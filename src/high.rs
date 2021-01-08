// pub struct Object<R> {
//     fns: Fns<R>,
//     state: Box<dyn State<R>>,
// }
//
//
// impl<R> Object<R> {
//     pub fn new(fns: Fns<R>) -> Self {
//         Self {
//             fns,
//             state: Mid::boxed(MidFns {
//                 nop: move || Box::new(move |o| (o.fns.nop)())
//             }),
//         }
//     }
//     pub fn boxed(fns: Fns<R>) -> Box<Self> {
//         Box::new(Self::new(fns))
//     }
//     pub fn update(&mut self) -> R {
//         let f = self.state.update();
//         f(self)
//     }
// }
//
//
// pub struct Fns<R> {
//     pub nop: fn() -> R,
// }
//
//
// trait State<R> {
//     fn update(&mut self) -> Box<dyn FnOnce(&mut Object<R>) -> R>;
// }
//
//
// type Mid<R> = crate::mid::Object<Box<dyn FnOnce(&mut Object<R>) -> R>>;
// type MidFns<R> = crate::mid::Fns<Box<dyn FnOnce(&mut Object<R>) -> R>>;
//
//
// impl<R> State<R> for Mid<R> {
//     fn update(&mut self) -> Box<dyn FnOnce(&mut Object<R>) -> R> {
//         Mid::update(self)
//     }
// }