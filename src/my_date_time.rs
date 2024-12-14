extern crate chrono;
use std::time::{Duration, Instant};

use chrono::{NaiveDate, ParseError};
pub fn test_chrono(){
  let utc_time = chrono::Utc::now();
  println!("{}",utc_time.format("%Y-%m-%d %H:%M:%S %Z"));
  let local_time = chrono::Local::now();
  println!("{}",local_time.format("%Y-%m-%d %H:%M:%S %Z") );
  let date1 = NaiveDate::from_isoywd_opt(2024, 1, chrono::Weekday::Fri);
  let unwrapped_date = date1.unwrap();
  println!("{}",unwrapped_date.format("Day of year is: %j") );
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
pub fn test_naive_date() ->Result<(),ParseError>{
  let birthday = NaiveDate::parse_from_str("1990-01-10", "%Y-%m-%d")?;
  println!("my birthday is:{}",birthday );
  Ok(())

}