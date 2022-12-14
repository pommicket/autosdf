extern crate rand;
use rand::Rng;

pub trait GenRandom: Sized {
	fn gen_random(rng: &mut impl Rng) -> Self;
	fn gen_thread_random() -> Self {
		let mut thread_rng = rand::thread_rng();
		Self::gen_random(&mut thread_rng)
	}
}

impl GenRandom for f32 {
	fn gen_random(rng: &mut impl Rng) -> Self {
		rng.gen_range(0.0..1.0)
	}
}

impl GenRandom for f64 {
	fn gen_random(rng: &mut impl Rng) -> Self {
		rng.gen_range(0.0..1.0)
	}
}

impl<T: GenRandom> GenRandom for Box<T> {
	fn gen_random(rng: &mut impl Rng) -> Self {
		Box::new(T::gen_random(rng))
	}
}
