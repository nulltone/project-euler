#![feature(env)]
#![feature(core)]

use std::env;
use std::num::Int;

fn main() {
    let args: Vec<String> = env::args().collect();

    check_args(&args);

    let max_digits: u32 = match args[1].parse() {
        Ok(val) => {val},
        Err(_) => {
            println!("Cannot convert argument(s) to an uint8!");
            return;
        }
    };

    let max_number = 10.pow(max_digits as usize) - 1;
    println!("Max starting products: {} x {}", max_number, max_number);

    let value = calculate_max_palindrome(max_number);
    println!("Max palindrome: {}", value);
}

fn check_args(args: &Vec<String>) {
    if args.len() < 2 {
        println!("Please enter a number.");
        return;
    }
}

fn calculate_max_palindrome(number: u32) -> u32 {
    let mut highest_product = 0;

    for left_product in range(0, number + 1).rev() {
        for right_product in range(0, number + 1).rev() {
            let value: u32 = left_product * right_product;
            if value < highest_product {
                break;
            }

            if is_palindrome(explode_digits(value)) {
                if value > highest_product {
                    highest_product = value;
                }
            }
        }
    }

    return highest_product;
}

fn explode_digits(number: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = Vec::new();
    let mut num = number;

    while num > 0 {
        digits.push(((num as u32) % 10) as u32);
        num /= 10;
    }

    return digits;
}

fn is_palindrome(exploded: Vec<u32>) -> bool {
    let mut x = 0;
    let mut y = exploded.len() - 1;

    if exploded.len() == 0 {
        return false;
    }

    while x != y && x < y {
        if exploded[x] == exploded[y] {
            x += 1;
            y -= 1;
            continue;
        } else {
            return false;
        }
    }

    return true;
}

