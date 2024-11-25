use std::vec;

pub fn test_vec_init(){
  let mut vec :Vec<i32> = Vec::new();
  vec.push(10);
  vec.push(20);
  vec.push(30);
  vec.push(40);
  vec.push(50);
  vec.push(60);
  vec.push(70);
  vec.push(80);
  vec.push(90);
  println!("size of vec is {}",vec.len() );
  println!("capacity of vec is {}",vec.capacity() );
  println!("{:?}",vec );
}

pub fn test_vec_string(){
  let names = vec!["Trevor","Nancy","Shannon","Billy","Rachel"];
  for first_name in names.as_slice(){
    println!("{}...", first_name);
  }
  for first_name in names.clone(){
    println!("{}...", first_name);
  }
  println!("{:?}",names );
}

#[derive(Debug)]
struct Car{
  manufacture:String,
  model:String,
}
pub fn test_dynamic(){
  let mut car_lists :Vec<Car> = vec![];
  let mut car_lot_2: Vec<Car> = vec![];
  for _ in 1..=100{
    car_lists.push(Car{manufacture:"auto".to_string(),model:"dazong".to_string()});
  }
  for _ in 1..=100{
    car_lot_2.push(Car{manufacture:"Bentian".to_string(),model:"Fengtian".to_string()});
  }
  println!("length of car list is {}",car_lists.len() );
  println!("{:?}",car_lists );
  println!("{:?}",car_lists.get(0).unwrap());
  car_lists.append(&mut car_lot_2);
  println!("length of car list is {}",car_lists.len() );
  println!("length of car list is {}",car_lot_2.len() );
  car_lists.insert(0, Car{manufacture:"bentian".to_string(),model:"test".to_string()});
  car_lists.remove(0);
  let keep = |e:&Car| {if e.manufacture == "Bentian".to_string(){return true;}else{return false;}};
  car_lists.retain(keep);
  car_lists.reserve(5000);
  println!("length of car list is {}",car_lists.len() );
  println!("capacity of car list is {}",car_lists.capacity() );


}