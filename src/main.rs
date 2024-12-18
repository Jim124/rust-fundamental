use closures::test_closures;
use matches::{test_age, test_struct_match};
use optiontest::{test_char, test_option_type};
use uuid::Uuid;

pub mod helper;
pub mod test_if;
pub mod test_while;
pub mod test_for;
pub mod closures;
pub mod matches;
pub mod optiontest;
pub mod structs;
pub mod test_trait;
pub mod myvec;
pub mod myHashmap;
pub mod myhashset;
pub mod myiter;
pub mod my_borrow;
pub mod my_date_time;
pub mod my_thread;
pub mod my_thread2;
pub mod my_mutex;
pub mod my_channel;
pub mod myargs;
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

    // structs::test_create_person();
    // structs::test_create_customer();
    // structs::test_create_vehicle();
    // structs::test_vehicle_method();
    // test_trait::create_person();
    // myvec::test_vec_init();
    // myvec::test_vec_string();
    // myvec::test_dynamic();
    // myHashmap::test_hashmap();
    // myhashset::test_hashset();
    // myiter::test_iterator();
    // my_borrow::test_borrow();
    // my_borrow::test_mut_borrow();
    // my_date_time::test_duration();
    // my_date_time::test_chrono();
    // let r = my_date_time::test_naive_date();
    // match r {
    //     Ok(()) => println!("hello"),
    //     _ => println!("there is an error" ),
    // }
    // my_thread::test_thread();
    //my_thread::test_spawn();
    //my_thread2::test_scope_thread();
    //my_thread2::test_move();
    // my_thread2::test_scope();
    // my_mutex::test_mutex();
   // my_mutex::test_mutex_closure();
    //my_channel::test_message();
    let id = Uuid::new_v4();
    println!("{}",id.to_string());
    myargs::test_args();
}
