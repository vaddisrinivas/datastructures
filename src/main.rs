mod vector_intro;
use crate::vector_intro::dequeue;
use std::fmt::Error;
use vector_intro::vector_init;

fn main() {
    vector_init();
    match dequeue() {
        Ok(()) => println!("No error"),
        Err(e) => eprintln!("{}", e),
        _ => {}
    }
}
