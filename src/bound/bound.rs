use crate::introduction::intro::{Calculator, Data};

pub fn calculate<T: Calculator> (item: T) {
    println!("The result of Addition is : {}", item.add());
}

/*
//This is also same but with differenrt syntax

pub fn calculate(item: impl Calculator) {
    println!("The result of Addition is : {}", item.add());
}*/


