use::std::collections::HashMap;
pub fn test_hashmap(){
  let mut stock_list:HashMap<String, f32> = HashMap::new();
  let mut another_stock_List = HashMap::<String,f32>::new();
  println!("{}",stock_list.len() );
  println!("{}",stock_list.is_empty() );
  stock_list.insert("NvDa".to_string(), 3.22);
  stock_list.insert("appl".to_string(), 233.22);
  stock_list.insert("amsc".to_string(),50.78 );
  stock_list.insert("appl".to_string(),236.66);
  println!("{:#?}",stock_list );
  stock_list.remove(&("appl".to_string()));
  println!("{:#?}",stock_list );
  stock_list.entry("Meta".to_string()).or_insert(245.67);
  stock_list.entry("Meta".to_string()).or_insert(456.78);
  println!("{:?}",stock_list );
  for (ticker,value) in stock_list.clone(){
    println!("{} tracking at price is {}",ticker,value );
  }
  println!("{:#?}",stock_list );
 
 
}