use vector_intro::vector_init;

use crate::vector_intro::{dequeue, search_int};
use std::time::{SystemTime};

mod vector_intro;

fn main() {
       let now = SystemTime::now();

     println!("{}",search_int(&mut vector_init()));
     match now.elapsed() {
         Ok(elapsed)=> println!("{:?}",elapsed.as_micros()),
         Err(e) =>{ println!("error occured {:?}",e)}

     }
}
