#![allow(dead_code)] // @TODO @TEMPORARY

use std::fmt::Write;

// we're only writing numbers and strings so write! should never fail.
macro_rules! write_str {
	($( $arg:tt )*) => { write!($($arg)*).unwrap() }
}

/// these are constant across 3D space, not across time/user input/etc.
pub enum Constant {
	F32(f32),
	Time(f32, f32),
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
	Translate(Constant3),
}

pub enum RToR {
	Identity,
	Add(Constant),
}

pub enum R3ToR {
	Sphere(Constant),
	Cube(Constant),
	PrePost(Box<R3ToR3>, Box<R3ToR>, Box<RToR>),
	Mix(Box<R3ToR>, Box<R3ToR>, Constant),
	SmoothMin(Box<R3ToR>, Box<R3ToR>),
	Min(Box<R3ToR>, Box<R3ToR>),
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
}

struct VarCounter {
	idx: u32,
}

impl VarCounter {
	fn new() -> Self {
		Self { idx: 0 }
	}

	fn prev(&self) -> u32 {
		assert!(self.idx != 0);
		self.idx - 1
	}

	fn next(&mut self) -> u32 {
		let ret = self.idx;
		self.idx += 1;
		ret
	}
}

pub struct Sdf {
	distance_function: R3ToR,
}

trait Function {
	/// treats `v<input>` as the input, and puts the output in `v<return value>`.
	fn to_glsl(&self, input: u32, code: &mut String, var: &mut VarCounter) -> u32;
}

impl Function for RToR {
	fn to_glsl(&self, input: u32, code: &mut String, var: &mut VarCounter) -> u32 {
		use RToR::*;

		match self {
			Identity => return input,
			Add(x) => {
				write_str!(
					code,
					"float v{} = v{input} + {};\n",
					var.next(),
					x.to_glsl()
				);
			}
		}
		var.prev()
	}
}

impl Function for R3ToR3 {
	fn to_glsl(&self, input: u32, code: &mut String, var: &mut VarCounter) -> u32 {
		use R3ToR3::*;

		match self {
			Identity => return input,
			Translate(by) => {
				write_str!(
					code,
					"vec3 v{} = v{input} + {};\n",
					var.next(),
					by.to_glsl()
				);
			}
		}

		var.prev()
	}
}

impl Function for R3ToR {
	fn to_glsl(&self, input: u32, code: &mut String, var: &mut VarCounter) -> u32 {
		use R3ToR::*;
		match self {
			// thanks to https://iquilezles.org/articles/distfunctions/ for
			// these SDFs.
			Sphere(r) => {
				let r = r.to_glsl();
				write_str!(code, "float v{} = length(v{input}) - {r};\n", var.next());
			}
			Cube(r) => {
				let r = r.to_glsl();
				let q = var.next();
				write_str!(code, "vec3 v{q} = abs(v{input}) - {r};\n");
				write_str!(
					code,
					"float v{} = length(max(v{q},0.0)) + min(max(v{q}.x,max(v{q}.y,v{q}.z)),0.0);\n",
					var.next()
				)
			}
			Mix(a, b, t) => {
				let t = t.to_glsl();
				let a_output = a.to_glsl(input, code, var);
				let b_output = b.to_glsl(input, code, var);
				write_str!(
					code,
					"float v{} = mix(v{a_output}, v{b_output}, clamp({t}, 0.0, 1.0));\n",
					var.next()
				);
			}
			_ => todo!(),
		}

		var.prev()
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

	/// appends some glsl code including a function `float sdf(vec3 p) { ... }`
	pub fn to_glsl(&self, code: &mut String) {
		code.push_str("float sdf(vec3 p) {\n");
		// don't start out right next to the origin, since weird stuff might be happening there
		let origin_dist: f32 = 3.0;
		let mut var = VarCounter::new();
		write_str!(
			code,
			"vec3 v{} = p - vec3(0,0,-{}.);\n",
			var.next(),
			origin_dist
		);
		let output = self.distance_function.to_glsl(var.prev(), code, &mut var);
		write_str!(code, "return v{output};\n");
		code.push('}');
	}
}
