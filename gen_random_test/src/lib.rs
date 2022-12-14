
#[cfg(test)]
mod tests {
	extern crate rand;
	extern crate gen_random_proc_macro;
	extern crate gen_random;
	use gen_random::{GenRandom, gen_thread_random_vec};
	use gen_random_proc_macro::GenRandom;
	
	#[derive(GenRandom, Debug)]
	enum Test1 {
		#[prob = 0.2]
		A(f32),
		#[prob = 0.8]
		B(Option<f32>)
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

	#[test]
	fn basic() {
		let tests1: Vec<Test1> = gen_thread_random_vec(10);
		println!("{tests1:?}");
	}
	
	#[test]
	fn many_types_of_variants() {
		let tests2: Vec<Test2> = gen_thread_random_vec(10);
		println!("{tests2:?}");
	}
	
	#[test]
	fn linked_list() {
		let ll = LinkedList::gen_thread_random();
		println!("{ll:?}");
	}
}
