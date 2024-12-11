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
  
  let first_names = vec!["Trevor","Shannon","James","Tasha"];
  let first_names_string = first_names.iter().map(|e|String::from(*e));
  let last_names = vec!["Jones","Sullivan","Tanner","Redman"];

  let last_names_string =  last_names.iter().map(|e| String::from(*e));
  let full_names = first_names_string.zip(last_names_string);
  // zip method
  full_names.clone().for_each(|e| println!("{} {}", e.0,e.1));
  // enumerate method
  for (index,value) in full_names.enumerate(){
    println!("index:{0} value is {1},{2}",index,value.0,value.1 );
  }


  // for fruit in fruit_iterator{
  //   println!("{}", fruit);
  // }
  // fruit_iterator.next();
}