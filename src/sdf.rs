#![allow(dead_code)] // @TODO @TEMPORARY

use std::fmt::Write;

// we're only writing numbers and strings so write! should never fail.
macro_rules! write_str {
	($( $arg:tt )*) => { write!($($arg)*).unwrap() }
}

enum R3ToR3 {
	Identity,
	Translate([f32; 3])
}

enum RToR {
	Identity,
	Add(f32)
}

pub struct Sdf {
	pre: R3ToR3,
	post: RToR
}

impl RToR {
	/// treats `v<*initial value of var_idx>` as the input, and puts the output in `v<final value of *var_idx>`.
	fn to_glsl(&self, code: &mut String, var_idx: &mut u32) {
		use RToR::*;
		let input = *var_idx;
		
		match self {
			Identity => {}, // no code
			Add(x) => {
				*var_idx += 1;
				let output = *var_idx;
				write_str!(code, "float v{output} = v{input} + {x};\n");
			}
		}
	}
}

impl R3ToR3 {
	/// treats `v<*initial value of var_idx>` as the input, and puts the output in `v<final value of *var_idx>`.
	fn to_glsl(&self, code: &mut String, var_idx: &mut u32) {
		use R3ToR3::*;
		let input = *var_idx;
		
		match self {
			Identity => {}, // no code
			Translate([x, y, z]) => {
				*var_idx += 1;
				let output = *var_idx;
				write_str!(code, "vec3 v{output} = v{input} + vec3({x}, {y}, {z});\n");
			},
		}
	}
}

impl Sdf {
	/// test sphere
	pub fn sphere() -> Self {
		Self {
			pre: R3ToR3::Identity,
			post: RToR::Identity
		}
	}

	/// appends some glsl code including a function `float sdf(vec3 p) { ... }`
	pub fn to_glsl(&self, code: &mut String) {
		code.push_str("float sdf(vec3 p) {\n");
		// don't start out right next to the origin, since weird stuff might be happening there
		let origin_dist: f32 = 3.0;
		write_str!(code, "vec3 v0 = p - vec3(0,0,-{}.);\n", origin_dist);
		let mut var_idx = 0;
		self.pre.to_glsl(code, &mut var_idx);
		write_str!(code, "float v{} = length(v{}) - 1.0;\n", var_idx + 1, var_idx);
		var_idx += 1;
		self.post.to_glsl(code, &mut var_idx);
		write_str!(code, "return v{var_idx};\n");
		code.push('}');
	}
}
