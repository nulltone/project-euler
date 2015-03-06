fn main() {
    let mut smallest_num = 20;
    let increment = smallest_num;
    let dividers = [20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2];

    loop {
        let mut is_evenly_divisible = true;

        for div in dividers.iter() {
            if (smallest_num % *div) != 0 {
                smallest_num += increment;
                is_evenly_divisible = false;
                break;
            }
        }

        if is_evenly_divisible {
            break;
        }
    }

    println!("Smallest number: {}", smallest_num);
}

