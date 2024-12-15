use std::thread::spawn;
pub fn test_thread(){
  let mut x = 0u128;
  for i in 1..5000{
    x += i;
  }
  println!(" main thread finished a little bit, let's check another worker!");
}

pub fn test_spawn(){
  let thread_fn = || {
      let mut x = 0u128;
      for i in 1..5000{
        x += i;
      };
    println!("value of x is:{x}"); 
  };
  println!("Starting working with thread");
  let handle = spawn(thread_fn);
  let handle2 = spawn(thread_fn);
  println!("Working thread completed");
  
  // use is_finished method 
  loop {
    test_thread();
      if handle.is_finished() && handle2.is_finished(){
        println!("All the workers are done, let's get out of here!", );
        break;
      }
  }
  // use join method 
  // let r = handle.join();
  // r.ok().unwrap();
  // handle2.join();
}