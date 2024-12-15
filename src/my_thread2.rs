use std::thread::spawn;
pub fn test_scope_thread(){
  let age = 34;
  let print_age = move || {println!(" my age is {age}");};
  let _handle = spawn(print_age).join();
  println!("Finished printing age");
  println!("{age}");
}

pub fn test_move(){
  let v = vec![1, 2, 3];
    let handle = spawn( move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
     // error: v has moved to closer;
    //println!("{:?}",v );

}

struct Person{
  first_name:String,
}
pub fn test_scope(){
  let age = 34;
  let person = Person{first_name:String::from("jim")};
  let print_age = || {
    println!("this is the child scope");
    println!("Your age is:{age}");
    println!("your first name is:{}",&person.first_name );
  };
  // use scope
  std::thread::scope(|scope|{
    scope.spawn(print_age);
  });
  println!("coming back to the main thread");
  println!("Your age is:{age}");
  println!("your first name is:{}",person.first_name );
}