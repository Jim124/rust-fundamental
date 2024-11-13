pub fn test_for(){
  let ages = [14,16,18,26,35,41];
  let age_to_drive = 16i32;
  for age in ages{
    if age > age_to_drive{
      println!("you are big enough");
    } else {
      println!("waiting.........");
    }
  }
}