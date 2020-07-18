/*
The idea of this piece of code is to get accustomed with basics of Vector in Rust
Quick points -
    Book url        -   https://doc.rust-lang.org/book/ch08-01-vectors.html
    stdlib url      -   https://doc.rust-lang.org/std/vec/struct.Vec.html
    Defination      -   A contiguous growable array type, written Vec<T> but pronounced 'vector'.
    Allocation      -   Happens as a contigious blocks of memory. Reallocates after exhaustion in
                        initiation capacity, which can be slow.
    Constitution    -   Vec is and always will be a (pointer, capacity, length) triplet.

*/
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{ErrorKind, Read};
use std::option::Option::Some;

static mut vec_data: Vec<i8> = Vec::new();

fn get_input(mut name: &String) -> String {
    println!("{}", name);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    line
}
pub fn vector_init() {
    println!("Datastructues with Vectors");
    let capacity=get_input(&String::from("Please enter the size of Vector, 0, to give none.\nPlease note, giving an right number can help you speed up your code")).trim().parse::<usize>().unwrap();
    for i in 0..capacity {
        vector_add(
            get_input(&format!("enter the {}th element", i + 1))
                .trim()
                .parse::<i8>()
                .unwrap(),
        )
    }
    unsafe {
        println!("Your Vector is {:?}", vec_data);
    }
}

pub fn vector_add(i: i8) {
    unsafe { vec_data.push(i) };
}

pub fn dequeue() -> Result<(), String> {
    /* this prints exception when we are underflowing */
    unsafe {
        if vec_data.is_empty() == false {
            vec_data.remove(0);
            Ok(())
        } else {
            Err("MyError".to_owned())
        }
    }
}
