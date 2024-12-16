use std::{sync::mpsc, thread::{self, sleep}, time::Duration};

pub fn test_message(){
  let (transmitter,receiver) = mpsc::channel::<u8>();
  // drop receiver the transmitter status will be false
  // drop(receiver);
  let send_result = transmitter.send(100);
  // let send_result1 = transmitter.send(152);
  println!("send status {}",send_result.is_ok());
  transmitter.send(152);
  let receive_result = receiver.recv_timeout(Duration::from_millis(300));
  println!("receive status {}, the result is {}",receive_result.is_ok(),receive_result.unwrap() );
  let receive_result = receiver.recv_timeout(Duration::from_millis(300));
  println!("receive status {}, the result is {}",receive_result.is_ok(),receive_result.unwrap() );

  let processor_code =   move ||{
    println!("Starting processor thread ...");
    let mut failed_count =0;
    loop {
        println!("Attempting to receive message from channel ...");
        let receive_result = receiver.recv_timeout(Duration::from_millis(300));
        if receive_result.is_ok(){
          println!("receive result is {}",receive_result.unwrap() );
        }else{
          failed_count+=1;
        }
        if failed_count >10{
          println!("Aborting processor thread .. no work available");
          break;
        }
    }
  };
  
  for x in 1.. 6{
    let send_result = transmitter.send(x);
    println!("send status {}",send_result.is_ok() );
    sleep(Duration::from_millis(300));
  }
 thread::spawn(processor_code).join();

}