pub trait Calculator {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
    fn div(&self) -> i32;
    fn mul(&self) -> i32;

    /// Default Implementation
    fn get_result(&self) {
        println!("The result of Addition is: {}", self.add());
        println!("The result of Subtraction is: {}", self.sub());
    }
}

pub struct Data {
    pub first_number: i32,
    pub second_number: i32
}

impl Calculator for Data {
     fn add(&self) -> i32 {
        self.first_number + self.second_number
    }

    fn sub(&self) -> i32 {
        self.first_number - self.second_number
    }

    fn div(&self) -> i32 {
        unimplemented!()
    }

    fn mul(&self) -> i32 {
        unimplemented!()
    }
}