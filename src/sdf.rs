extern crate gen_random_proc_macro;
extern crate rand;
extern crate serde;
extern crate serde_cbor;

use gen_random::{GenRandom, GenRandomParams};
use gen_random_proc_macro::GenRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter, Write};

/// macro used to write to strings.
///
/// we're only writing numbers and strings so write! should never fail.
macro_rules! write_str {
	($( $arg:tt )*) => { write!($($arg)*).unwrap() }
}

/// parameters used to generate SDF
#[derive(Copy, Clone)]
pub struct SdfParams {
	/// maximum expression depth
	max_depth: i32,
}

impl Default for SdfParams {
	fn default() -> Self {
		Self { max_depth: 5 }
	}
}

impl GenRandomParams for SdfParams {
	fn inc_depth(self) -> Self {
		Self {
			max_depth: self.max_depth - 1,
		}
	}
}

/// constants across 3D space (but not across time/user input/etc.)
#[derive(Debug, GenRandom, Serialize, Deserialize, Clone, Copy)]
#[params(SdfParams)]
pub enum Constant {
	/// a number
	#[prob(0.0)]
	F32(f32),
	/// `Time(a, b) = a * time + b`, where `time` is in seconds
	#[prob(0.5)]
	Time(
		#[scale(0.2)]
		#[bias(-0.1)]
		f32,
		f32,
	),
}

/// a trait for string serialization
///
/// this is automatically implemented for anything with [Serialize] + [Deserialize].
pub trait ImportExport: Sized {
	/// export self as a string
	fn export_string(&self) -> String;
	/// import a string
	///
	/// returns None if `s` is not a valid serialized string
	fn import_string(s: &str) -> Option<Self>;
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
///
/// returns None if this isn't a valid hexadecimal string.
fn decode_hex(data: &str) -> Option<Vec<u8>> {
	let data = data.replace(char::is_whitespace, "");
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

impl<T: Serialize + for<'a> Deserialize<'a>> ImportExport for T {
	fn export_string(&self) -> String {
		let mut data: Vec<u8> = vec![];
		// write errors should never happen
		// that said, we don't want to panic if for whatever reason this fails.
		let _ = serde_cbor::to_writer(&mut data, self);
		encode_hex(&data)
	}

	fn import_string(s: &str) -> Option<Self> {
		let bytes = decode_hex(s)?;
		serde_cbor::from_reader(&bytes[..]).ok()?
	}
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

/// a `vec3` of [Constant]s
#[derive(GenRandom, Debug, Serialize, Deserialize)]
#[params(SdfParams)]
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

/// a generic function from float/vec*n* to float/vec*n*
#[derive(GenRandom, Debug, Serialize, Deserialize)]
#[params(SdfParams)]
pub enum RnToRn {
	/// 1/c sin(cx)
	#[prob(4)]
	#[bias(0.01)] // prevent division by 0
	Sin(Constant),
	/// based on modulus function
	#[prob(4)]
	#[bias(0.01)]
	InfiniteMirrors(Constant),
	/// arctan(c x) / c
	#[prob(2)]
	Arctan(Constant),
	/// based on 1/x² sin(x²)
	#[prob(2)]
	#[bias(0.01)]
	SqSin(Constant),
	/// based on sigmoid(x) = 1 / (1 + e^-x)
	#[prob(2)]
	#[bias(0.01)]
	Sigmoid,
	#[prob(2)]
	Wibbly,
	/// based on sqrt(x)
	#[prob(2)]
	Sqrt(Constant),
}

/// a function from vec3 to vec3
#[derive(GenRandom, Debug, Serialize, Deserialize)]
#[params(SdfParams)]
pub enum R3ToR3 {
	/// the identity function f(x) = x
	#[prob(0)]
	Identity,
	/// a composition of two functions
	#[prob(8)]
	#[only_if(params.max_depth >= 0)]
	Compose(Box<R3ToR3>, Box<R3ToR3>),
	/// f(x, y, z) = (f₁(x), f₂(y), f₃(z))
	#[prob(3)]
	#[only_if(params.max_depth >= 0)]
	PerComponent(Box<RToR>, Box<RToR>, Box<RToR>),
	/// f(x) = (f₁(x), f₂(x), f₃(x))
	#[prob(3)]
	#[only_if(params.max_depth >= 0)]
	Multiplex(Box<R3ToR>, Box<R3ToR>, Box<R3ToR>),
	/// a linear interpolation between two functions
	#[prob(4)]
	#[only_if(params.max_depth >= 0)]
	Mix(Box<R3ToR3>, Box<R3ToR3>, Constant),
	/// translate by a constant amount  f(x) = x + p
	#[prob(0.5)]
	Translate(Constant3),
	/// this was removed at some point.
	/// it doesn't really seem to be helpful.
	#[prob(0)]
	Twisty(Constant),
	/// a generic function applied to vec3
	#[prob(12)]
	NToN(Box<RnToRn>),
	/// rotate by a constant amount
	#[prob(0.5)]
	#[scale(2 * std::f32::consts::PI)]
	Rotate(Constant3),

	// ---- everything below has been moved to RnToRn and is only here for backwards compatibility ----
	#[doc(hidden)]
	#[prob(0)]
	Sin(Constant), // 1/c sin(cx)
	#[doc(hidden)]
	#[prob(0)]
	InfiniteMirrors(Constant),
	#[doc(hidden)]
	#[prob(0)]
	Arctan(Constant),
	#[doc(hidden)]
	#[prob(0)]
	SqSin(Constant),
	#[doc(hidden)]
	#[prob(0)]
	Sigmoid,
	#[doc(hidden)]
	#[prob(0)]
	Wibbly,
	#[doc(hidden)]
	#[prob(0)]
	Sqrt(Constant),
}

/// a function from float to float
#[derive(GenRandom, Debug, Serialize, Deserialize)]
#[params(SdfParams)]
pub enum RToR {
	/// the identity function f(x) = x
	#[prob(1)]
	Identity,
	/// composition of two functions
	#[prob(0)]
	#[only_if(params.max_depth >= 0)]
	Compose(Box<RToR>, Box<RToR>),
	/// subtract a constant
	#[prob(0)]
	Subtract(Constant),
	/// a generic function applied to float
	#[prob(2)]
	NToN(Box<RnToRn>),
}

/// a function from vec3 to float
#[derive(GenRandom, Debug, Serialize, Deserialize)]
#[params(SdfParams)]
pub enum R3ToR {
	/// SDF for a sphere
	#[prob(0.1)]
	Sphere(Constant),
	/// SDF for a cube
	#[prob(0.1)]
	Cube(Constant),
	/// SDF for a box frame
	#[prob(0.1)]
	BoxFrame {
		#[scale(3.0)]
		size: Constant,
		#[scale(0.2)]
		thickness: Constant,
	},
	/// SDF for a torus
	#[prob(0.1)]
	Torus {
		#[scale(3.0)]
		radius: Constant,
		#[scale(0.2)]
		thickness: Constant,
	},
	/// SDF for a triangular prism
	#[prob(0.1)]
	TriPrism(Constant, Constant),
	/// SDF for a vertical line segment
	#[prob(0.1)]
	VLineSegment(Constant),
	/// SDF for a cylinder
	#[prob(0.1)]
	Cylinder(Constant, Constant),
	/// apply a function from vec3 to vec3, then vec3 to float, then float to float
	#[prob(8)]
	#[only_if(params.max_depth >= 0)]
	Compose(Box<R3ToR3>, Box<R3ToR>, Box<RToR>),
	/// linear interpolation between two functions
	#[prob(4)]
	#[only_if(params.max_depth >= 0)]
	Mix(Box<R3ToR>, Box<R3ToR>, Constant),
	/// sin(f(x))·cos(g(x))
	#[prob(4)]
	#[only_if(params.max_depth >= 0)]
	SinCos(Box<R3ToR>, Box<R3ToR>),
	/// "smooth" minimum of two functions
	#[prob(2)]
	#[only_if(params.max_depth >= 0)]
	SmoothMin(Box<R3ToR>, Box<R3ToR>),
	/// minimum of two functions
	#[prob(2)]
	#[only_if(params.max_depth >= 0)]
	Min(Box<R3ToR>, Box<R3ToR>),
	/// f(x,y,z) = x
	#[prob(0.1)]
	ProjectX,
	/// f(x,y,z) = y
	#[prob(0.1)]
	ProjectY,
	/// f(x,y,z) = z
	#[prob(0.1)]
	ProjectZ,
}

impl R3ToR3 {
	/// create composition of two functions
	pub fn compose(self, b: Self) -> Self {
		Self::Compose(Box::new(self), Box::new(b))
	}
}

impl RToR {
	/// create composition of two functions
	pub fn compose(self, b: Self) -> Self {
		Self::Compose(Box::new(self), Box::new(b))
	}
}

impl R3ToR {
	/// create sphere SDF
	pub fn sphere_f32(r: f32) -> Self {
		Self::Sphere(r.into())
	}

	/// create cube SDF
	pub fn cube_f32(r: f32) -> Self {
		Self::Cube(r.into())
	}

	/// create mix of two SDFs
	pub fn mix(a: Self, b: Self, t: Constant) -> Self {
		Self::Mix(Box::new(a), Box::new(b), t)
	}

	/// create mix of two SDFs
	pub fn mix_f32(a: Self, b: Self, t: f32) -> Self {
		Self::mix(a, b, t.into())
	}

	/// create minimum of two SDFs
	pub fn min(a: Self, b: Self) -> Self {
		Self::Min(Box::new(a), Box::new(b))
	}

	/// create "smooth" minimum of two SDFs
	pub fn smooth_min(a: Self, b: Self) -> Self {
		Self::SmoothMin(Box::new(a), Box::new(b))
	}

	/// create composition of three functions
	pub fn compose(pre: R3ToR3, f: Self, post: RToR) -> Self {
		Self::Compose(Box::new(pre), Box::new(f), Box::new(post))
	}
}

impl Default for RToR {
	fn default() -> Self {
		Self::Identity
	}
}

impl Default for R3ToR3 {
	fn default() -> Self {
		Self::Identity
	}
}

impl Default for R3ToR {
	fn default() -> Self {
		Self::Sphere(Constant::F32(1.0))
	}
}

/// a variable in a GLSL program
#[derive(Clone, Copy)]
struct Variable {
	id: u32,
}

impl Display for Variable {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "v{}", self.id)
	}
}

/// a counter used to create variables
struct VarCounter {
	idx: u32,
}

impl VarCounter {
	fn new() -> Self {
		Self { idx: 0 }
	}

	/// get next variable
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

/// a trait implemented by [R3ToR3], [R3ToR], etc.
trait Function: Sized + Default + GenRandom<SdfParams> + ImportExport {
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

	/// generates a random function with the given length
	fn good_random(rng: &mut impl Rng, function_length: usize) -> Self {
		let default_len = Self::default().export_string().len();
		for max_depth in 1.. {
			let params = SdfParams { max_depth };
			let mut functions = vec![];
			for _i in 0..20 {
				let f = Self::gen_random_params(rng, params);
				let len = f.export_string().len().saturating_sub(default_len);
				functions.push((len, f));
			}
			functions.sort_by_key(|&(len, _)| len);
			if functions[functions.len() - 1].0 < function_length {
				// max_depth isn't large enough to get functions of this complexity
				continue;
			}
			let mut closest = 0;
			for (i, (len, _)) in functions.iter().enumerate() {
				if len.abs_diff(function_length) < functions[closest].0.abs_diff(function_length) {
					closest = i;
				}
			}
			let selected = functions.remove(closest);

			return selected.1;
		}
		// weird that rust thinks 1.. "might have zero elements to iterate on"
		panic!("wtf")
	}

	/// generates a random function with the given length using the thread random number generator
	fn good_thread_random(function_length: usize) -> Self {
		Self::good_random(&mut rand::thread_rng(), function_length)
	}
}

impl RnToRn {
	fn to_glsl(&self, input: Variable, code: &mut String, var: &mut VarCounter, n: u8) -> Variable {
		let r#type = match n {
			1 => "float",
			2 => "vec2",
			3 => "vec3",
			4 => "vec4",
			_ => panic!("bad n: {n}"),
		};
		use RnToRn::*;
		match self {
			Arctan(c) => {
				let output = var.next();
				// we need to scale arctan(cx) so it doesn't break the SDF
				write_str!(
					code,
					"{type} {output} = (1.0 / {c}) * atan({c} * {input});\n"
				);
				output
			}
			SqSin(c) => {
				let output = var.next();
				let a = var.next();
				write_str!(code, "{type} {a} = 0.1 + abs({input});\n");
				write_str!(code, "{a} *= {a};\n");
				write_str!(
					code,
					"{type} {output} = 0.7593/(pow({c},1.5)*{a}) * sin({c}*{a});\n"
				);
				output
			}
			Sigmoid => {
				let output = var.next();
				write_str!(
					code,
					"{type} {output} = 2.0 - abs(4.0 / (1.0 + exp(-{input})) - 2.0);\n"
				);
				output
			}
			Wibbly => {
				let output = var.next();
				write_str!(
					code,
					"{type} {output} = sqrt({input}*({input}+3*sin({input}))) * 0.39;\n"
				);
				output
			}
			Sqrt(c) => {
				let output = var.next();
				write_str!(
					code,
					"{type} {output} = sqrt({c} * abs({input}) + {c}*{c}) * 2.0;\n"
				);
				output
			}
			Sin(c) => {
				let output = var.next();
				write_str!(
					code,
					"{type} {output} = sin({c} * {input}) * (1.0 / {c});\n"
				);
				output
			}
			InfiniteMirrors(c) => {
				let q = var.next();
				let r = var.next();
				let output = var.next();
				write_str!(code, "{type} {q} = mod(floor({input} * {c}), 2.0);\n");
				write_str!(code, "{type} {r} = mod({input} * {c}, 1.0);\n");
				write_str!(
					code,
					"{type} {output} = (1.0 / {c}) * ({q} + {r} * (1.0 - 2 * {q}));\n"
				);
				output
			}
		}
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
			NToN(f) => f.to_glsl(input, code, var, 1),
		}
	}
}

impl Function for R3ToR3 {
	const INPUT_TYPE: GLSLType = GLSLType::Vec3;
	const OUTPUT_TYPE: GLSLType = GLSLType::Vec3;

	fn to_glsl(&self, input: Variable, code: &mut String, var: &mut VarCounter) -> Variable {
		use R3ToR3::*;

		match &self {
			Identity => input,
			Translate(by) => {
				let output = var.next();
				write_str!(code, "vec3 {output} = {input} + {by};\n");
				output
			}
			Compose(a, b) => {
				let a_output = a.to_glsl(input, code, var);
				b.to_glsl(a_output, code, var)
			}
			PerComponent(fx, fy, fz) => {
				let x_input = var.next();
				let y_input = var.next();
				let z_input = var.next();
				let output = var.next();
				write_str!(
					code,
					"float {x_input} = {input}.x;\n
					float {y_input} = {input}.y;\n
					float {z_input} = {input}.z;\n"
				);
				let x_output = fx.to_glsl(x_input, code, var);
				let y_output = fy.to_glsl(y_input, code, var);
				let z_output = fz.to_glsl(z_input, code, var);
				write_str!(
					code,
					"vec3 {output} = vec3({x_output}, {y_output}, {z_output});\n"
				);
				output
			}
			Multiplex(fx, fy, fz) => {
				// we need to scale by 1/sqrt(3) to get a valid SDF
				let a = var.next();
				write_str!(
					code,
					"vec3 {a} = {input} * (1.0 / sqrt(3.0));\n"
				);
				let output = var.next();
				let x_output = fx.to_glsl(a, code, var);
				let y_output = fy.to_glsl(a, code, var);
				let z_output = fz.to_glsl(a, code, var);
				write_str!(
					code,
					"vec3 {output} = vec3({x_output}, {y_output}, {z_output});\n"
				);
				output
			}
			Mix(a, b, t) => {
				let a_output = a.to_glsl(input, code, var);
				let b_output = b.to_glsl(input, code, var);
				let output = var.next();
				write_str!(
					code,
					"vec3 {output} = mix({a_output}, {b_output}, clamp({t}, 0.0, 1.0));\n"
				);
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
			Twisty(c) => {
				let s = var.next();
				let a = var.next();
				let theta = var.next();
				let output = var.next();
				write_str!(code, "vec3 {s} = {input} * {c};\n");
				write_str!(code, "vec2 {a} = vec2(cos({s}.x), sin({s}.y));\n");
				write_str!(code, "float {theta} = {s}.z * sqrt(2.0);\n");
				write_str!(
					code,
					"vec3 {output} = vec3({a}.x*cos({theta})+{a}.y*sin({theta}),
						{a}.y*cos({theta})-{a}.x*sin({theta}),{s}.z) * (1.0/(4.0 * {c}));\n"
				);
				output
			}
			NToN(f) => f.to_glsl(input, code, var, 3),
			Sin(c) => RnToRn::Sin(*c).to_glsl(input, code, var, 3),
			InfiniteMirrors(c) => RnToRn::InfiniteMirrors(*c).to_glsl(input, code, var, 3),
			SqSin(c) => RnToRn::SqSin(*c).to_glsl(input, code, var, 3),
			Arctan(c) => RnToRn::Arctan(*c).to_glsl(input, code, var, 3),
			Sigmoid => RnToRn::Sigmoid.to_glsl(input, code, var, 3),
			Wibbly => RnToRn::Wibbly.to_glsl(input, code, var, 3),
			Sqrt(c) => RnToRn::Sqrt(*c).to_glsl(input, code, var, 3),
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
			TriPrism(x, y) => {
				let output = var.next();
				write_str!(
					code,
					"float {output} = sdf_tri_prism({input}, vec2({x}, {y}));\n"
				);
				output
			}
			VLineSegment(h) => {
				let output = var.next();
				write_str!(
					code,
					"float {output} = sdf_vertical_line_segment({input}, {h});\n"
				);
				output
			}
			Cylinder(x, y) => {
				let output = var.next();
				write_str!(code, "float {output} = sdf_cylinder({input}, {x}, {y});\n");
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
			SinCos(a, b) => {
				let a = a.to_glsl(input, code, var);
				let b = b.to_glsl(input, code, var);
				let output = var.next();
				write_str!(code, "float {output} = sin({a}) * cos({b});\n");
				output
			}
			Compose(pre, f, post) => {
				let pre_output = pre.to_glsl(input, code, var);
				let f_output = f.to_glsl(pre_output, code, var);
				post.to_glsl(f_output, code, var)
			}
			ProjectX => {
				let output = var.next();
				write_str!(code, "float {output} = {input}.x;\n");
				output
			}
			ProjectY => {
				let output = var.next();
				write_str!(code, "float {output} = {input}.y;\n");
				output
			}
			ProjectZ => {
				let output = var.next();
				write_str!(code, "float {output} = {input}.z;\n");
				output
			}
		}
	}
}

impl R3ToR {
	pub fn good_random(rng: &mut impl Rng, length: usize) -> Self {
		<Self as Function>::good_random(rng, length)
	}

	pub fn good_thread_random(length: usize) -> Self {
		<Self as Function>::good_thread_random(length)
	}

	pub fn to_glsl_function(&self, name: &str, code: &mut String) {
		<Self as Function>::to_glsl_function(self, name, code);
	}
}

impl R3ToR3 {
	pub fn good_random(rng: &mut impl Rng, length: usize) -> Self {
		<Self as Function>::good_random(rng, length)
	}

	pub fn good_thread_random(length: usize) -> Self {
		<Self as Function>::good_thread_random(length)
	}

	pub fn to_glsl_function(&self, name: &str, code: &mut String) {
		<Self as Function>::to_glsl_function(self, name, code);
	}
}

/// options for generating a [Scene]
pub struct SceneConfig {
	pub sdf_length: usize,
	pub color_length: usize,
}

/// a "scene" (includes SDF and color function)
#[derive(Serialize, Deserialize, Default)]
pub struct Scene {
	pub sdf: R3ToR,
	pub color_function: R3ToR3,
}

impl Scene {
	/// generate a random scene
	pub fn good_random(rng: &mut impl Rng, config: &SceneConfig) -> Self {
		let sdf = R3ToR::good_random(rng, config.sdf_length);
		let color_function = R3ToR3::good_random(rng, config.color_length);
		Scene {
			sdf,
			color_function,
		}
	}
}
