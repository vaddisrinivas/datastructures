
use crate::vector_intro::{dequeue, search_int, sort_linear, vector_add, vector_init};
use std::time::SystemTime;
use rand::Rng;

mod vector_intro;

fn main() {
    /*
    Implementing Vectors with Generics
    */
    let now = SystemTime::now();
    let mut my_vec =vector_init((5 as u64));
    let mut rng = rand::thread_rng();
    //let capacity = get_input(&String::from("Please enter the size of Vector, 0, to give none.\nPlease note, giving an right number can help you speed up your code")).trim().parse::<usize>().unwrap();
    for i in 0..10000 {
        vector_add(&mut my_vec, rng.gen())
    }
    println!("{:?}",my_vec);

    match now.elapsed() {
        Ok(elapsed) => println!("{:?}", elapsed.as_micros()),
        Err(e) => println!("error occured {:?}", e),
    }
}
