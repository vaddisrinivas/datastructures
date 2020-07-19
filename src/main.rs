/*
The idea of this piece of code is to get accustomed with basics of Vector in Rust
Quick points -
    Book url        -   https://doc.rust-lang.org/book/ch08-01-vectors.html
    stdlib url      -   https://doc.rust-lang.org/std/vec/struct.Vec.html
    Defination      -   A contiguous growable array type, written Vec<T> but pronounced 'vector'.
    Allocation      -   Happens as a contigious blocks of memory. Reallocates after exhaustion in
                        initiation capacity, which can be slow.
    Constitution    -   Vec is and always will be a (pointer, capacity, length) triplet.

Generics -
    URL - https://doc.rust-lang.org/book/ch10-01-syntax.html
    Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics.
    When the code runs, it performs just as it would if we had duplicated each definition by hand.
    The process of monomorphization makes Rust’s generics extremely efficient at runtime.

*/

use crate::vector_intro::{get_input, insertion_sort, selection_sort, vector_add, vector_init};
use rand::Rng;
use std::borrow::Borrow;
use std::time::SystemTime;

mod vector_intro;

fn main() {
    /*
    Implementing Vectors with Generics
    */
    println!("Please provide custom input for the following");
    let capacity = get_input(&String::from(String::from("Please enter the size of Vector, 0, to give none.\nPlease note, giving an right number can help you speed up your code"))).trim().parse::<usize>().unwrap();
    let mut my_vec = vector_init(
        get_input(&String::from("Please enter the first element here"))
            .trim()
            .parse::<i8>()
            .unwrap(),
        capacity,
    );
    let mut rng = rand::thread_rng();
    match get_input(&String::from("Do you want to provide input or do you want to continue with entropy?\n Enter 1 for custom input, 0 for entropy")).trim().parse::<i8>(){
        Ok(choice) => {
                match choice {
        1 => {
                    for i in 1..capacity
                    {
                        vector_add(&mut my_vec, get_input(&String::from(format!("Please enter your element at position {}",i))).trim().parse::<i8>().unwrap())
                    }
                },
        0 => {
                    for i in 1..capacity
                    {
                        vector_add(&mut my_vec, rng.gen())
                    }
        }
                    _ => {}
                }
        },
        Err(e) => {},
        _ => {}
    }
    let now = SystemTime::now();
    insertion_sort(&mut my_vec);

    println!("{:?}", my_vec);
    selection_sort(&mut my_vec);

    match now.elapsed() {
        Ok(elapsed) => println!("{:?}", elapsed.as_millis()),
        Err(e) => println!("error occured {:?}", e),
    }
}
