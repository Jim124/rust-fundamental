struct Person{
  first_name:String,
  last_name:String,
}
pub fn test_closures(){
  let add = |x,y| {
    println!("x is {}, y is {}",x,y );
    x + y
  };
  
   let result = add(3,2);
   let print_result = |x| println!("the result is {}",result + x);

   print_result(99);
   // mutable Person
   let mut p1 = Person{first_name:"Jim".to_string(),last_name:"Du".to_owned()};
   let mut change_name = |new_last_name:&str| p1.last_name = new_last_name.to_string();
   change_name("Jonesey");
   change_name("o'sullivan");
   println!("{}",p1.last_name);
}