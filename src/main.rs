/*
@TODO:
- auto-select level set by sampling a bunch of points
- bring time back, w pause/rewind/adjust time speed (start out paused?)
- Params instead of depth for GenRandom
   - allow multiple endpoints (cube & sphere & ...)
- seed control (maybe save seeds to a file then let user go back&forth through past sdfs)
- fullscreen key
- mathematical analysis
- options for:
	- max framerate
	- mouse sensitivity
	- fov, focal length
	- AA quality
	- # iterations, distance cutoff
- documentation
- GenRandom integers (+ gen_random_scale_bias

-----
cool seeds:
commit ae29a61c9917da5ad9fbb7a24151bff506669ffb "cool stuff"
18413841503509874975
**17878446840930313726
commit 35cbbb40298389efcd2fe87a9c6458d49c1c567e "add torus, box frame"
2876923889725946210
*12145962426879404199
commit d7f810524a30843417253f80e454f1d9173aaeb3 "more functions"
2607779313513160780
16956394651920792998
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
	rotation: Mat3,
	level_set: f32,
}

impl Default for View {
	fn default() -> Self {
		// don't start out right next to the origin, since weird stuff might be happening there
		let pos = Vec3::new(0.0, 0.0, 4.0);
		let rotation = Mat3::identity();
		Self {
			pos,
			rotation,
			level_set: 0.0,
		}
	}
}

impl View {
	/// `rotation() * vec3(0, 0, -1)` is the direction the camera is pointing
	fn rotation(&self) -> Mat3 {
		self.rotation
	}

	fn yaw_by(&mut self, yaw: f32) {
		self.rotation = self.rotation * Rot3::from_euler_angles(0.0, yaw, 0.0);
	}
	
	fn pitch_by(&mut self, pitch: f32) {
		self.rotation = self.rotation * Rot3::from_euler_angles(pitch, 0.0, 0.0);
	}

	fn translation(&self) -> Mat4 {
		Mat4::new_translation(&self.pos)
	}

	#[allow(dead_code)]
	fn transform(&self) -> Mat4 {
		self.translation() * self.rotation().to_homogeneous()
	}
}

fn gen_program_with_seed(window: &mut win::Window, program: &mut win::Program, seed: u64) -> Result<(), String> {
	use rand::SeedableRng;
	
	let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
	let my_sdf = sdf::R3ToR::good_random(&mut rng, 6);
	let color_function = sdf::R3ToR3::good_random(&mut rng, 7);
	
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
uniform int u_hsv;

float smooth_min(float a, float b, float k) {
	k = clamp(k, 0.0, 1.0);
	float h = max(k-abs(a-b), 0.0)/k;
	return min(a, b) - h*h*h*k*(1.0/6.0);
}

// thanks to https://iquilezles.org/articles/distfunctions/

float sdf_box_frame( vec3 p, vec3 b, float e ) {
       p = abs(p  )-b;
  vec3 q = abs(p+e)-e;
  return min(min(
      length(max(vec3(p.x,q.y,q.z),0.0))+min(max(p.x,max(q.y,q.z)),0.0),
      length(max(vec3(q.x,p.y,q.z),0.0))+min(max(q.x,max(p.y,q.z)),0.0)),
      length(max(vec3(q.x,q.y,p.z),0.0))+min(max(q.x,max(q.y,p.z)),0.0));
}

float sdf_torus(vec3 p, vec2 t) {
	vec2 q = vec2(length(p.xy)-t.x,p.z);
	return length(q)-t.y;
}
",
	);
	my_sdf.to_glsl_function("sdf", &mut fshader_source);
	color_function.to_glsl_function("get_color_", &mut fshader_source);
	fshader_source.push_str(
		"
		
// see https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB_alternative
float hsvf(float n, vec3 hsv) {
	float k = mod(n + hsv.x * 6.0, 6.0);
	return hsv.z - hsv.z * hsv.y * clamp(min(k, 4.0 - k), 0.0, 1.0);
}

vec3 hsv_to_rgb(vec3 hsv) {
	hsv.yz = clamp(hsv.yz, 0.0, 1.0);
	return vec3(hsvf(5.0, hsv), hsvf(3.0, hsv), hsvf(1.0, hsv));
}

vec3 get_color(vec3 p) {
	if (u_hsv != 0) {
		vec3 hsv = get_color_(p);
		// make sure object isn't too dark so we can actually see it
		hsv.z = mix(hsv.z, 1.0, 0.5);
		return hsv_to_rgb(hsv);
	} else {
		// we're not clamping this because it makes a cool glowing effect if we don't
		vec3 color = get_color_(p);
		return mix(color, vec3(1.0), 0.2);
	}
}

#define ITERATIONS 30
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
		o_color = vec4(get_color(p), 1.0);
		return;
	}
	int i;
	for (i = 0; i < ITERATIONS; i++) {
		float dist = sdf(p);
		min_dist = min(min_dist, dist);
		if (dist > 100.0) break;//little optimization
		p += dist * delta;
	}

	float threshold = 0.02;
	if (min_dist < threshold) {
		vec3 N = normal(p);
		vec3 light_direction = normalize(vec3(.8,1,.6));
		float L_diffuse = max(0., dot(N, light_direction));
		// Phong lighting
		vec3 R = reflect(light_direction, N);
		vec3 view_direction = u_rotation * vec3(0.0, 0.0, -1.0);
		// wikipedia calls this exponent the shininess (α)
		float shininess = 16.0;
		float L_specular = pow(max(0.0, dot(R, view_direction)), shininess);
		float brightness = (1.0/threshold) * (threshold-min_dist);
		brightness = pow(brightness, 16.0);
		float L_ambient = 0.3;
		vec3 color = get_color(p);
		float specularity = 0.15; // strength of specular lighting
		final_color += brightness * mix(mix(L_diffuse, 1.0, L_ambient) * color, vec3(L_specular), specularity);
		break;
	}
	
	}
	}
	final_color *= 1.0 / (AA_X * AA_Y);
	o_color = vec4(final_color, 1.0);
}",
	);

	//println!("{fshader_source}");
	println!("seed: {seed}");

	window
		.link_program(program,
			"IN vec2 v_pos;
		OUT vec2 pos;
		uniform float u_aspect_ratio;
		
		void main() {
			pos = v_pos * vec2(u_aspect_ratio, 1.0);
			gl_Position = vec4(v_pos, 0.0, 1.0);
		}",
			&fshader_source,
		)
		.map_err(|e| format!("Error compiling shader:\n{e}"))?;
	Ok(())
}

fn gen_program(window: &mut win::Window, program: &mut win::Program) -> Result<(), String> {
	let seed = rand::random::<u64>();
	gen_program_with_seed(window, program, seed)
}

fn try_main() -> Result<(), String> {

	let mut window = win::Window::new("AutoSDF", 1280, 720, true)
		.map_err(|e| format!("Error creating window: {e}"))?;
	let mut program = window.new_program();
	gen_program(&mut window, &mut program)?;

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
				KeyDown(R) => {
					gen_program(&mut window, &mut program)?;
					view.level_set = 0.0;
				},
				KeyDown(N0) => view.level_set = 0.0,
				MouseMotion { xrel, yrel, .. } => {
					let mouse_sensitivity = 0.05;
					view.yaw_by(-xrel as f32 * mouse_sensitivity * frame_dt);
					view.pitch_by(-yrel as f32 * mouse_sensitivity * frame_dt);
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
			let mut speed_multiplier = if window.is_shift_down() { 10.0 } else { 1.0 };
			speed_multiplier *= if window.is_ctrl_down() { 0.1 } else { 1.0 };
			
			let motion = Vec3::new(dx, dy, dz);
			if let Some(motion) = motion.try_normalize(0.001) {
				let move_speed = 4.0 * speed_multiplier;
				let motion = motion * frame_dt * move_speed;
				let motion = view.rotation() * motion;
				view.pos += motion;
			}

			let level_set_speed = 1.0 * speed_multiplier;
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
