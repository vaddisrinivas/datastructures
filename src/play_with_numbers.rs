use std::borrow::Borrow;
use std::io::stdin;
use std::num::ParseIntError;
use std::process::exit;

fn get_input(mut name: &String) -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line);
    line
}

winter is coming now
fn main() {
    let mut NQ: Vec<i64> = get_input(&String::from(""))
        .trim()
        .split(" ")
        .filter_map(|text| text.parse::<i64>().ok())
        .collect();
    let mut N = NQ.first().unwrap();
    let mut Q = NQ.last().unwrap();
    let mut my_vec: Vec<i64> = get_input(&String::from(""))
        .trim()
        .split(" ")
        .filter_map(|text| text.parse::<i64>().ok())
        .collect();
    let mut prev = 0;
    for i in 0..*N {
        prev = prev + my_vec[i as usize];
        my_vec[i as usize] = prev;
    }
    for i in 0..*Q {
        let mut line = String::new();
        stdin().read_line(&mut line).expect("Unable to read i & j");
        let mut line_parts = line.trim().split(" ");
        let n = line_parts
            .next()
            .expect("Unable to read i")
            .parse::<usize>()
            .expect("Unable to parse i");
        let o = line_parts
            .next()
            .expect("Unable to read j")
            .parse::<usize>()
            .expect("Unable to parse j");

        let sum = if n > 1 {
            my_vec[o - 1] - my_vec[n - 2]
        } else {
            my_vec[o - 1]
        };
        println!("{:?}", sum / ((o - n + 1) as i64));
    }
}
