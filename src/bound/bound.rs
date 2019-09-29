use crate::introduction::intro::{Calculator, Data};

/*pub fn calculate(item: impl Calculator) {
    println!("The result of Addition is : {}", item.add());
}*/

pub fn calculate<T: Calculator> (item: T) {
    println!("The result of Addition is : {}", item.add());
}

