mod high;
mod mid;
mod low;


type High = crate::high::Object<Box<dyn FnOnce(&mut String)>>;
type HighFns = crate::high::Fns<Box<dyn FnOnce(&mut String)>>;


fn main() {
    let mut s: String = "hello".to_owned();
    let mut h = High::new(HighFns { nop: || Box::new(|s| s.push_str(" world!")) });
    let f = h.update();
    f(&mut s);
    println!("{}", s);
}