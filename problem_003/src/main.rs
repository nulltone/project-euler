use std::collections::HashMap;

fn main() {
	let mut primes : HashMap<u64, bool> = HashMap::new();
	//let max_num: u64 = 600851475143;
	let max_num: u64 = 13195;

	println!("Primes: ");
	for x in 2..max_num {
		match primes.get(&x) {
			Some(_) => {
				continue;
			} 
			None => {}
		}

		primes.insert(x, true);
		let mut inc = x;
		while inc <= max_num {
				primes.insert(inc, false);
				inc += inc;
		}

		match primes.get(&x) {
			Some(t) => {
				if *t {
					println!("{}", *t);
				} 
			}
			None => {}
		}
	}
}
