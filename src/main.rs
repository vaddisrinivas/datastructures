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

use std::borrow::Borrow;
use std::time::SystemTime;

use rand::{random, Rng};

use crate::vector_intro::{
    bubble_sort, get_input, insertion_sort, iter_merge_sort, quick_sort, rec_bubble_sort,
    rec_insertions_sort, rec_merge_sort, rec_selection_sort, selection_sort,
};
use std::process::exit;

mod vector_intro;

fn main() {
    /*
    Implementing Vectors with Generics
    */

    for i in generate_array_sizes(
        get_input(&String::from(
            "Give the max power of 10 you want to test for \
        (9*10^x would be the range or random list)",
        ))
        .trim()
        .parse::<i64>()
        .unwrap(),
    )
    .iter()
    {
        let mut my_vec: Vec<u16> = Vec::with_capacity(*i as usize);
        let mut rng = rand::thread_rng();

        for j in 0..*i {
            vector_add(&mut my_vec, rng.gen())
        }
        println!(
            "_____________________________________________________________\
        ___________________________________"
        );
        println!(
            "insertion sort of {} (i64) -> {:?}",
            i,
            insertion_sort(&mut my_vec)
        );
        println!(
            "selection  sort of {} (i64) -> {:?}",
            i,
            selection_sort(&mut my_vec)
        );
        println!(
            "Bubble sort of {} (i64) -> {:?}",
            i,
            bubble_sort(&mut my_vec)
        );
                println!(
            "iterative merge sort took - {:?}",
            iter_merge_sort(&mut my_vec)
        );
        if *i>3000{
            continue
        }
        let now = SystemTime::now();
        rec_insertions_sort(&mut my_vec.clone(), 1 as usize, (*i - 1) as usize);
        match now.elapsed() {
            Ok(elapsed) => println!(
                "Recursive insertion  sort of {} (i64) -> {:?}",
                i,
                elapsed.as_micros()
            ),
            Err(e) => {
                println!("error occured {:?}", e);
            }
        }
        rec_selection_sort(&mut my_vec.clone(), 0 as usize, (*i - 1) as usize);
        match now.elapsed() {
            Ok(elapsed) => println!("Recursive selection of {} (i64) -> {:?}", i, elapsed),
            Err(e) => {
                println!("error occured {:?}", e);
            }
        }

        rec_bubble_sort(&mut my_vec.clone(), (*i) as usize);

        rec_merge_sort(&mut my_vec);
        match now.elapsed() {
            Ok(elapsed) => println!(
                "Recursive merge sort of {} (i64) -> {:?}",
                i,
                elapsed.as_micros()
            ),
            Err(e) => {
                println!("error occured {:?}", e);
            }
        }
        match now.elapsed() {
            Ok(elapsed) => println!("Recursive bubble  of {} (i64) -> {:?}", i, elapsed),
            Err(e) => {
                println!("error occured {:?}", e);
            }
        }

        let res_quick = quick_sort(&mut my_vec.clone(), 0, my_vec.len() - 1);
        match now.elapsed() {
            Ok(elapsed) => println!("quick sort of {} (i64) -> {:?}", i, elapsed.as_micros()),
            Err(e) => {
                println!("error occured {:?}", e);
            }
        }

        let sorted = my_vec.sort();
        match now.elapsed() {
            Ok(elapsed) => println!("Standard sort of {} (i64) -> {:?}", i, elapsed.as_micros()),
            Err(e) => {
                println!("error occured {:?}", e);
            }
        }

        if res_quick == sorted {
            println!("Quicksort fuking works")
        } else {
            panic!("Quicksort breaks here {:?}", my_vec)
        }
    }
}
fn generate_array_sizes(mut size: i64) -> Vec<i64> {
    /*
    increase array sizes as- [100,200..1000,2000,3000..,10000,20000,30000..100000,200000,3lc.10lcs,20lcs,30lcs,1cr
     */
    let mut vec_my = Vec::with_capacity(size as usize);
    let mut i: i64 = 10;
    let mut max = 10;
    while size > 0 {
        for mut c in 1..11 {
            if c == 10 {
                break;
            }
            vector_add(&mut vec_my, i * c);
            println!("{:?}", i * c)
        }
        i = i * max;
        size = size - 1;
    }
    println!("{:?}", vec_my);
    vec_my
}
fn vector_add<T>(vec_data: &mut Vec<T>, val: T) {
    vec_data.push(val);
}

fn nomain() {
    for i in generate_array_sizes(
        get_input(&String::from(
            "Give the max power of 10 you want to test for \
        (9*10^x would be the range or random list)",
        ))
        .trim()
        .parse::<i64>()
        .unwrap(),
    )
    .iter()
    {
        let mut my_vec: Vec<u16> = Vec::with_capacity(*i as usize);
        let mut rng = rand::thread_rng();

        for j in 0..*i {
            vector_add(&mut my_vec, rng.gen())
        }
        exit(0);
    }
}
