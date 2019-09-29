use traits::object::object::{Computation, Age, Add};
use traits::bound::bound_vs_objct::BoundVsObject;
use traits::introduction::intro::{Data, Calculator};
use traits::bound::bound::calculate;


fn main() {

  /// Introduction of the Traits

  let introduction = Data { first_number: 20, second_number: 10 };
  introduction.get_result();







}

/*  /// Trait Bound

  let bound = Data { first_number: 20, second_number: 10 };
  calculate(bound);*/

/*    /// Trait Object
    let object = Computation{ types: vec![
        Box::new(Age{
            birth_year: 1996,
            current_year: 2019
        }),
        Box::new(Add{
            first_number: 10,
            second_number: 20
        })] };
    object.run();*/



/*
/// Trait Bound vs Trait Objects

let object = BoundVsObject{ types: vec![
    Box::new(Age{
        birth_year: 1996,
        current_year: 2019
    }),
    Box::new(Add{
        first_number: 10,
        second_number: 20
    })] };
*/
