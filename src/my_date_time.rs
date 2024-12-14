extern crate chrono;
use std::time::{Duration, Instant};
pub fn test_chrono(){
  let utc_time = chrono::Utc::now();
  println!("{}",utc_time.format("%Y-%m-%d %H:%M:%S %Z"));
  let local_time = chrono::Local::now();
  println!("{}",local_time.format("%Y-%m-%d %H:%M:%S %Z") );
}

pub fn test_duration(){
  let dur1 = Duration::from_secs(15);
  println!("{}",dur1.as_millis() );
  let dur2 = Duration::from_millis(15500);
  let dur3 = dur1.checked_sub(dur2);
  println!("{}", dur3.unwrap_or_default().as_millis());
  let now = Instant::now();
  std::thread::sleep(Duration::from_millis(200));
  println!("{}",now.elapsed().as_micros() );
}