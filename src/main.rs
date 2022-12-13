extern crate nalgebra;

mod sdl;
pub mod win;
pub mod sdf;

use nalgebra::{Vector3, Matrix3, Rotation3, Matrix4};
use std::time::Instant;
	
type Vec3 = Vector3<f32>;
type Mat3 = Matrix3<f32>;
type Mat4 = Matrix4<f32>;
type Rot3 = Rotation3<f32>;

struct View {
	pos: Vec3,
	yaw: f32,
	pitch: f32
}

impl Default for View {
	fn default() -> Self {
		Self {
			pos: Vec3::zeros(),
			yaw: 0.0,
			pitch: 0.0
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
	
	fn transform(&self) -> Mat4 {
		self.translation() * self.rotation().to_homogeneous()
	}
	
	fn inv_transform(&self) -> Mat4 {
		// this matrix should always be invertible
		self.transform().try_inverse().unwrap()
	}
}


fn try_main() -> Result<(), String> {
	let my_sdf = sdf::Sdf::sphere();

	let mut window = win::Window::new("AutoSDF", 1280, 720, true)
		.map_err(|e| format!("Error creating window: {e}"))?;
	
	let mut fshader_source = String::new();
	fshader_source.push_str("
IN vec2 pos;
uniform mat4 u_transform;
");
	my_sdf.to_glsl(&mut fshader_source);
	fshader_source.push_str("
#define ITERATIONS 20
#define AA_X 1
#define AA_Y 1



float fbm(vec3 p) {
    float t = 0.0;
    float freq = 24.0;
    mat3 m = mat3(cos(1.),sin(1.),0,
                 -sin(1.),cos(1.),0,
                 0,            0, 1) * mat3(
                  1,           0, 0,
                  0,      cos(1.),sin(1.),
                  0,     -sin(1.),cos(1.)
    );
    for(int i = 0; i < 5; i++)
    {
        p = m * p;
        t += pow(0.6, float(i)) * sin(freq*p.x)*sin(freq*p.y)*sin(freq*p.z);
        freq *= 2.0;
    }
    return t;
}
		
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
	float focal_length = 1.0;
	float min_dist = 10.;
	vec2 inv_screen_size = 1.0 / vec2(1280.0, 720.0); // @TODO
	vec2 aa_delta = inv_screen_size / vec2(AA_X, AA_Y);
	vec3 final_color = vec3(0);
	for (int m = 0; m < AA_X; m++) {
	for (int n = 0; n < AA_Y; n++) {
	vec2 aa_offset = vec2(float(m), float(n)) * aa_delta;
	vec3 absolute_pos = vec3(pos + aa_offset, -focal_length);
	vec3 delta = normalize(absolute_pos);
	int i;
	for (i = 0; i < ITERATIONS; i++) {
		vec3 p = (u_transform * vec4(absolute_pos, 1.0)).xyz;
		float dist = sdf(p);
		min_dist = min(min_dist, dist);
		if (dist <= 0.01) {
			float L = 0.3 + max(0., dot(normal(p), normalize(vec3(.8,1,.6))));
			final_color += L * vec3(1.0, 0.0, 0.0);
			break;
		} 
		if (dist > 100.0) break;//little optimization
		absolute_pos += dist * delta;
	}
	}
	}
	final_color *= 1.0 / (AA_X * AA_Y);
	o_color = vec4(final_color, 1.0);
}");
	
	println!("{fshader_source}");
	
	let program = window.create_program(
		"attribute vec2 v_pos;
		OUT vec2 pos;
		uniform float u_aspect_ratio;
		
		void main() {
			pos = v_pos * vec2(u_aspect_ratio, 1.0);
			gl_Position = vec4(v_pos, 0.0, 1.0);
		}",
		&fshader_source
	).map_err(|e| format!("Error compiling shader:\n{e}"))?;
	
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
	
	'mainloop: loop {
		while let Some(event) = window.next_event() {
			use win::Event::*;
			use win::Key::*;
			match event {
				Quit | KeyDown(Escape) => break 'mainloop,
				KeyDown(F1) => show_debug_info = !show_debug_info,
				MouseMotion { xrel, yrel, .. } => {
					view.yaw += xrel as f32 * 0.01;
					view.pitch += yrel as f32 * 0.01;
				},
				_ => {},
			}
		}
		
		window.viewport_full_screen();
		
		window.clear_screen(win::ColorF32::BLACK);
		window.use_program(&program);
		window.uniform1f("u_aspect_ratio", window.aspect_ratio());
		window.uniform4x4f("u_transform", view.inv_transform().as_slice());
		
		window.draw_array(&array);
		
		window.swap();
		if show_debug_info {
			println!("frame time = {:?}",frame_time.elapsed());
			frame_time = Instant::now();
		}
	}
	
	Ok(())
}

fn main() {
	if let Err(e) = try_main() {
		win::display_error_message(&e);
	}
}
