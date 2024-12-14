pub fn test_borrow(){
  // Primitive types (Integers, Floats, and Booleans) have a known size at compile time 
  //and are stored entirely on the stack. Due to this, 
  //primitive types are cheap to copy, and they implement the copy trait instead of the move.
  let original = 1;
  let mut new = original;
  new = 32;
  println!("{}",original );
  println!("{}",new );

  let fruit = "apple".to_string();
  let fruit1 = fruit;
  println!("{}", fruit1);
  // ownership move to fruit1
  // println!("{}", fruit);
}

pub fn test_mut_borrow(){
  
  let mut original = String::from("hello");
  // can not change original
  let new = &original;
  println!("{}", original);
  println!("{}", new);
  let mut_borrowed = &mut original;
  //  borrow and mut borrow can not exist at the same time.
  // println!("{}",new );
  mut_borrowed.push_str(", world");
  println!("{}", original);
  

}