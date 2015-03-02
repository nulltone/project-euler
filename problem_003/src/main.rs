use std::collections::HashMap;

fn main() {
	let mut primes : HashMap<u64, bool> = HashMap::new();
	//let max_num: u64 = 600851475143;
	let max_num: u64 = 35;

	for x in 2..max_num {
        // Checks if the value has been inserted already.
		match primes.get(&x) {
			Some(&exists) => {
                // Value already inserted. Skipping.
                println!("exists - primes[{}]:{}", x, exists);
				continue;
			} 
			None => {
                // Inserts the prime if it doesn't exist.
                primes.insert(x, true);
                println!("inserted - primes[{}]:true", x);
            }
		}

        // Marks off all multiples of current values to false.
        let mut counter = 2;
		let mut inc = x * counter;
		while inc <= max_num {
				primes.insert(inc, false);
                //println!("--- marked - primes[{}]:false", inc);
                counter += 1;
				inc = x * counter;
		}
	}

    println!("Prime numbers: ");
    for (&num, &is_prime) in primes.iter() {
        if is_prime {
            println!("{}", num);
        }
    }
}

