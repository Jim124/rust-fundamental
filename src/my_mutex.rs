use std::{ops::AddAssign, sync::Mutex, thread::{self, sleep}, time::Duration};

pub fn test_mutex(){
  let score = Mutex::new(16u16);
  let mut data = score.lock().unwrap();
  data.add_assign(5);
  println!("{}",data);
}

pub fn test_mutex_closure(){
  let score = Mutex::new(0u16);
  let my_func = || {
    println!("Threading 1 is waiting for mutex lock");
    let mut data = score.lock().unwrap();
    for i in 1..10{
      data.add_assign(i);
      println!("Threading 1 is adding {i}");
      sleep(Duration::from_millis(400));

    }
  };

  let my_func2 = || {
    loop {
      println!("Threading 2 is waiting for mutex lock ");
      let guard = score.lock();
      if guard.is_ok(){
        let mut data = guard.unwrap();
        for i in 1..10{
          data.add_assign(i);
          println!("Thread 2 is adding {i}");
        }
        break;
      }
      
      sleep(Duration::from_millis(300));
    }
    
    
  };
  thread::scope(|scope| {
    scope.spawn(my_func2);
    scope.spawn(my_func);
    // if handle2.is_err(){
    //   println!("there is an error in handle2");
    // }
  });
  println!("{:?}",score.lock().unwrap() );

}