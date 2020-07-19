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

use rand::Rng;
use std::borrow::Borrow;

fn get_input(mut name: &String) -> String {
    println!("{}", name);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    line
}

pub fn vector_init<T>(first_elem:  T) -> Vec<T> {
    let mut rng = rand::thread_rng();
    let mut vec_data: Vec<T> = Vec::new();
    //let capacity = get_input(&String::from("Please enter the size of Vector, 0, to give none.\nPlease note, giving an right number can help you speed up your code")).trim().parse::<usize>().unwrap();
    //vector_add(&mut vec_data,  first_elem);
    vec_data
}

pub fn vector_add <T>(vec_data: &mut Vec<T>, val: T) {
    vec_data.push(val);
}

pub fn dequeue(vec_data: &mut Vec<u64>) -> Result<(), String> {
    if vec_data.is_empty() == false {
        vec_data.remove(0);
        println!("Dequeued one element from the given DataStructure");
        Ok(())
    } else {
        Err("How could you do that?\nSo mean! Vector didnt have anything at all!".to_owned())
    }
}

pub fn search_int(vec_data: &Vec<u64>) -> bool {
    let mut rng = rand::thread_rng();
    let rn: u64 = rng.gen();
    vec_data.contains(&rn)
}

pub fn sort_linear(vec_data: &mut Vec<u64>) -> &mut Vec<u64> {
    vec_data.sort();
    vec_data
}
