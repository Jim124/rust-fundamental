use std::cell::Cell;

#[derive(Debug)]
#[allow(dead_code)]
enum VehicleColor{
  White,
  Blue,
  Red,
  Black,
}
struct Person{
  first_name:String,
  last_name:String,
  birth_year:u16,
  birth_month:u8,
}
struct Customer<'p>{
  name:Cell<&'p str>,
  age:u16,
  email:String,
  address:String,
}

struct VehicleTuple(String,String,u16);
#[derive(Debug)]
struct Vehicle{
  manufacture:String,
  model:String,
  year:u16,
  color:VehicleColor,
  meter_driven:u32,
}

impl Vehicle{
  fn paint(&mut self,new_color:VehicleColor){
    self.color = new_color;
  }
  fn new_vehicle() ->Self{
    Self { manufacture: "default".to_string(), model: "default".to_string(), year: 2024, color: VehicleColor::Black,meter_driven:2000 }
  }
  fn meter_add(&mut self, meter:u32){
    self.meter_driven+= meter;
  }
}

fn new_vehicle() -> VehicleTuple{
  let vehicle = VehicleTuple("Demo".to_string(),"test".to_string(),18);
  return vehicle;
}
fn new_customer() ->Customer<'static>{
  let customer = Customer{
    name:Cell::from("Jim"),
    age:15,
    email:"email@example.com".to_string(),
    address:"address".to_string(),
  };
  return customer;
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
pub fn test_create_vehicle(){
  let vehicle = new_vehicle();
  println!("vehicle is {0},{1},{2}",vehicle.0,vehicle.1,vehicle.2 );
}

pub fn test_create_customer(){
  let my_customer = new_customer();
  println!("name is: {}, age is: {},  email is: {}, address is: {}", my_customer.name.get(),my_customer.age,my_customer.email,my_customer.address );
  my_customer.name.set("Daniel");
  println!("name is: {}, age is: {},  email is: {}, address is: {}", my_customer.name.get(),my_customer.age,my_customer.email,my_customer.address );


}

pub fn test_vehicle_method(){
  let mut vehicle = Vehicle::new_vehicle();
  vehicle.paint(VehicleColor::Red);
  println!("{:?}",vehicle );
  vehicle.meter_add(300);
  println!("{:?}",vehicle);
}
