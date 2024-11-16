pub fn test_age(){
  let my_age :u16 = 240;
  match my_age{
    0 =>{
      println!("you are newborn infant");
    }
    1..=35 =>{
      println!("your age is up to 35");
    }
    36..=149 =>{
      println!("Age is between 36 and 149");
    }
    150.. =>{
      println!("Your age is over 150.");
    }
  }
}
struct Person{
  name:String,
  age: i32,
}
struct Point {
  x: i32,
  y: i32,
}

pub fn test_struct_match(){
  let p1 = Person{name:"Jim".to_string(),age:23};
  match p1 {
    Person{name,age:23}=>println!("you name is {}", name),
    _ =>println!("{}", "nothing"),
  }
  let point = Point{x:0,y:7};
  let Point {x:a,y:b} = point;
  println!("{},{}",a,b );
  match point {
    Point { x, y: 0 } => println!("On the x axis at {x}"),
    Point { x: 0, y } => println!("On the y axis at {y}"),
    Point { x, y } => {
        println!("On neither axis: ({x}, {y})");
    }
}
}