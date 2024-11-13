use closures::test_closures;

pub mod helper;
pub mod test_if;
pub mod test_while;
pub mod test_for;
pub mod closures;
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
}
