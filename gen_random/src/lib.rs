extern crate rand;
use rand::Rng;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::{Cell, RefCell};

pub trait GenRandom: Sized {
	fn gen_random(rng: &mut impl Rng) -> Self;
	fn gen_thread_random() -> Self {
		let mut thread_rng = rand::thread_rng();
		Self::gen_random(&mut thread_rng)
	}
}

pub fn gen_random_vec<T: GenRandom>(rng: &mut impl Rng, len: usize) -> Vec<T> {
	(0..len).map(|_| T::gen_random(rng)).collect()
}

pub fn gen_thread_random_vec<T: GenRandom>(len: usize) -> Vec<T> {
	gen_random_vec(&mut rand::thread_rng(), len)
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

impl<T: GenRandom> GenRandom for [T; 1] {
	fn gen_random(rng: &mut impl Rng) -> Self {
		[T::gen_random(rng)]
	}
}

impl<T: GenRandom> GenRandom for [T; 2] {
	fn gen_random(rng: &mut impl Rng) -> Self {
		[T::gen_random(rng), T::gen_random(rng)]
	}
}

impl<T: GenRandom> GenRandom for [T; 3] {
	fn gen_random(rng: &mut impl Rng) -> Self {
		[T::gen_random(rng), T::gen_random(rng), T::gen_random(rng)]
	}
}

impl<T: GenRandom> GenRandom for [T; 4] {
	fn gen_random(rng: &mut impl Rng) -> Self {
		[T::gen_random(rng), T::gen_random(rng), T::gen_random(rng), T::gen_random(rng)]
	}
}

impl<T: GenRandom> GenRandom for (T, T) {
	fn gen_random(rng: &mut impl Rng) -> Self {
		(T::gen_random(rng), T::gen_random(rng))
	}
}

impl<T: GenRandom> GenRandom for (T, T, T) {
	fn gen_random(rng: &mut impl Rng) -> Self {
		(T::gen_random(rng), T::gen_random(rng), T::gen_random(rng))
	}
}

impl<T: GenRandom> GenRandom for (T, T, T, T) {
	fn gen_random(rng: &mut impl Rng) -> Self {
		(T::gen_random(rng), T::gen_random(rng), T::gen_random(rng), T::gen_random(rng))
	}
}

impl<T: GenRandom> GenRandom for Rc<T> {
	fn gen_random(rng: &mut impl Rng) -> Self {
		Self::new(T::gen_random(rng))
	}
}

impl<T: GenRandom> GenRandom for Arc<T> {
	fn gen_random(rng: &mut impl Rng) -> Self {
		Self::new(T::gen_random(rng))
	}
}

impl<T: GenRandom> GenRandom for Cell<T> {
	fn gen_random(rng: &mut impl Rng) -> Self {
		Self::new(T::gen_random(rng))
	}
}

impl<T: GenRandom> GenRandom for RefCell<T> {
	fn gen_random(rng: &mut impl Rng) -> Self {
		Self::new(T::gen_random(rng))
	}
}

impl<T: GenRandom> GenRandom for Option<T> {
	fn gen_random(rng: &mut impl Rng) -> Self {
		if rng.gen_range(0..2) == 0 {
			None
		} else {
			Some(T::gen_random(rng))
		}
	}
}

