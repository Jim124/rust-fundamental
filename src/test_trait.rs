trait Animal{
  fn make_sound(&self) ->();
}

trait NotDangerous{}
trait Dangerous{}
#[allow(dead_code)]
struct Person<PetType:Animal+NotDangerous,PetType2> where PetType2:Animal + Dangerous{
  first_name:String,
  pet:PetType,
  pet2:PetType2,
}

struct Dog{}
impl NotDangerous for Dog {
    
}
impl Animal for Dog {
    fn make_sound(&self) ->() {
        println!("dog barked");
    }
}
struct Cat{}
impl NotDangerous for Cat {
    
}
impl Animal for Cat {
    fn make_sound(&self) ->() {
        println!("Cat meow");
    }
}
struct Bear{}
impl Dangerous for Bear {
    
}
impl Animal for Bear{
  fn make_sound(&self) ->() {
      println!("Bear roared");
  }
}
pub fn create_person(){
  let dog = Dog{};
  let cat = Cat{};
  let bear = Bear{};
  
  let person = Person{first_name:"dog".to_string(),pet:dog,pet2:bear};
  person.pet.make_sound();
  person.pet2.make_sound();
}