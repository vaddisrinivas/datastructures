use rand::Rng;

pub fn get_input(mut name: &String) -> String {
    println!("{}", name);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    line
}

pub fn vector_init<T>(first_elem: T, capacity: usize) -> Vec<T> {
    let mut rng = rand::thread_rng();
    let mut vec_data: Vec<T> = Vec::with_capacity(capacity);
    vector_add(&mut vec_data, first_elem);
    vec_data
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

pub fn insertion_sort<T: PartialOrd + std::fmt::Debug + Clone>(my_vec: &mut Vec<T>) {
    println!("INSERTION SORT");
    let mut i = 0;
    let mut cp = my_vec.to_vec();
    println!("Given array {:?}", cp);
    for i in 1..cp.len() {
        let mut j = i;
        println!("Header at -> \t{:?}", cp[i]);
        while j > 0 && cp[j - 1] > cp[j] {
            cp.swap(j - 1, j);
            j -= 1;
        }
        println!("{:?}", cp)
    }
    println!("________________________________________________________________________________________________");
}

/*
Selection Sort

 */
pub fn selection_sort<T: PartialOrd + std::fmt::Debug + Clone>(my_vec: &mut Vec<T>) {
    println!("SELECTION SORT");
    let mut cp = my_vec.to_vec();
    println!("Given array {:?}", cp);
    for i in 0..cp.len() {
        let mut min_index = i;
        println!("Header at -> \t{:?}", cp[i]);
        //since everything towards the left would hav already been sorted, passing i+1
        for j in i + 1..cp.len() {
            if cp[j] < cp[i] {
                min_index = j;
            }
        }
        cp.swap(i, min_index);
        println!("{:?}", cp)
    }
    println!("________________________________________________________________________________________________");
}
