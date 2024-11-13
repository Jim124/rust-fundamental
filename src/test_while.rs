pub fn test_while(){
  let age_to_drive = 16u8;
  let mut current_age = 0u8;
  while current_age < age_to_drive{
    println!("waiting...");
    current_age+=1;
  }
}