#![allow(dead_code)] // @TODO @TEMPORARY

use std::fmt::{self, Write, Display, Formatter};

// we're only writing numbers and strings so write! should never fail.
macro_rules! write_str {
	($( $arg:tt )*) => { write!($($arg)*).unwrap() }
}


/// these are constant across 3D space, not across time/user input/etc.
pub enum Constant {
	F32(f32),
	Time(f32, f32),
	// @TODO: sin, sqrt, mod
}

impl From<f32> for Constant {
	fn from(x: f32) -> Self {
		Self::F32(x)
	}
}

impl Constant {
	fn to_glsl(&self) -> String {
		use Constant::*;
		match self {
			F32(x) => format!("{x:.1}"),
			Time(x, y) => format!("({x:.1} * u_time + {y:.1})"),
		}
	}
}

pub struct Constant3(Constant, Constant, Constant);

impl From<(Constant, Constant, Constant)> for Constant3 {
	fn from(x: (Constant, Constant, Constant)) -> Self {
		Self(x.0, x.1, x.2)
	}
}

impl Constant3 {
	fn to_glsl(&self) -> String {
		format!(
			"vec3({}, {}, {})",
			self.0.to_glsl(),
			self.1.to_glsl(),
			self.2.to_glsl()
		)
	}
}

pub enum R3ToR3 {
	Identity,
	Compose(Box<R3ToR3>, Box<R3ToR3>),
	Translate(Constant3),
	Sin(Constant),
	//@TODO InfiniteMirrors(f32)
}

pub enum RToR {
	Identity,
	Compose(Box<RToR>, Box<RToR>),
	Add(Constant),
}

pub enum R3ToR {
	Sphere(Constant),
	Cube(Constant),
	Compose(Box<R3ToR3>, Box<R3ToR>, Box<RToR>),
	Mix(Box<R3ToR>, Box<R3ToR>, Constant),
	SmoothMin(Box<R3ToR>, Box<R3ToR>),
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
	id: u32
}

impl Display for Variable {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
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

	fn prev(&self) -> Variable {
		assert!(self.idx != 0);
		Variable { id: self.idx - 1 }
	}

	fn next(&mut self) -> Variable {
		let ret = Variable { id: self.idx};
		self.idx += 1;
		ret
	}
}

pub struct Sdf {
	distance_function: R3ToR,
}

trait Function {
	/// appends `code` with glsl code to apply the function to the input variable.
	/// returns the output variable.
	#[must_use]
	fn to_glsl(&self, input: Variable, code: &mut String, var: &mut VarCounter) -> Variable;
}

impl Function for RToR {
	fn to_glsl(&self, input: Variable, code: &mut String, var: &mut VarCounter) -> Variable {
		use RToR::*;

		match self {
			Identity => input,
			Add(x) => {
				let output = var.next();
				write_str!(
					code,
					"float {output} = {input} + {};\n",
					x.to_glsl()
				);
				output
			}
			Compose(a, b) => {
				let a_output = a.to_glsl(input, code, var);
				let b_output = b.to_glsl(a_output, code, var);
				b_output
			},
		}
	}
}

impl Function for R3ToR3 {
	fn to_glsl(&self, input: Variable, code: &mut String, var: &mut VarCounter) -> Variable {
		use R3ToR3::*;

		match self {
			Identity => input,
			Translate(by) => {
				let output = var.next();
				write_str!(
					code,
					"vec3 {output} = {input} + {};\n",
					by.to_glsl()
				);
				output
			}
			Sin(c) => {
				// we shouldn't just do sin(c x), since that
				// would multiply the derivative by c (which breaks the SDF if c > 1)
				// so we'll do sin(c x) / c instead.
				let c = c.to_glsl();
				let output = var.next();
				write_str!(
					code,
					"vec3 {output} = sin({c} * {input}) * (1.0 / {c});\n"
				);
				output
			},
			Compose(a, b) => {
				let a_output = a.to_glsl(input, code, var);
				let b_output = b.to_glsl(a_output, code, var);
				b_output
			}
		}
	}
}

impl Function for R3ToR {
	fn to_glsl(&self, input: Variable, code: &mut String, var: &mut VarCounter) -> Variable {
		use R3ToR::*;
		match self {
			// thanks to https://iquilezles.org/articles/distfunctions/ for
			// these SDFs.
			Sphere(r) => {
				let r = r.to_glsl();
				let output = var.next();
				write_str!(code, "float {output} = length({input}) - {r};\n");
				output
			}
			Cube(r) => {
				let r = r.to_glsl();
				let q = var.next();
				write_str!(code, "vec3 {q} = abs({input}) - {r};\n");
				let output = var.next();
				write_str!(
					code,
					"float {output} = length(max({q},0.0)) + min(max({q}.x,max({q}.y,{q}.z)),0.0);\n"
				);
				output
			}
			Mix(a, b, t) => {
				let t = t.to_glsl();
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
				write_str!(
					code,
					"float {output} = min({a_output}, {b_output});\n"
				);
				output
			},
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
					"float {output} = sdf_smooth_min({a_output}, {b_output}, {k});\n"
				);
				output
			},
			Compose(pre, f, post) => {
				let pre_output = pre.to_glsl(input, code, var);
				let f_output = f.to_glsl(pre_output, code, var);
				let post_output = post.to_glsl(f_output, code, var);
				post_output
			},
		}
	}
}

impl Sdf {
	/// test sphere
	pub fn sphere(r: f32) -> Self {
		Self {
			distance_function: R3ToR::Sphere(Constant::F32(r)),
		}
	}

	pub fn from_function(distance_function: R3ToR) -> Self {
		Self { distance_function }
	}

	/// appends some glsl code including a function `float sdf(vec3) { ... }`
	pub fn to_glsl(&self, code: &mut String) {
		code.push_str("
float sdf_smooth_min(float a, float b, float k) {
	k = clamp(k, 0.0, 1.0);
	float h = max(k-abs(a-b), 0.0)/k;
	return min(a, b) - h*h*h*k*(1.0/6.0);
}
");
		code.push_str("float sdf(vec3 p) {\n");
		let mut var = VarCounter::new();
		write_str!(code, "vec3 {} = p;\n", var.next());
		let output = self.distance_function.to_glsl(var.prev(), code, &mut var);
		write_str!(code, "return {output};\n");
		code.push('}');
	}
}
