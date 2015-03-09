use std::num::Int;

fn main() {
    let max_number = 100;
    let mut sum_squares = 0;
    let mut square_sum = 0;

    println!("Max number: {}", max_number);

    for num in 0..(max_number + 1) {
        sum_squares += num.pow(2);
        square_sum += num;
    }

    square_sum = square_sum.pow(2);
    
    println!("Sum of squares: {}, square of sum: {}, difference: {}", sum_squares, square_sum,
             (square_sum - sum_squares));
}
