use closures::test_closures;
use matches::{test_age, test_struct_match};
use optiontest::{test_char, test_option_type};

pub mod helper;
pub mod test_if;
pub mod test_while;
pub mod test_for;
pub mod closures;
pub mod matches;
pub mod optiontest;
pub mod structs;
fn main() {
    println!("Hello, world!");
    let full_name = helper::get_full_name("Jim", "Du");
    println!("{}",full_name );
    let value = 5 as f32 / 3 as f32;
    println!("{:.2}",value);
    // test_if::test_if();
    // test_while::test_while();
    // test_for::test_for();
    test_closures();
    test_age();
    test_struct_match();
    let result = test_option_type();
    println!("{0}",result.unwrap());
    let char_result = test_char();
    if char_result.is_some(){
        println!("there is a value");
        println!("the result is {}",char_result.unwrap().to_string() );

    } else {
        println!("there is no value");
    }

    structs::test_create_person();
    structs::test_create_customer();
    structs::test_create_vehicle();
    structs::test_vehicle_method();
}
