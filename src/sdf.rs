extern crate gen_random_proc_macro;
extern crate rand;
extern crate serde;
extern crate serde_cbor;

use gen_random::GenRandom;
use gen_random_proc_macro::GenRandom;
use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter, Write};

// we're only writing numbers and strings so write! should never fail.
macro_rules! write_str {
	($( $arg:tt )*) => { write!($($arg)*).unwrap() }
}

/// these are constant across 3D space, not across time/user input/etc.
#[derive(Debug, GenRandom, Serialize, Deserialize)]
pub enum Constant {
	#[prob(0.5)]
	F32(f32),
	#[prob(0)]
	Time(
		#[scale(0.2)]
		#[bias(-0.1)]
		f32,
		f32,
	),
}

impl From<f32> for Constant {
	fn from(x: f32) -> Self {
		Self::F32(x)
	}
}

impl std::ops::Add<f32> for Constant {
	type Output = Self;
	fn add(self, a: f32) -> Self::Output {
		use Constant::*;
		match self {
			F32(x) => F32(x + a),
			Time(s, b) => Time(s, b + a),
		}
	}
}

impl std::ops::Mul<f32> for Constant {
	type Output = Self;
	fn mul(self, m: f32) -> Self::Output {
		use Constant::*;
		match self {
			F32(x) => F32(x * m),
			Time(s, b) => Time(s * m, b * m),
		}
	}
}

impl Display for Constant {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use Constant::*;
		match self {
			F32(x) => write!(f, "{x:.1}"),
			Time(x, y) => write!(f, "({x:.1} * u_time + {y:.1})"),
		}
	}
}

#[derive(GenRandom, Debug, Serialize, Deserialize)]
pub struct Constant3(Constant, Constant, Constant);

impl std::ops::Add<f32> for Constant3 {
	type Output = Self;
	fn add(self, a: f32) -> Self::Output {
		Self(self.0 + a, self.1 + a, self.2 + a)
	}
}

impl std::ops::Mul<f32> for Constant3 {
	type Output = Self;
	fn mul(self, m: f32) -> Self::Output {
		Self(self.0 * m, self.1 * m, self.2 * m)
	}
}

impl From<(Constant, Constant, Constant)> for Constant3 {
	fn from(x: (Constant, Constant, Constant)) -> Self {
		Self(x.0, x.1, x.2)
	}
}

impl Display for Constant3 {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		let Self(x, y, z) = self;
		write!(f, "vec3({x}, {y}, {z})")
	}
}

#[derive(GenRandom, Debug, Serialize, Deserialize)]
pub enum R3ToR3 {
	#[prob(0)]
	Identity,
	#[prob(6)]
	Compose(Box<R3ToR3>, Box<R3ToR3>),
	#[prob(1)]
	Translate(Constant3),
	#[prob(2)]
	#[bias(0.01)] // prevent division by 0
	Sin(Constant), // 1/c sin(cx)
	#[prob(2)]
	#[bias(0.01)]
	InfiniteMirrors(Constant),
	#[prob(2)]
	#[scale(2 * std::f32::consts::PI)]
	Rotate(Constant3),
	#[prob(2)]
	Arctan(Constant), // arctan(c x)  / c
	#[prob(2)]
	#[bias(0.01)]
	SqSin(Constant), // based on 1/x² sin(x²)
	#[prob(2)]
	#[bias(0.01)]
	Sigmoid, //based on sigmoid(x) = 1 / (1 + e^-x)
}

// note : i dont think R → R transformations really accomplish that much
// that can't be done with R³ → R³.
#[derive(GenRandom, Debug, Serialize, Deserialize)]
pub enum RToR {
	#[prob(1)]
	Identity,
	#[prob(0)]
	Compose(Box<RToR>, Box<RToR>),
	#[prob(0)]
	Subtract(Constant),
}

#[derive(GenRandom, Debug, Serialize, Deserialize)]
pub enum R3ToR {
	#[prob(1)]
	Sphere(Constant),
	#[prob(1)]
	Cube(Constant),
	#[prob(1)]
	BoxFrame {
		#[scale(3.0)]
		size: Constant,
		#[scale(0.2)]
		thickness: Constant,
	},
	#[prob(1)]
	Torus {
		#[scale(3.0)]
		radius: Constant,
		#[scale(0.2)]
		thickness: Constant,
	},
	#[prob(8)]
	Compose(Box<R3ToR3>, Box<R3ToR>, Box<RToR>),
	#[prob(4)]
	Mix(Box<R3ToR>, Box<R3ToR>, Constant),
	#[prob(2)]
	SmoothMin(Box<R3ToR>, Box<R3ToR>),
	#[prob(2)]
	Min(Box<R3ToR>, Box<R3ToR>),
}

impl R3ToR3 {
	pub fn compose(self, b: Self) -> Self {
		Self::Compose(Box::new(self), Box::new(b))
	}
}

impl RToR {
	pub fn compose(self, b: Self) -> Self {
		Self::Compose(Box::new(self), Box::new(b))
	}
}

impl R3ToR {
	pub fn sphere_f32(r: f32) -> Self {
		Self::Sphere(r.into())
	}

	pub fn cube_f32(r: f32) -> Self {
		Self::Cube(r.into())
	}

	pub fn mix(a: Self, b: Self, t: Constant) -> Self {
		Self::Mix(Box::new(a), Box::new(b), t)
	}

	pub fn mix_f32(a: Self, b: Self, t: f32) -> Self {
		Self::mix(a, b, t.into())
	}

	pub fn min(a: Self, b: Self) -> Self {
		Self::Min(Box::new(a), Box::new(b))
	}

	pub fn smooth_min(a: Self, b: Self) -> Self {
		Self::SmoothMin(Box::new(a), Box::new(b))
	}

	pub fn compose(pre: R3ToR3, f: Self, post: RToR) -> Self {
		Self::Compose(Box::new(pre), Box::new(f), Box::new(post))
	}
}

#[derive(Clone, Copy)]
struct Variable {
	id: u32,
}

impl Display for Variable {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "v{}", self.id)
	}
}

struct VarCounter {
	idx: u32,
}

impl VarCounter {
	fn new() -> Self {
		Self { idx: 0 }
	}

	fn next(&mut self) -> Variable {
		let ret = Variable { id: self.idx };
		self.idx += 1;
		ret
	}
}

/// a type in GLSL. this doesn't have all the types.
enum GLSLType {
	Float,
	Vec3,
}

impl fmt::Display for GLSLType {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use GLSLType::*;
		match self {
			Float => write!(f, "float"),
			Vec3 => write!(f, "vec3"),
		}
	}
}

trait Function: Sized + GenRandom {
	/// appends `code` with glsl code to apply the function to the input variable.
	/// returns the output variable.
	#[must_use]
	fn to_glsl(&self, input: Variable, code: &mut String, var: &mut VarCounter) -> Variable;

	/// GLSL type which is the input to this function
	const INPUT_TYPE: GLSLType;
	/// GLSL type which is the output of this function
	const OUTPUT_TYPE: GLSLType;

	/// adds GLSL code for function to `code`.
	fn to_glsl_function(&self, name: &str, code: &mut String) {
		let mut var = VarCounter::new();
		let input = var.next();
		write_str!(
			code,
			"{} {name}({} {input}) {{\n",
			Self::OUTPUT_TYPE,
			Self::INPUT_TYPE
		);
		let output = self.to_glsl(input, code, &mut var);
		write_str!(code, "return {output};\n}}\n\n");
	}

	fn good_random(rng: &mut impl Rng, max_depth: isize) -> Self {
		// to make sure the function isn't too boring or too slow,
		// we'll generate a bunch then take the one with the median code length.
		let mut functions = vec![];
		for _i in 0..20 {
			let f = Self::gen_random_max_depth(rng, max_depth);
			let mut code = String::new();
			let mut var = VarCounter::new();
			let _ = f.to_glsl(var.next(), &mut code, &mut var);
			let len = code.len();

			functions.push((len, f));
		}
		functions.sort_by_key(|x| x.0);
		functions.remove(functions.len() / 2).1
	}

	fn good_thread_random(max_depth: isize) -> Self {
		Self::good_random(&mut rand::thread_rng(), max_depth)
	}
}

impl Function for RToR {
	const INPUT_TYPE: GLSLType = GLSLType::Float;
	const OUTPUT_TYPE: GLSLType = GLSLType::Float;

	fn to_glsl(&self, input: Variable, code: &mut String, var: &mut VarCounter) -> Variable {
		use RToR::*;

		match self {
			Identity => input,
			Subtract(x) => {
				let output = var.next();
				write_str!(code, "float {output} = {input} - {x};\n");
				output
			}
			Compose(a, b) => {
				let a_output = a.to_glsl(input, code, var);
				b.to_glsl(a_output, code, var)
			}
		}
	}
}

impl Function for R3ToR3 {
	const INPUT_TYPE: GLSLType = GLSLType::Vec3;
	const OUTPUT_TYPE: GLSLType = GLSLType::Vec3;

	fn to_glsl(&self, input: Variable, code: &mut String, var: &mut VarCounter) -> Variable {
		use R3ToR3::*;

		match self {
			Identity => input,
			Translate(by) => {
				let output = var.next();
				write_str!(code, "vec3 {output} = {input} + {by};\n");
				output
			}
			Sin(c) => {
				// we shouldn't just do sin(c x), since that
				// would multiply the derivative by c (which breaks the SDF if c > 1)
				// so we'll do sin(c x) / c instead.
				let output = var.next();
				write_str!(code, "vec3 {output} = sin({c} * {input}) * (1.0 / {c});\n");
				output
			}
			InfiniteMirrors(c) => {
				// similar to Sin(c), but uses mod instead
				let q = var.next();
				let r = var.next();
				let output = var.next();
				write_str!(code, "vec3 {q} = mod(floor({input} * {c}), 2.0);\n");
				write_str!(code, "vec3 {r} = mod({input} * {c}, 1.0);\n");
				write_str!(
					code,
					"vec3 {output} = (1.0 / {c}) * ({q} + {r} * (1.0 - 2 * {q}));\n"
				);
				output
			}
			Compose(a, b) => {
				let a_output = a.to_glsl(input, code, var);
				b.to_glsl(a_output, code, var)
			}
			Arctan(c) => {
				let output = var.next();
				// we need to scale arctan(cx) so it doesn't break the SDF
				write_str!(code, "vec3 {output} = (1.0 / {c}) * atan({c} * {input});\n");
				output
			}
			Rotate(by) => {
				// by = euler angles
				// see https://en.wikipedia.org/wiki/Rotation_matrix#General_rotations
				// for matrix
				// this is the RzRyRx one
				let c = var.next();
				let s = var.next();
				let m = var.next();
				let output = var.next();
				write_str!(code, "vec3 {c} = cos({by});\n");
				write_str!(code, "vec3 {s} = sin({by});\n");
				write_str!(
					code,
					"mat3 {m} = mat3(
{c}.y*{c}.z, {s}.x*{s}.y*{c}.z - {c}.x*{s}.z, {c}.x*{s}.y*{c}.z + {s}.x*{s}.z,
{c}.y*{s}.z, {s}.x*{s}.y*{s}.z + {c}.x*{c}.z, {c}.x*{s}.y*{s}.z - {s}.x*{c}.z,
-{s}.y,    {s}.x*{c}.y,               {c}.x*{c}.y
);\n"
				);
				write_str!(code, "vec3 {output} = {m} * {input};\n");
				output
			}
			SqSin(c) => {
				let output = var.next();
				let a = var.next();
				write_str!(code, "vec3 {a} = 0.1 + abs({input});\n");
				write_str!(code, "{a} *= {a};\n");
				write_str!(
					code,
					"vec3 {output} = 0.7593/(pow({c},1.5)*{a}) * sin({c}*{a});\n"
				);
				output
			}
			Sigmoid => {
				let output = var.next();
				write_str!(
					code,
					"vec3 {output} = 2.0 - abs(4.0 / (1.0 + exp(-{input})) - 2.0);\n"
				);
				output
			}
		}
	}
}

impl Function for R3ToR {
	const INPUT_TYPE: GLSLType = GLSLType::Vec3;
	const OUTPUT_TYPE: GLSLType = GLSLType::Float;

	fn to_glsl(&self, input: Variable, code: &mut String, var: &mut VarCounter) -> Variable {
		use R3ToR::*;
		match self {
			// thanks to https://iquilezles.org/articles/distfunctions/ for
			// these SDFs.
			Sphere(r) => {
				let output = var.next();
				write_str!(code, "float {output} = length({input}) - {r};\n");
				output
			}
			Cube(r) => {
				let q = var.next();
				write_str!(code, "vec3 {q} = abs({input}) - {r};\n");
				let output = var.next();
				write_str!(
					code,
					"float {output} = length(max({q},0.0)) + min(max({q}.x,max({q}.y,{q}.z)),0.0);\n"
				);
				output
			}
			BoxFrame { size, thickness } => {
				let output = var.next();
				write_str!(
					code,
					"float {output} = sdf_box_frame({input}, vec3({size}), {thickness});\n"
				);
				output
			}
			Torus { radius, thickness } => {
				let output = var.next();
				write_str!(
					code,
					"float {output} = sdf_torus({input}, vec2({radius}, {thickness}));\n"
				);
				output
			}
			Mix(a, b, t) => {
				let a_output = a.to_glsl(input, code, var);
				let b_output = b.to_glsl(input, code, var);
				let output = var.next();
				write_str!(
					code,
					"float {output} = mix({a_output}, {b_output}, clamp({t}, 0.0, 1.0));\n"
				);
				output
			}
			Min(a, b) => {
				let a_output = a.to_glsl(input, code, var);
				let b_output = b.to_glsl(input, code, var);
				let output = var.next();
				write_str!(code, "float {output} = min({a_output}, {b_output});\n");
				output
			}
			SmoothMin(a, b) => {
				let a_output = a.to_glsl(input, code, var);
				let b_output = b.to_glsl(input, code, var);
				let output = var.next();
				// for now we're using a fixed k value
				// i don't want to make this a Constant right now,
				// since most values of k (i.e. <0, >1) look bad/just like min.
				let k = 0.2;
				write_str!(
					code,
					"float {output} = smooth_min({a_output}, {b_output}, {k});\n"
				);
				output
			}
			Compose(pre, f, post) => {
				let pre_output = pre.to_glsl(input, code, var);
				let f_output = f.to_glsl(pre_output, code, var);
				post.to_glsl(f_output, code, var)
			}
		}
	}
}

/// encode `data` in hexadecimal
fn encode_hex(data: &[u8]) -> String {
	let mut s = String::with_capacity(data.len() * 2);
	for byte in data {
		write_str!(s, "{byte:02x}");
	}
	s
}

/// decode `data` from hexadecimal.
/// returns None if this isn't a valid hexadecimal string.
fn decode_hex(data: &str) -> Option<Vec<u8>> {
	let data = data.trim();
	if data.len() % 2 != 0 {
		return None;
	}

	let mut bytes = Vec::with_capacity(data.len() / 2);
	for i in 0..data.len() / 2 {
		let s = data.get(2 * i..2 * i + 2)?;
		let byte = u8::from_str_radix(s, 16).ok()?;
		bytes.push(byte);
	}
	Some(bytes)
}

impl R3ToR {
	pub fn good_random(rng: &mut impl Rng, max_depth: isize) -> Self {
		<Self as Function>::good_random(rng, max_depth)
	}

	pub fn good_thread_random(max_depth: isize) -> Self {
		<Self as Function>::good_thread_random(max_depth)
	}

	pub fn to_glsl_function(&self, name: &str, code: &mut String) {
		<Self as Function>::to_glsl_function(self, name, code);
	}
}

impl R3ToR3 {
	pub fn good_random(rng: &mut impl Rng, max_depth: isize) -> Self {
		<Self as Function>::good_random(rng, max_depth)
	}

	pub fn good_thread_random(max_depth: isize) -> Self {
		<Self as Function>::good_thread_random(max_depth)
	}

	pub fn to_glsl_function(&self, name: &str, code: &mut String) {
		<Self as Function>::to_glsl_function(self, name, code);
	}
}

pub struct SceneConfig {
	pub sdf_max_depth: isize,
	pub color_max_depth: isize
}

#[derive(Serialize, Deserialize)]
pub struct Scene {
	pub sdf: R3ToR,
	pub color_function: R3ToR3,
}

impl Default for Scene {
	/// a sphere. pretty boring
	fn default() -> Self {
		Self {
			sdf: R3ToR::Sphere(Constant::F32(1.0)),
			color_function: R3ToR3::Identity,
		}
	}
}

impl Scene {
	pub fn export_string(&self) -> String {
		let mut data: Vec<u8> = vec![];
		// write errors should never happen
		// that said, we don't want to panic if for whatever reason this fails.
		let _ = serde_cbor::to_writer(&mut data, self);
		encode_hex(&data)
	}

	/// returns None if `s` is not a valid SDF string
	pub fn import_string(s: &str) -> Option<Self> {
		let bytes = decode_hex(s)?;
		serde_cbor::from_reader(&bytes[..]).ok()?
	}
	
	pub fn good_random(rng: &mut impl Rng, config: &SceneConfig) -> Self {
		let sdf = R3ToR::good_random(rng, config.sdf_max_depth);
		let color_function = R3ToR3::good_random(rng, config.color_max_depth);
		Scene {
			sdf,
			color_function,
		}
	}
}
