use std::env::args;

struct Dog{
  name:String,
  year_born:i32,
}
impl Dog {
    fn get_details(&self){
      println!("The name of dog is {}, was born in {}",self.name,self.year_born );
    }
}
pub fn test_args(){
  let args:Vec<String> = args().collect();
  println!("{:?}",args );
  println!("{}",args[1] );
  let name = args.get(1).unwrap().into();
  let year_born = args.get(2).unwrap().parse::<i32>();
  if !year_born.is_ok(){
    println!("There is an error occurred parse ");
    return;
  }
  let age = year_born.ok().unwrap();
  let dog = Dog{name,year_born:age};
  dog.get_details();
}