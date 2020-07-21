use rand::Rng;
use std::time::SystemTime;

pub fn get_input(mut name: &String) -> String {
    println!("{}", name);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    line
}

pub fn vector_add<T>(vec_data: &mut Vec<T>, val: T) {
    vec_data.push(val);
}

pub fn dequeue<T>(vec_data: &mut Vec<T>) -> Result<(), String> {
    if vec_data.is_empty() == false {
        vec_data.remove(0);
        println!("Dequeued one element from the given DataStructure");
        Ok(())
    } else {
        Err("How could you do that?\nSo mean! Vector didnt have anything at all!".to_owned())
    }
}

/*
Insertion sort is one of the most easiest sorting algorithms.
We loop through the array, compare each element with every other element and arrange it like wise
 */

pub fn insertion_sort<T: PartialOrd + std::fmt::Debug + Clone>(my_vec: &mut Vec<T>) -> u128 {
    println!("INSERTION SORT");
    let mut i = 0;
    let mut cp = my_vec.to_vec();
    let now = SystemTime::now();
    for i in 1..cp.len() {
        let mut j = i;
        while j > 0 && cp[j - 1] > cp[j] {
            cp.swap(j - 1, j);
            j -= 1;
        }
    }
    println!("{:?}", cp);
    println!("________________________________________________________________________________________________");
    match now.elapsed() {
        Ok(elapsed) => elapsed.as_millis(),
        Err(e) => {
            println!("error occured {:?}", e);
            1 as u128
        }
    }
}

/*
Selection Sort

 */
pub fn selection_sort<T: PartialOrd + std::fmt::Debug + Clone>(my_vec: &mut Vec<T>) -> u128 {
    println!("SELECTION SORT");
    let mut cp = my_vec.to_vec();
    let now = SystemTime::now();
    for i in 0..cp.len() {
        let mut min_index = i;
        //since everything towards the left would hav already been sorted, passing i+1
        for j in i + 1..cp.len() {
            if cp[j] < cp[min_index] {
                min_index = j;
            }
        }
        cp.swap(i, min_index);
    }
    println!("{:?}", cp);
    println!("________________________________________________________________________________________________");
    match now.elapsed() {
        Ok(elapsed) => elapsed.as_millis(),
        Err(e) => {
            println!("error occured {:?}", e);
            1 as u128
        }
    }
}

/*
Bubble sort

checks adjecant numbers, swaps them and continues.
smokes biggest value to the right
this can be a very bad thing if all values are in reverse order (may be, i dunno, will check)
 */

pub fn bubble_sort<T: PartialOrd + std::fmt::Debug + std::clone::Clone>(
    my_vec: &mut Vec<T>,
) -> u128 {
    println!("BUBBLE SORT");
    let mut cp = my_vec.to_vec();
    let now = SystemTime::now();
    for i in 0..cp.len() - 1 {
        for j in 0..cp.len() - 1 {
            if cp[j] > cp[j + 1] {
                cp.swap(j, j + 1);
            }
        }
    }
    println!("Sorted array \n {:?}", cp);
    //    println!("________________________________________________________________________________________________");

    match now.elapsed() {
        Ok(elapsed) => elapsed.as_millis(),
        Err(e) => {
            println!("error occured {:?}", e);
            1 as u128
        }
    }
}
