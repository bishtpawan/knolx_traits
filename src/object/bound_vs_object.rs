use crate::object::object::Calculator;


pub struct BoundVsObject<T: Calculator> {
    types: Vec<Box<T>>
}

impl<T> BoundVsObject<T>
    where T: Calculator {
    pub fn run(&self) {
        for data in self.types.iter() {
            println!("{}", data.calculate());
        }
    }
}