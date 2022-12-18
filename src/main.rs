/*
@TODO:
- bring time back, w pause/rewind/adjust time speed (start out paused?)
- fullscreen key
- options for:
	- max framerate
	- mouse sensitivity
	- fov, focal length
	- AA quality
	- # iterations, distance cutoff
---release---
- switch framebuffer texture to grayscale
- show that   θ = σ(z) / sqrt(x² + y²)
			  (x,y,z) → (x cosθ + y sinθ, y cosθ - x sinθ, z)
  is lipschitz continuous, & add it
- feedback for copy/paste (flash screen or something)
- Params instead of depth for GenRandom
   - allow multiple endpoints (cube & sphere & ...)
- save seeds to a file then let user go back&forth through past sdfs
- mathematical analysis
- documentation
- GenRandom integers (+ gen_random_scale_bias)
- better SDL api: Context  +  Window<'a> impl !Send+!Sync

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
3714031566539178742
---
the era of serialization
a263736466a169536d6f6f74684d696e82a169536d6f6f74684d696e82a1634d696e82a166537068657265a163463332fa3ee5212ca167436f6d706f736583a167436f6d706f736582a167436f6d706f736582684964656e74697479684964656e74697479675369676d6f6964a166537068657265a163463332fa3f7bf840684964656e74697479a1634d697883a165546f727573a266726164697573a163463332fa3f1b29f269746869636b6e657373a163463332fa3e08d8c8a167436f6d706f736583a16f496e66696e6974654d6972726f7273a163463332fa3e59bc38a16443756265a163463332fa3f155e80684964656e74697479a163463332fa3f42e49aa168426f784672616d65a26473697a65a163463332fa4034884d69746869636b6e657373a163463332fa3d0bd15a6e636f6c6f725f66756e6374696f6ea167436f6d706f736582a16641726374616ea163463332fa3eea41e4a16f496e66696e6974654d6972726f7273a163463332fa3f5ddffe
a263736466a167436f6d706f736583a166526f7461746583a163463332fa3f76cab2a163463332fa3d81cad0a163463332fa3f76ebd4a1634d696e82a167436f6d706f736583675369676d6f6964a1634d696e82a1634d697883a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3e5a3e68684964656e74697479a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3f46ade4684964656e74697479a163463332fa3f551da4a1634d696e82a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3f306be8684964656e74697479a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3ca99ac0684964656e74697479684964656e74697479a167436f6d706f736583a16f496e66696e6974654d6972726f7273a163463332fa3e9febeca167436f6d706f736583a16f496e66696e6974654d6972726f7273a163463332fa3ee05424a167436f6d706f736583675369676d6f6964a1634d696e82a166537068657265a163463332fa3e16dcf0a166537068657265a163463332fa3f48f0dc684964656e74697479684964656e74697479684964656e74697479684964656e746974796e636f6c6f725f66756e6374696f6ea167436f6d706f736582a16353696ea163463332fa3f1c2a8e675369676d6f6964
a263736466a167436f6d706f736583a165537153696ea163463332fa3e784c98a1634d697883a166537068657265a163463332fa3ea1ce4ca1634d696e82a166537068657265a163463332fa3f55f124a167436f6d706f736583675369676d6f6964a166537068657265a163463332fa3ef84fb0684964656e74697479a163463332fa3d2f72c0684964656e746974796e636f6c6f725f66756e6374696f6ea16f496e66696e6974654d6972726f7273a163463332fa3e9d85d0
a263736466a1634d696e82a1634d696e82a166537068657265a163463332fa3f2365aca1634d697883a167436f6d706f736583a16641726374616ea163463332fa3eb04a5ca167436f6d706f736583675369676d6f6964a169536d6f6f74684d696e82a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3eeb25b8684964656e74697479a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3f10d6a2684964656e74697479684964656e74697479684964656e74697479a165546f727573a266726164697573a163463332fa3d9099f069746869636b6e657373a163463332fa3e00b102a163463332fa3f06b5a2a1634d696e82a165546f727573a266726164697573a163463332fa40121b0169746869636b6e657373a163463332fa3e32f4faa1634d697883a16443756265a163463332fa3f7f9dc8a1634d697883a165546f727573a266726164697573a163463332fa3f9286e369746869636b6e657373a163463332fa3d8a3f27a167436f6d706f736583a167436f6d706f736582a167436f6d706f736582684964656e74697479684964656e74697479a16f496e66696e6974654d6972726f7273a163463332fa3ea62688a168426f784672616d65a26473697a65a163463332fa3fc0fb4969746869636b6e657373a163463332fa3e472462684964656e74697479a163463332fa3f69a73ea163463332fa3f5b9c9e6e636f6c6f725f66756e6374696f6ea16f496e66696e6974654d6972726f7273a163463332fa3ef4ea8c
a263736466a169536d6f6f74684d696e82a1634d696e82a167436f6d706f736583a167436f6d706f736582a16641726374616ea163463332fa3e6d7230a1695472616e736c61746583a163463332fa3e7262f8a163463332fa3eece0eca163463332fa3f49c42ca168426f784672616d65a26473697a65a163463332fa3ff3ee1169746869636b6e657373a163463332fa3df6dfed684964656e74697479a167436f6d706f736583a16f496e66696e6974654d6972726f7273a163463332fa3f2a2de8a165546f727573a266726164697573a163463332fa3fc93f1e69746869636b6e657373a163463332fa3e0fa700684964656e74697479a169536d6f6f74684d696e82a169536d6f6f74684d696e82a1634d696e82a169536d6f6f74684d696e82a167436f6d706f736583675369676d6f6964a1634d696e82a166537068657265a163463332fa3e0586b0a166537068657265a163463332fa3f4d6214684964656e74697479a167436f6d706f736583a167436f6d706f736582684964656e74697479684964656e74697479a1634d697883a166537068657265a163463332fa3f5c46e6a166537068657265a163463332fa3f3f4896a163463332fa3f10ba30684964656e74697479a166537068657265a163463332fa3ef3b604a167436f6d706f736583a165537153696ea163463332fa3ef1bfb8a1634d697883a1634d697883a168426f784672616d65a26473697a65a163463332fa3ee4801069746869636b6e657373a163463332fa3dc84d37a168426f784672616d65a26473697a65a163463332fa4019d5d269746869636b6e657373a163463332fa3d5d0307a163463332fa3eb7554ca167436f6d706f736583a165537153696ea163463332fa3f31c33ca167436f6d706f736583684964656e74697479a166537068657265a163463332fa3e7db3c0684964656e74697479684964656e74697479a163463332fa3f3176c0684964656e74697479a165546f727573a266726164697573a163463332fa3f253f0c69746869636b6e657373a163463332fa3c96c48d6e636f6c6f725f66756e6374696f6ea16f496e66696e6974654d6972726f7273a163463332fa3ea219f4
*/

extern crate nalgebra;

pub mod sdf;
mod sdl;
pub mod win;

use nalgebra::{Matrix3, Matrix4, Rotation3, Vector3};
use std::time::Instant;
use win::ColorF32;

type Vec3 = Vector3<f32>;
type Mat3 = Matrix3<f32>;
type Mat4 = Matrix4<f32>;
type Rot3 = Rotation3<f32>;

#[derive(Clone)]
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
		self.rotation *= Rot3::from_euler_angles(0.0, yaw, 0.0);
	}

	fn pitch_by(&mut self, pitch: f32) {
		self.rotation *= Rot3::from_euler_angles(pitch, 0.0, 0.0);
	}

	fn translation(&self) -> Mat4 {
		Mat4::new_translation(&self.pos)
	}

	#[allow(dead_code)]
	fn transform(&self) -> Mat4 {
		self.translation() * self.rotation().to_homogeneous()
	}
}

struct Programs {
	main: win::Program,
	test: win::Program,
}

impl Programs {
	fn new(window: &mut win::Window) -> Self {
		Programs {
			main: window.new_program(),
			test: window.new_program(),
		}
	}

	fn load_scene(&mut self, window: &mut win::Window, scene: &sdf::Scene) -> Result<(), String> {
		let source_main = include_str!("fshader_main.glsl");
		let source_test = include_str!("fshader_test.glsl");
		let source_common = include_str!("fshader_common.glsl");

		let mut sdf = String::new();
		let mut get_color = String::new();
		scene.sdf.to_glsl_function("sdf", &mut sdf);
		scene
			.color_function
			.to_glsl_function("get_color_", &mut get_color);
		let source_main = source_main
			.replace("%SDF%", &sdf)
			.replace("%COLOR%", &get_color)
			.replace("%COMMON%", source_common);
		let source_test = source_test
			.replace("%SDF%", &sdf)
			.replace("%COMMON%", source_common);

		//println!("{fshader_source}");
		println!("scene: {}", scene.export_string());

		window
			.link_program(
				&mut self.main,
				"IN vec2 v_pos;
			OUT vec2 pos;
			uniform float u_aspect_ratio;
			
			void main() {
				pos = v_pos * vec2(u_aspect_ratio, 1.0);
				gl_Position = vec4(v_pos, 0.0, 1.0);
			}",
				&source_main,
			)
			.map_err(|e| format!("Error compiling shader:\n{e}"))?;
		window
			.link_program(
				&mut self.test,
				"IN vec2 v_pos;
			OUT vec2 pos;
			
			void main() {
				pos = v_pos;
				gl_Position = vec4(v_pos, 0.0, 1.0);
			}",
				&source_test,
			)
			.map_err(|e| format!("Error compiling shader:\n{e}"))?;
		Ok(())
	}
}

fn get_rng() -> impl rand::Rng {
	use rand::SeedableRng;
	rand::rngs::SmallRng::seed_from_u64(rand::random::<u64>())
}

// sample size when testing to find default level set
const TEST_HEIGHT: u16 = 100;
const TEST_WIDTH: u16 = 100;

struct State {
	window: win::Window,
	view: View,
	initial_view: View,
	show_debug_info: bool,
	total_time: f64,
	frame_time: Instant,
	programs: Programs,
	config: sdf::SceneConfig,
	scene: sdf::Scene,
	framebuffer_texture: win::Texture,
	framebuffer: win::Framebuffer,
	main_array: win::VertexArray,
	test_array: win::VertexArray,
}

impl State {
	fn new() -> Result<Self, String> {
		let mut window = win::Window::new("AutoSDF", 1280, 720, true)
			.map_err(|e| format!("Error creating window: {e}"))?;
		let mut programs = Programs::new(&mut window);
		let config = sdf::SceneConfig {
			sdf_max_depth: 7,
			color_max_depth: 6,
		};
		let scene = sdf::Scene::good_random(&mut get_rng(), &config);
		programs
			.load_scene(&mut window, &scene)
			.unwrap_or_else(|e| eprintln!("Error: {e}"));
		//gen_program_from_string(&mut window, &mut program, "a263736466a167436f6d706f736583a1695472616e736c61746583a163463332fa3ea4c00ca163463332fa3e85dc00a163463332fa3f2bbdaea167436f6d706f736583a166526f7461746583a163463332fa3f750dc2a163463332fa3f5a7f0ea163463332fa3f2df98ca1634d696e82a167436f6d706f736583a167436f6d706f736582a16353696ea163463332fa3f7cc2a0a167436f6d706f736582684964656e74697479684964656e74697479a166537068657265a163463332fa3f26f8f6684964656e74697479a167436f6d706f736583a166526f7461746583a163463332fa3f1bfed8a163463332fa3f1e1e30a163463332fa3eddc6b0a1634d697883a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3ea149ec684964656e74697479a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3f6b0018684964656e74697479a163463332fa3e60a8d8684964656e74697479684964656e74697479684964656e746974796e636f6c6f725f66756e6374696f6ea165537153696ea163463332fa3ebaa7ec")?;

		let mut framebuffer_texture = window.create_texture(&Default::default());
		// we don't really care if there's an error. not much bad will happen.
		let _ = window.set_texture_no_data::<win::ColorF32>(
			&mut framebuffer_texture,
			TEST_WIDTH.into(),
			TEST_HEIGHT.into(),
		);

		let mut framebuffer = window.create_framebuffer();
		window.set_framebuffer_texture(
			&mut framebuffer,
			win::FramebufferAttachment::Color0,
			&framebuffer_texture,
		);

		let mut main_buffer = window.create_buffer();
		let mut test_buffer = window.create_buffer();
		let data: &[[f32; 2]] = &[
			[-1.0, -1.0],
			[1.0, -1.0],
			[1.0, 1.0],
			[-1.0, -1.0],
			[1.0, 1.0],
			[-1.0, 1.0],
		];
		window.set_buffer_data(&mut main_buffer, data);
		window.set_buffer_data(&mut test_buffer, data);
		let main_array = window.create_vertex_array(main_buffer, &programs.main);
		let test_array = window.create_vertex_array(test_buffer, &programs.test);
		
		window.set_mouse_relative(true);

		let mut me = Self {
			window,
			programs,
			view: View::default(),
			initial_view: View::default(),
			config,
			frame_time: Instant::now(),
			show_debug_info: false,
			total_time: 0.0,
			scene: sdf::Scene::default(),
			framebuffer_texture,
			framebuffer,
			main_array,
			test_array,
		};
		me.load_scene(scene);
		Ok(me)
	}

	fn load_scene(&mut self, scene: sdf::Scene) {
		match self.programs.load_scene(&mut self.window, &scene) {
			Ok(()) => {
				self.scene = scene;
				// *technically speaking* the location of v_pos could change between reloads
				self.window.array_attrib2f(&mut self.main_array, "v_pos", 0);
				self.window.array_attrib2f(&mut self.test_array, "v_pos", 0);

				// percentage of space occupied by object
				let frac = 0.25;
				// determine default level set
				// specifically we want to select y such that
				//   for ~frac of p values, sdf(p) < y
				self.window.bind_framebuffer(Some(&self.framebuffer));
				self.window
					.viewport(0, 0, TEST_WIDTH.into(), TEST_HEIGHT.into());

				self.window.use_program(&self.programs.test);
				self.window.draw_array(&self.test_array);

				self.window.viewport_full_screen();
				self.window.bind_framebuffer(None);

				let mut sdf_values: Vec<f32> = self
					.window
					.get_texture_data_vec::<ColorF32>(&self.framebuffer_texture)
					.iter()
					.map(|c| c.r)
					.collect();
				let i = (sdf_values.len() as f64 * frac) as usize;
				let level_set = *sdf_values
					.select_nth_unstable_by(i, |a, b| a.total_cmp(b))
					.1;
				drop(sdf_values);
				let mut initial_view = View::default();
				initial_view.level_set = level_set;
				self.initial_view = initial_view.clone();
				self.view = initial_view;
			}
			Err(e) => {
				eprintln!("Error: {e}")
			}
		};
	}

	// returns false if we should quit
	fn frame(&mut self) -> bool {
		let frame_dt = self.frame_time.elapsed().as_secs_f32();
		self.frame_time = Instant::now();
		self.total_time += f64::from(frame_dt);

		while let Some(event) = self.window.next_event() {
			use win::Event::*;
			use win::Key::*;
			match event {
				Quit | KeyDown { key: Escape, .. } => return false,
				KeyDown { key: F1, .. } => self.show_debug_info = !self.show_debug_info,
				KeyDown { key: R, .. } => {
					let new_scene = sdf::Scene::good_random(&mut get_rng(), &self.config);
					self.load_scene(new_scene);
				}
				KeyDown {
					key: C, modifier, ..
				} if modifier.ctrl() => {
					// copy scene
					match self.window.set_clipboard_text(&self.scene.export_string()) {
						Ok(()) => {}
						Err(e) => {
							eprintln!("couldn't copy text to clipboard: {e}")
						}
					}
				}
				KeyDown {
					key: V, modifier, ..
				} if modifier.ctrl() => {
					// paste scene
					match self.window.get_clipboard_text() {
						Ok(s) => match sdf::Scene::import_string(&s) {
							Some(new_scene) => {
								self.load_scene(new_scene);
							}
							None => {
								eprintln!("bad string")
							}
						},
						Err(e) => {
							// very unlikely to happen
							eprintln!("couldn't get clipboard text: {e}")
						}
					}
				}
				KeyDown { key: N0, .. } => self.view = self.initial_view.clone(),
				MouseMotion { xrel, yrel, .. } => {
					let mouse_sensitivity = 0.05;
					self.view
						.yaw_by(-xrel as f32 * mouse_sensitivity * frame_dt);
					self.view
						.pitch_by(-yrel as f32 * mouse_sensitivity * frame_dt);
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
			let window = &self.window;
			use win::Key::*;
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
			if window.any_key_down(&[PageUp, NumPad9, Equals]) {
				dl += 1.0;
			}
			if window.any_key_down(&[PageDown, NumPad3, Minus]) {
				dl -= 1.0;
			}
			let mut speed_multiplier = if window.is_shift_down() { 10.0 } else { 1.0 };
			speed_multiplier *= if window.is_ctrl_down() { 0.1 } else { 1.0 };

			let motion = Vec3::new(dx, dy, dz);
			if let Some(motion) = motion.try_normalize(0.001) {
				let move_speed = 4.0 * speed_multiplier;
				let motion = motion * frame_dt * move_speed;
				let motion = self.view.rotation() * motion;
				self.view.pos += motion;
			}

			let level_set_speed = 1.0 * speed_multiplier;
			self.view.level_set += dl * frame_dt * level_set_speed;
		}

		let window = &mut self.window;
		let view = &self.view;
		window.viewport_full_screen();

		window.clear_screen(win::ColorF32::BLACK);
		window.use_program(&self.programs.main);
		window.uniform1f("u_aspect_ratio", window.aspect_ratio());
		window.uniform1f("u_time", self.total_time as f32);
		window.uniform1f("u_fov", std::f32::consts::PI * 0.25);
		window.uniform1f("u_focal_length", 1.0);
		window.uniform1f("u_level_set", view.level_set);
		window.uniform1i("u_hsv", 0);
		window.uniform3x3f("u_rotation", view.rotation().as_slice());
		window.uniform3f_slice("u_translation", view.pos.as_slice());

		window.draw_array(&self.main_array);

		window.swap();
		if self.show_debug_info {
			println!("frame time = {:?}ms", frame_dt * 1000.0);
		}

		true
	}
}

fn try_main() -> Result<(), String> {
	let mut state = State::new()?;
	while state.frame() {}

	Ok(())
}

fn main() {
	if let Err(e) = try_main() {
		win::display_error_message(&e);
	}
}
