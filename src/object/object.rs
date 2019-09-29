pub trait Calculator {
    fn calculate(&self) -> i32;
}

pub struct Age {
    pub birth_year: i32,
    pub current_year: i32
}

pub struct Add {
    pub first_number: i32,
    pub second_number: i32
}

impl Calculator for Age {
    fn calculate(&self) -> i32 {
        print!("Age :");
        self.current_year - self.birth_year
    }
}
impl Calculator for Add {
    fn calculate(&self) -> i32 {
        print!("Addition :");
        self.first_number + self.second_number
    }
}

pub struct Computation {
    pub types: Vec<Box<dyn Calculator>>,
}

impl Computation {
    pub fn run(&self) {
        for component in self.types.iter() {
            println!("{}", component.calculate());
        }
    }
}