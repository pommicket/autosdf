/*
@TODO:
- fix rotation
- fullscreen key
- options for:
	- max framerate
	- mouse sensitivity
	- AA quality
	- # iterations, distance cutoff
*/

extern crate nalgebra;

pub mod sdf;
mod sdl;
pub mod win;

use nalgebra::{Matrix3, Matrix4, Rotation3, Vector3};
use std::time::Instant;

type Vec3 = Vector3<f32>;
type Mat3 = Matrix3<f32>;
type Mat4 = Matrix4<f32>;
type Rot3 = Rotation3<f32>;

struct View {
	pos: Vec3,
	yaw: f32,
	pitch: f32,
	level_set: f32,
}

impl Default for View {
	fn default() -> Self {
		// don't start out right next to the origin, since weird stuff might be happening there
		let pos = Vec3::new(0.0, 0.0, 4.0);

		Self {
			pos,
			yaw: 0.0,
			pitch: 0.0,
			level_set: 0.0,
		}
	}
}

impl View {
	/// `rotation() * vec3(0, 0, -1)` is the direction the camera is pointing
	fn rotation(&self) -> Mat3 {
		*Rot3::from_euler_angles(self.pitch, self.yaw, 0.0).matrix()
	}

	fn translation(&self) -> Mat4 {
		Mat4::new_translation(&self.pos)
	}

	#[allow(dead_code)]
	fn transform(&self) -> Mat4 {
		self.translation() * self.rotation().to_homogeneous()
	}
}

fn try_main() -> Result<(), String> {
	use sdf::{R3ToR};
	let funciton = R3ToR::min(
		R3ToR::sphere_f32(1.5),
		R3ToR::cube_f32(1.0),
	);
	let my_sdf = sdf::Sdf::from_function(funciton);

	let mut window = win::Window::new("AutoSDF", 1280, 720, true)
		.map_err(|e| format!("Error creating window: {e}"))?;

	let mut fshader_source = String::new();
	fshader_source.push_str(
		"
IN vec2 pos;
uniform mat3 u_rotation;
uniform vec3 u_translation;
uniform float u_time;
uniform float u_fov;
uniform float u_focal_length;
uniform float u_level_set;
",
	);
	my_sdf.to_glsl(&mut fshader_source);
	fshader_source.push_str(
		"
#define ITERATIONS 20
#define AA_X 1
#define AA_Y 1

float sdf_adjusted(vec3 p) {
	return sdf(p) - u_level_set;
}
#define sdf sdf_adjusted

vec3 normal(vec3 p)
{
// thanks to https://iquilezles.org/articles/normalsSDF/
    float h = 0.0001;
    vec2 k = vec2(1.,-1.);
    vec3 sdf_normal = k.xyy*sdf(p + k.xyy*h) + 
                      k.yyx*sdf(p + k.yyx*h) + 
                      k.yxy*sdf(p + k.yxy*h) + 
                      k.xxx*sdf(p + k.xxx*h);
    return normalize(sdf_normal);
}

void main() {
	float min_dist = 10.;
	vec2 inv_screen_size = 1.0 / vec2(1280.0, 720.0); // @TODO
	vec2 aa_delta = inv_screen_size / vec2(AA_X, AA_Y);
	vec3 final_color = vec3(0);
	for (int m = 0; m < AA_X; m++) {
	for (int n = 0; n < AA_Y; n++) {
	vec2 aa_offset = vec2(float(m), float(n)) * aa_delta;
	vec3 pos3d = vec3((pos + aa_offset) * sin(u_fov * 0.5), -1.0) * u_focal_length;
	vec3 p = u_rotation * pos3d;
	vec3 delta = normalize(p);
	p += u_translation;
	if (sdf(p) < 0.0) {
		// looking inside object
		o_color = vec4(1.0, 0.0, 1.0, 1.0);
		return;
	}
	int i;
	for (i = 0; i < ITERATIONS; i++) {
		float dist = sdf(p);
		min_dist = min(min_dist, dist);
		if (dist <= 0.01) {
			float L = 0.3 + max(0., dot(normal(p), normalize(vec3(.8,1,.6))));
			final_color += L * vec3(1.0, 0.0, 0.0);
			break;
		} 
		if (dist > 100.0) break;//little optimization
		p += dist * delta;
	}
	}
	}
	final_color *= 1.0 / (AA_X * AA_Y);
	o_color = vec4(final_color, 1.0);
}",
	);

	println!("{fshader_source}");

	let program = window
		.create_program(
			"attribute vec2 v_pos;
		OUT vec2 pos;
		uniform float u_aspect_ratio;
		
		void main() {
			pos = v_pos * vec2(u_aspect_ratio, 1.0);
			gl_Position = vec4(v_pos, 0.0, 1.0);
		}",
			&fshader_source,
		)
		.map_err(|e| format!("Error compiling shader:\n{e}"))?;

	let mut buffer = window.create_buffer();
	let data: &[[f32; 2]] = &[
		[-1.0, -1.0],
		[1.0, -1.0],
		[1.0, 1.0],
		[-1.0, -1.0],
		[1.0, 1.0],
		[-1.0, 1.0],
	];
	window.set_buffer_data(&mut buffer, data);
	let mut array = window.create_vertex_array(buffer, &program);
	window.array_attrib2f(&mut array, "v_pos", 0);

	let mut view = View::default();

	window.set_mouse_relative(true);

	let mut frame_time = Instant::now();
	let mut show_debug_info = false;
	let mut total_time = 0.0;

	'mainloop: loop {
		let frame_dt = frame_time.elapsed().as_secs_f32();
		frame_time = Instant::now();

		while let Some(event) = window.next_event() {
			use win::Event::*;
			use win::Key::*;
			match event {
				Quit | KeyDown(Escape) => break 'mainloop,
				KeyDown(F1) => show_debug_info = !show_debug_info,
				MouseMotion { xrel, yrel, .. } => {
					view.yaw -= xrel as f32 * frame_dt;
					view.pitch -= yrel as f32 * frame_dt;
				}
				_ => {}
			}
		}

		{
			// movement
			let mut dx = 0.0;
			let mut dy = 0.0;
			let mut dz = 0.0;
			let mut dl = 0.0;
			use win::Key::{
				Down, Left, NumPad3, NumPad9, PageDown, PageUp, Right, Up, A, D, E, M, N, Q, S, W,
			};
			if window.any_key_down(&[W, Up]) {
				dz -= 1.0;
			}
			if window.any_key_down(&[S, Down]) {
				dz += 1.0;
			}
			if window.any_key_down(&[A, Left]) {
				dx -= 1.0;
			}
			if window.any_key_down(&[D, Right]) {
				dx += 1.0;
			}
			if window.is_key_down(Q) {
				dy += 1.0;
			}
			if window.is_key_down(E) {
				dy -= 1.0;
			}
			if window.any_key_down(&[PageUp, NumPad9, M]) {
				dl += 1.0;
			}
			if window.any_key_down(&[PageDown, NumPad3, N]) {
				dl -= 1.0;
			}
			let motion = Vec3::new(dx, dy, dz);
			if let Some(motion) = motion.try_normalize(0.001) {
				let move_speed = 2.0;
				let motion = motion * frame_dt * move_speed;
				let motion = view.rotation() * motion;
				view.pos += motion;
			}

			let level_set_speed = 1.0;
			view.level_set += dl * frame_dt * level_set_speed;
		}

		window.viewport_full_screen();

		window.clear_screen(win::ColorF32::BLACK);
		window.use_program(&program);
		window.uniform1f("u_aspect_ratio", window.aspect_ratio());
		window.uniform1f("u_time", total_time);
		window.uniform1f("u_fov", std::f32::consts::PI * 0.25);
		window.uniform1f("u_focal_length", 1.0);
		window.uniform1f("u_level_set", view.level_set);
		window.uniform3x3f("u_rotation", view.rotation().as_slice());
		window.uniform3f_slice("u_translation", view.pos.as_slice());

		window.draw_array(&array);

		window.swap();
		if show_debug_info {
			println!("frame time = {:?}ms", frame_dt * 1000.0);
		}

		total_time += frame_dt;
	}

	Ok(())
}

fn main() {
	if let Err(e) = try_main() {
		win::display_error_message(&e);
	}
}
