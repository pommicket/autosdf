extern crate rand;
extern crate gen_random_proc_macro;
extern crate gen_random;
use gen_random::GenRandom;
use gen_random_proc_macro::GenRandom;

#[derive(GenRandom, Debug)]
enum Test1 {
	#[prob = 0.2]
	A(f32),
	#[prob = 0.8]
	B(f32)
}

#[derive(GenRandom, Debug)]
#[allow(dead_code)]
enum Test2 {
	#[prob = 0.1]
	Variant1,
	#[prob = 0.7]
	Variant2 { x : f32, y: f64, z: Test1 },
	#[prob = 0.2]
	Variant3(f32, Box<Test2>)
}

#[derive(GenRandom, Debug)]
enum LinkedList {
	#[prob = 0.1]
	Empty,
	#[prob = 0.9]
	Cons(f32, Box<LinkedList>)
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic() {
		let mut rng = rand::thread_rng();
		
		let tests1: Vec<_> = (0..10).map(|_| {
			Test1::gen_random(&mut rng)
		}).collect();
		println!("{tests1:?}");
	}
	
	#[test]
	fn many_types_of_variants() {
		let mut rng = rand::thread_rng();
		let tests2: Vec<_> = (0..10).map(|_| {
			Test2::gen_random(&mut rng)
		}).collect();
		println!("{tests2:?}");
	}
	
	#[test]
	fn linked_list() {
		let ll = LinkedList::gen_thread_random();
		println!("{ll:?}");
	}
}
