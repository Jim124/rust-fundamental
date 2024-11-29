pub fn test_iterator(){
  let fruit_list = vec!["Apple","Strawberry","Blueberry","Orange"];
  let mut  fruit_iterator = fruit_list.iter();

  fruit_iterator.next();
  fruit_iterator.next();
  let fruit = fruit_iterator.next();
  println!("{}",fruit.unwrap() );
  // for fruit in fruit_iterator{
  //   println!("{}", fruit);
  // }
  // fruit_iterator.next();
}