pub fn test_iterator(){
  let fruit_list = vec!["Apple","Strawberry","Blueberry","Orange"];
  let mut  fruit_iterator = fruit_list.iter();
  let nuts_list = vec!["penuts","almonds","walsnuts"];
  fruit_iterator.next();
  fruit_iterator.next();
  let fruit = fruit_iterator.next();
  println!("{}",fruit.unwrap());
  let aggregrate_list = fruit_list.iter().chain(&nuts_list);
  for food in aggregrate_list{
    println!("{}",food );
  }
  // for fruit in fruit_iterator{
  //   println!("{}", fruit);
  // }
  // fruit_iterator.next();
}