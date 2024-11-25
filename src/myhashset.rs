use::std::collections::HashSet;
pub fn test_hashset(){
  let  mut planet_list = HashSet::from(["Earth","Mercury","Venus"]);
  let planet_list_more = HashSet::from(["Earth","Mars","jupiter"]);
  let planet_different = planet_list.difference(&planet_list_more);
  println!("{:?}", planet_different);
  let planet_sys = planet_list.symmetric_difference(&planet_list_more);
  println!("{:?}",planet_sys );
  planet_list.insert("Pluto");
  println!("{:?}",planet_list);
}