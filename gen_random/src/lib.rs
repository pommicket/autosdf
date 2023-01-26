extern crate rand;
use rand::Rng;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::{Cell, RefCell};

/// parameters passed to [GenRandom] implementation
pub trait GenRandomParams: Sized + Default + Copy {
	/// If `params` is used to generate `object`, then `params.inc_depth()` will
	/// be used to generate `object`'s members.
	fn inc_depth(self) -> Self;
}

impl GenRandomParams for () {
	fn inc_depth(self) -> Self {
		()
	}
}


impl GenRandomParams for i64 {
	fn inc_depth(self) -> Self {
		self - 1
	}
}

/// Generate random structs and enums!
///
/// You don't need to implement this trait yourself — instead, use the `derive` macro:
/// ```
/// use gen_random_proc_macro::GenRandom;
/// use gen_random::{GenRandom, GenRandomParams};
///
/// #[derive(GenRandom, Debug)]
/// enum MyType {
///     // this variant will be chosen 7 / 10.5 = 2/3 of the time
///     #[prob(7)]
///     Variant1(f64),
///     // this variant will be chosen 3.5 / 10.5 = 1/3 of the time
///     #[prob(3.5)]
///     Variant2 {
///         // bias & scale attributes can be used for fields of type f32/f64.
///         // this makes `a` range from 2 to 6 (as opposed to the default 0 to 1).
///         #[bias(2.0)]
///         #[scale(4.0)]
///         a: f64,
///         // we can even include a randomly-generated MyType inside this MyType!
///         // be careful when doing this or else you might try to generate an infinite struct!
///         b: Box<MyType>
///     }
/// }
///
/// fn main() {
///     let my_value = MyType::gen_thread_random();
///     println!("{my_value:?}");
/// }
/// ```

pub trait GenRandom<Params: GenRandomParams>: Sized {
	/// Generate a random value with parameters.
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self;
	
	/// Generate a random value
	fn gen_random(rng: &mut impl Rng) -> Self {
		Self::gen_random_params(rng, Params::default())
	}
	
	/// Generate a random instance of this struct using `rand::thread_rng()` with parameters.
	fn gen_thread_random_params(params: Params) -> Self {
		let mut thread_rng = rand::thread_rng();
		Self::gen_random_params(&mut thread_rng, params)
	}
	
	/// Generate a random instance of this struct using `rand::thread_rng()`.
	fn gen_thread_random() -> Self {
		Self::gen_thread_random_params(Params::default())
	}
}

/// generate a [Vec] of `len` random items.
pub fn gen_random_vec<P: GenRandomParams, T: GenRandom<P>>(rng: &mut impl Rng, len: usize) -> Vec<T> {
	(0..len).map(|_| T::gen_random(rng)).collect()
}

/// generate a [Vec] of `len` random items using `rand::thread_rng()`.
pub fn gen_thread_random_vec<P: GenRandomParams, T: GenRandom<P>>(len: usize) -> Vec<T> {
	gen_random_vec(&mut rand::thread_rng(), len)
}

impl<Params: GenRandomParams> GenRandom<Params> for f32 {
	fn gen_random_params(rng: &mut impl Rng, _params: Params) -> Self {
		rng.gen_range(0.0..1.0)
	}
}

impl<Params: GenRandomParams> GenRandom<Params> for f64 {
	fn gen_random_params(rng: &mut impl Rng, _params: Params) -> Self {
		rng.gen_range(0.0..1.0)
	}
}

impl<Params: GenRandomParams, T: GenRandom<Params>> GenRandom<Params> for Box<T> {
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self {
		Box::new(T::gen_random_params(rng, params))
	}
}

impl<Params: GenRandomParams, T: GenRandom<Params>> GenRandom<Params> for [T; 1] {
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self {
		[T::gen_random_params(rng, params)]
	}
}

impl<Params: GenRandomParams, T: GenRandom<Params>> GenRandom<Params> for [T; 2] {
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self {
		[T::gen_random_params(rng, params), T::gen_random_params(rng, params)]
	}
}

impl<Params: GenRandomParams, T: GenRandom<Params>> GenRandom<Params> for [T; 3] {
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self {
		[T::gen_random_params(rng, params), T::gen_random_params(rng, params), T::gen_random_params(rng, params)]
	}
}

impl<Params: GenRandomParams, T: GenRandom<Params>> GenRandom<Params> for [T; 4] {
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self {
		[T::gen_random_params(rng, params), T::gen_random_params(rng, params), T::gen_random_params(rng, params), T::gen_random_params(rng, params)]
	}
}

impl<Params: GenRandomParams, T: GenRandom<Params>> GenRandom<Params> for (T, T) {
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self {
		(T::gen_random_params(rng, params), T::gen_random_params(rng, params))
	}
}

impl<Params: GenRandomParams, T: GenRandom<Params>> GenRandom<Params> for (T, T, T) {
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self {
		(T::gen_random_params(rng, params), T::gen_random_params(rng, params), T::gen_random_params(rng, params))
	}
}

impl<Params: GenRandomParams, T: GenRandom<Params>> GenRandom<Params> for (T, T, T, T) {
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self {
		(T::gen_random_params(rng, params), T::gen_random_params(rng, params), T::gen_random_params(rng, params), T::gen_random_params(rng, params))
	}
}

impl<Params: GenRandomParams, T: GenRandom<Params>> GenRandom<Params> for Rc<T> {
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self {
		Self::new(T::gen_random_params(rng, params))
	}
}

impl<Params: GenRandomParams, T: GenRandom<Params>> GenRandom<Params> for Arc<T> {
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self {
		Self::new(T::gen_random_params(rng, params))
	}
}

impl<Params: GenRandomParams, T: GenRandom<Params>> GenRandom<Params> for Cell<T> {
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self {
		Self::new(T::gen_random_params(rng, params))
	}
}

impl<Params: GenRandomParams, T: GenRandom<Params>> GenRandom<Params> for RefCell<T> {
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self {
		Self::new(T::gen_random_params(rng, params))
	}
}

impl<Params: GenRandomParams, T: GenRandom<Params>> GenRandom<Params> for Option<T> {
	fn gen_random_params(rng: &mut impl Rng, params: Params) -> Self {
		if rng.gen_range(0..2) == 0 {
			None
		} else {
			Some(T::gen_random_params(rng, params))
		}
	}
}


#[cfg(test)]
mod tests {
	extern crate gen_random_proc_macro;
	extern crate rand;
	use super::{gen_thread_random_vec, GenRandom, GenRandomParams};
	use gen_random_proc_macro::GenRandom;

	#[derive(GenRandom, Debug)]
	enum Test1 {
		#[prob(0.2)]
		A(f32),
		#[prob(0.8)]
		B(Option<f32>),
	}

	#[derive(GenRandom, Debug)]
	#[allow(dead_code)]
	enum Test2 {
		#[prob(0.1)]
		Variant1,
		#[prob(0.7)]
		Variant2 { x: f32, y: f64, z: Test1 },
		#[prob(0.2)]
		Variant3(f32, Box<Test2>),
	}

	#[derive(GenRandom, Debug)]
	enum LinkedList {
		#[prob(10)]
		Empty,
		#[prob(90)]
		Cons(f32, Box<LinkedList>),
	}

	#[derive(GenRandom, Debug)]
	#[params(i64)]
	enum BinaryTree {
		#[prob(1)]
		Empty,
		#[prob(99)]
		#[only_if(params >= 0)]
		Node(f64, Box<BinaryTree>, Box<BinaryTree>)
	}

	#[derive(GenRandom, Debug)]
	struct ScaleBias {
		#[bias(1.0)]
		#[scale(10.0)]
		a: f32,
		#[bias(2.0)]
		#[scale(0.0)]
		b: f32,
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

	#[test]
	fn scale_bias() {
		let sb: Vec<ScaleBias> = gen_thread_random_vec(10);
		println!("{sb:?}");
		for x in sb.iter() {
			if x.a < 1.0 || x.a > 11.0 {
				panic!("a field should be between 1 and 11; got {}", x.a);
			}
			if x.b != 2.0 {
				panic!("b field should be exactly 2; got {}", x.b);
			}
		}
	}
	
	#[test]
	fn binary_tree_params() {
		let bintree = BinaryTree::gen_thread_random_params(5);
		println!("{bintree:?}");
	}
}
