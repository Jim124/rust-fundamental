pub fn test_iterator(){
  let fruit_list = vec!["Apple","Strawberry","Blueberry","Orange","Mango"];
  let mut  fruit_iterator = fruit_list.iter();
  let nuts_list = vec!["penuts","almonds","walsnuts"];
  fruit_iterator.next();
  fruit_iterator.next();
  let fruit = fruit_iterator.next();
  println!("{}",fruit.unwrap());
  // chain method
  let aggregate_list = fruit_list.iter().chain(&nuts_list);

  // collection method change iterator to collection
  let all_foods: Vec<&&str> = aggregate_list.clone().collect();
  for food in all_foods{
    println!("collection to vec {}", food);
  }
  for food in aggregate_list{
    println!(" chain method {}",food );
  }
  // map method
  let fruit_list_strings = fruit_list.iter().map(|e| String::from(*e));
  let new_fruits = fruit_list_strings.map(| mut e| {e.push_str(" fruit");return e;});
  // for each method
  new_fruits.clone().for_each(|e| println!("{}", e));
  //last 
  println!("{}",new_fruits.clone().last().unwrap() );
  // stepBy method
  let mut step_by = new_fruits.step_by(2);
  println!("{}",step_by.next().unwrap() );
  println!("{}",step_by.next().unwrap() );
  println!("{}",step_by.next().unwrap() );
  // let new_fruit_list_strings = fruit_list.iter().map(String::from);
  

  // for fruit in fruit_iterator{
  //   println!("{}", fruit);
  // }
  // fruit_iterator.next();
}