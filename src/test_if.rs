pub fn test_if(){
  let age_to_drive = 16u8;
  println!("Enter the person's age:");
   let my_input = &mut String::from("");
   std::io::stdin().read_line(my_input).unwrap();
  // let mut my_input = String::new();
  // std::io::stdin().read_line(&mut my_input).unwrap();
  let age = my_input.replace("\n", "").parse::<u8>().unwrap();
  if age >= age_to_drive{
    println!("your age is {}",age);
  } else {
    println!("Wait a bit longer, you aren't old enough ");
  }

  let driver_license = if age >=16{true}else{false};
  println!("{}",driver_license );
}