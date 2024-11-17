struct Person{
  first_name:String,
  last_name:String,
  birth_year:u16,
  birth_month:u8,
}

fn new_person() ->Person{
  let p1 = Person{
    first_name:"Jim".to_string(),
    last_name:"Du".to_string(),
    birth_year:1990,
    birth_month:9,
  };
  return p1;
}
pub fn test_create_person(){
  let my_person = new_person();
  println!("First name: {0}, last name: {1}, birth month: {2}, birth year: {3}",my_person.first_name,my_person.last_name,my_person.birth_month,my_person.birth_year );
}