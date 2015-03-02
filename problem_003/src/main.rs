#![feature(env)]

use std::env;

fn main() {
    let args: Vec<String>= env::args().collect();

    if args.len() < 2 {
        println!("Please enter a number.");
        return;
    }

    let max_num: u64 = match args[1].parse() {
        Ok(val) => {val},
        Err(_) => {
            println!("Cannot convert string to an unsigned int!");
            return;
        }
    };

    let mut max_num_mut = max_num;
    let mut largest_prime = 1;
    let mut x = 2;

    while x <= max_num_mut {
        if max_num % x == 0 {
            largest_prime = x;
            max_num_mut /= x;
        }

        x += 1;
    }

    println!("Largest prime: {}", largest_prime);
}

