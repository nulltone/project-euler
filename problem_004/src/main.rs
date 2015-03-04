#![feature(env)]
#![feature(core)]

use std::env;
use std::num::Int;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please enter a number.");
        return;
    }

    let max_digits: u32 = match args[1].parse() {
        Ok(val) => {val},
        Err(_) => {
            println!("Cannot convert argument(s) to an unsigned int!");
            return;
        }
    };

    let max_number = 10.pow(max_digits as usize) - 1;
    println!("Max starting products: {} x {}", max_number, max_number);

    let value = calculate_max_palindrome(max_number);
    println!("Max palindrome: {}", value);
}

fn calculate_max_palindrome(number: u32) -> u32 {
    let mut highest_product = 0;

    for left_product in range(0, number + 1).rev() {
        for right_product in range(0, number + 1).rev() {
            let value: u32 = left_product * right_product;
            let value_str = value.to_string();
            if value_str == value_str.as_slice().chars().rev().collect::<String>() {
                if value > highest_product {
                    highest_product = value;
                }
            }
        }

        if (left_product * number) < highest_product {
            break;
        }
    }

    return highest_product;
}

