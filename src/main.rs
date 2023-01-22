/*
@TODO:
- screenshot
- RnToRn functions (& add back in RToR)
 -  also add PerComponent(Box<RToR>,Box<RToR>,Box<RToR>) in R3ToR3
- ProjectX, ProjectY, ProjectZ in R3ToR?
- let user go back&forth through past sdfs using scenes.txt file
- documentation
- GenRandom integers (just use 0..u32::MAX and add a modulus)
- blender-style rendering the picture in multiple frames
   (this lets us look at super complicated SDFs while still processing input, etc. at 60 fps)
- better SDL api: Context  +  Window<'a> impl !Send+!Sync
- gallery view
- record a cool video
*/

#![windows_subsystem = "windows"]
extern crate nalgebra;

pub mod sdf;
mod sdl;
pub mod win;

use nalgebra::{Matrix3, Matrix4, Rotation3, Vector3};
use sdf::ImportExport;
use std::{
	collections::HashMap,
	fs::File,
	io::{prelude::*, BufReader},
	time::Instant,
};
use win::{ColorF32, ColorGrayscaleF32, ColorU8};

type Vec3 = Vector3<f32>;
type Mat3 = Matrix3<f32>;
type Mat4 = Matrix4<f32>;
type Rot3 = Rotation3<f32>;

#[repr(i32)]
#[derive(Clone, Copy)]
enum Icon {
	None = 0,
	Copy = 1,
	Play = 2,
	Pause = 3,
	Rewind = 4,
}

#[derive(Clone)]
struct View {
	pos: Vec3,
	rotation: Mat3,
	time: f64,
	time_speed: f64,
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
			time: 0.0,
			time_speed: 0.0,
			level_set: 0.0,
		}
	}
}

impl View {
	/// `rotation() * vec3(0, 0, -1)` is the direction the camera is pointing
	fn rotation(&self) -> Mat3 {
		self.rotation
	}

	fn pause(&mut self) {
		self.time_speed = 0.0;
	}

	fn paused(&self) -> bool {
		self.time_speed == 0.0
	}

	fn unpause(&mut self, rewind: bool) {
		self.time_speed = if rewind { -1.0 } else { 1.0 };
	}

	fn pass_time(&mut self, dt: f64) {
		self.time += self.time_speed * dt;
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
	post: win::Program,
}

impl Programs {
	fn new(window: &mut win::Window) -> Self {
		Programs {
			main: window.new_program(),
			test: window.new_program(),
			post: window.new_program(),
		}
	}

	fn load_scene(&mut self, window: &mut win::Window, scene: &sdf::Scene) -> Result<(), String> {
		let vsource_main = include_str!("vshader_main.glsl");
		let fsource_main = include_str!("fshader_main.glsl");
		let vsource_test = include_str!("vshader_test.glsl");
		let fsource_test = include_str!("fshader_test.glsl");
		let source_common = include_str!("fshader_common.glsl");
		let vsource_post = include_str!("vshader_post.glsl");
		let fsource_post = include_str!("fshader_post.glsl");

		let mut sdf = String::new();
		let mut get_color = String::new();
		scene.sdf.to_glsl_function("sdf", &mut sdf);
		scene
			.color_function
			.to_glsl_function("get_color_", &mut get_color);
		let fsource_main = fsource_main
			.replace("%SDF%", &sdf)
			.replace("%COLOR%", &get_color)
			.replace("%COMMON%", source_common);
		let fsource_test = fsource_test
			.replace("%SDF%", &sdf)
			.replace("%COMMON%", source_common);

		window
			.link_program(&mut self.main, vsource_main, &fsource_main)
			.map_err(|e| format!("Error compiling shader:\n{e}"))?;
		window
			.link_program(&mut self.test, vsource_test, &fsource_test)
			.map_err(|e| format!("Error compiling shader:\n{e}"))?;
		window
			.link_program(&mut self.post, vsource_post, &fsource_post)
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

#[derive(Default)]
struct Settings {
	data: HashMap<String, f64>,
}

impl Settings {
	fn load(filename: &str) -> Result<Self, String> {
		let file = File::open(filename).map_err(|e| format!("{e}"))?;
		let reader = BufReader::new(file);
		let mut data = HashMap::new();
		for line in reader.lines() {
			let full_line = line.map_err(|e| format!("{e}"))?;
			let line = full_line.trim();
			if line.starts_with('#') {
				// comment
			} else {
				let parts: Vec<&str> = line.split(' ').collect();
				if parts.len() != 2 {
					return Err(format!("bad line: {line}"));
				}
				let key = parts[0].trim();
				let value = parts[1].trim();
				let value: f64 = value.parse().map_err(|_| format!("bad number: {value}"))?;
				data.insert(key.to_string(), value);
			}
		}
		Ok(Self { data })
	}

	fn get_f64(&self, key: &str) -> Option<f64> {
		self.data.get(key).copied()
	}

	fn get_f32(&self, key: &str) -> Option<f32> {
		self.get_f64(key).map(|x| x as f32)
	}

	fn get_usize(&self, key: &str) -> Option<usize> {
		self.get_f64(key).map(|x| x as usize)
	}

	fn get_i32(&self, key: &str) -> Option<i32> {
		self.get_f64(key).map(|x| x as i32)
	}
}

struct State {
	window: win::Window,
	view: View,
	settings: Settings,
	initial_view: View,
	show_debug_info: bool,
	fullscreen: bool,
	frame_time: Instant,
	programs: Programs,
	config: sdf::SceneConfig,
	scene: sdf::Scene,
	// can be none if opening failed for whatever reason
	scene_list: Option<File>,
	test_framebuffer_texture: win::Texture,
	test_framebuffer: win::Framebuffer,
	main_framebuffer_texture: win::Texture,
	main_framebuffer: win::Framebuffer,
	main_framebuffer_size: (i32, i32),
	main_array: win::VertexArray,
	test_array: win::VertexArray,
	post_array: win::VertexArray,
	// displayed on top of the screen. used for feedback when copying/pasting/etc
	flash: ColorF32,
	flash_icon: Icon,
}

impl State {
	fn new(settings: Settings) -> Result<Self, String> {
		let mut window = win::Window::new("AutoSDF", 1280, 720, &Default::default())
			.map_err(|e| format!("Error creating window: {e}"))?;
		window.set_icon("icon.bmp");
		let mut programs = Programs::new(&mut window);
		let config = sdf::SceneConfig {
			sdf_length: settings.get_usize("sdf-length").unwrap_or(500),
			color_length: settings.get_usize("color-length").unwrap_or(300),
		};
		let scene = sdf::Scene::good_random(&mut get_rng(), &config);
		programs
			.load_scene(&mut window, &scene)
			.unwrap_or_else(|e| eprintln!("Error: {e}"));

		let mut test_framebuffer_texture = window.create_texture(&Default::default());
		// we don't really care if there's an error. not much bad will happen.
		let _ = test_framebuffer_texture.set_data::<ColorGrayscaleF32>(
			None,
			TEST_WIDTH.into(),
			TEST_HEIGHT.into(),
		);

		let mut test_framebuffer = window.create_framebuffer();
		test_framebuffer.set_texture(
			win::FramebufferAttachment::Color0,
			&test_framebuffer_texture,
		);
		
		
		let main_texconfig = win::TextureParams {
			mag_filter: win::TextureFilter::Nearest,
			..Default::default()
		};
		let main_framebuffer_texture = window.create_texture(&main_texconfig);
		let main_framebuffer = window.create_framebuffer();
		
		let mut main_buffer = window.create_buffer();
		let mut test_buffer = window.create_buffer();
		let mut post_buffer = window.create_buffer();
		let data: &[[f32; 2]] = &[
			[-1.0, -1.0],
			[1.0, -1.0],
			[1.0, 1.0],
			[-1.0, -1.0],
			[1.0, 1.0],
			[-1.0, 1.0],
		];
		main_buffer.set_data(data);
		test_buffer.set_data(data);
		post_buffer.set_data(data);
		let main_array = window.create_vertex_array(main_buffer, &programs.main);
		let test_array = window.create_vertex_array(test_buffer, &programs.test);
		let post_array = window.create_vertex_array(post_buffer, &programs.post);

		window.set_mouse_relative(true);

		let scene_list = File::options()
			.append(true)
			.create(true)
			.open("scenes.txt")
			.ok();

		let mut me = Self {
			window,
			programs,
			view: View::default(),
			initial_view: View::default(),
			config,
			frame_time: Instant::now(),
			show_debug_info: false,
			fullscreen: false,
			scene: sdf::Scene::default(),
			test_framebuffer_texture,
			test_framebuffer,
			main_framebuffer_texture,
			main_framebuffer,
			main_framebuffer_size: (0, 0),
			main_array,
			test_array,
			post_array,
			scene_list,
			settings,
			flash: ColorF32::rgba(0.0, 0.0, 0.0, 0.0),
			flash_icon: Icon::None,
		};
		me.load_scene(scene);
		Ok(me)
	}

	fn load_scene(&mut self, scene: sdf::Scene) {
		match self.programs.load_scene(&mut self.window, &scene) {
			Ok(()) => {
				self.scene = scene;
				if let Some(list) = &mut self.scene_list {
					let mut string = self.scene.export_string();
					string.push('\n');
					// i dont really care if this fails, and it probably won't
					let _ = list.write_all(string.as_bytes());
				}

				// *technically speaking* the location of v_pos could change between reloads
				self.main_array.attrib2f("v_pos", 0);
				self.test_array.attrib2f("v_pos", 0);
				self.post_array.attrib2f("v_pos", 0);

				// percentage of space occupied by object
				let frac = 0.25;
				// determine default level set
				// specifically we want to select y such that
				//   for ~frac of p values, sdf(p) < y
				self.window.bind_framebuffer(Some(&self.test_framebuffer));
				self.window
					.viewport(0, 0, TEST_WIDTH.into(), TEST_HEIGHT.into());

				self.window.use_program(&self.programs.test);
				self.test_array.draw();

				self.window.bind_framebuffer(None);

				let mut sdf_values: Vec<f32> = self
					.window
					.get_texture_data_vec::<ColorGrayscaleF32>(&self.test_framebuffer_texture)
					.iter()
					.map(|c| c.value)
					.collect();
				let i = (sdf_values.len() as f64 * frac) as usize;
				let level_set = *sdf_values
					.select_nth_unstable_by(i, |a, b| a.total_cmp(b))
					.1;
				drop(sdf_values);
				let initial_view = View {
					level_set,
					..Default::default()
				};
				self.initial_view = initial_view.clone();
				self.view = initial_view;
			}
			Err(e) => {
				eprintln!("Error: {e}")
			}
		};
	}

	fn flash(&mut self, icon: Icon) {
		self.flash = match icon {
			Icon::None => ColorF32::BLACK,
			Icon::Copy => ColorF32::GREEN,
			_ => ColorF32::rgb(1.0, 0.5, 0.0),
		};
		self.flash_icon = icon;
	}
	
	fn render_resolution(&self) -> (i32, i32) {
		let scale = self.settings.get_f32("scale").unwrap_or(1.0);
		if scale <= 0.0 || scale > 1.0 {
			win::display_error_message(&format!("bad scale: {scale}"));
			std::process::exit(1);
		}
		let (w, h) = self.window.size();
		((w as f32 * scale) as i32, (h as f32 * scale) as i32)
	}

	// returns false if we should quit
	fn frame(&mut self) -> bool {
		if let Some(max_framerate) = self.settings.get_f32("max-framerate") {
			if max_framerate > 0.0 {
				let dt = self.frame_time.elapsed().as_secs_f32();
				let sleep_millis = 1000.0 * (1.0 / max_framerate - dt);
				if sleep_millis >= 1.0 {
					std::thread::sleep(std::time::Duration::from_millis(sleep_millis as u64));
				}
			}
		}
		let frame_dt = self.frame_time.elapsed().as_secs_f32();
		self.frame_time = Instant::now();

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
					self.flash(Icon::Copy);
				}
				KeyDown { key: F, .. } => {
					self.fullscreen = !self.fullscreen;
					self.window.set_fullscreen(self.fullscreen);
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
				KeyDown {
					key: Space,
					modifier,
				} => {
					if !self.view.paused() {
						self.view.pause();
						self.flash(Icon::Pause);
					} else if modifier.shift() {
						self.view.unpause(true);
						self.flash(Icon::Play);
					} else {
						self.view.unpause(false);
						self.flash(Icon::Rewind);
					}
				}
				MouseMotion { xrel, yrel, .. } => {
					let mouse_sensitivity =
						0.001 * self.settings.get_f32("mouse-sensitivity").unwrap_or(50.0);
					self.view
						.yaw_by(-xrel as f32 * mouse_sensitivity * frame_dt);
					self.view
						.pitch_by(-yrel as f32 * mouse_sensitivity * frame_dt);
				}
				_ => {}
			}
		}

		self.view.pass_time(frame_dt.into());

		{
			// movement
			let mut dx = 0.0;
			let mut dy = 0.0;
			let mut dz = 0.0;
			let mut dl = 0.0;
			let mut dt = 0.0;

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
			if window.any_key_down(&[PageUp, NumPad9, Q]) {
				dy += 1.0;
			}
			if window.any_key_down(&[PageDown, NumPad3, E]) {
				dy -= 1.0;
			}
			if window.any_key_down(&[Equals]) {
				dl += 1.0;
			}
			if window.any_key_down(&[Minus]) {
				dl -= 1.0;
			}
			if window.any_key_down(&[LeftBracket]) {
				dt -= 1.0;
			}
			if window.any_key_down(&[RightBracket]) {
				dt += 1.0;
			}
			let mut speed_multiplier = if window.is_shift_down() { 10.0 } else { 1.0 };
			speed_multiplier *= if window.is_ctrl_down() { 0.1 } else { 1.0 };
			speed_multiplier *= frame_dt;

			if dt != 0.0 {
				let dt = dt * speed_multiplier;
				self.view.pause();
				self.view.time += f64::from(dt);
			}

			let motion = Vec3::new(dx, dy, dz);
			if let Some(motion) = motion.try_normalize(0.001) {
				let move_amount = 4.0 * speed_multiplier;
				let motion = motion * move_amount;
				let motion = self.view.rotation() * motion;
				self.view.pos += motion;
			}

			let level_set_amount = 1.0 * speed_multiplier;
			self.view.level_set += dl * level_set_amount;
		}

		let render_resolution = self.render_resolution();
		if render_resolution != self.main_framebuffer_size {
			let result = self.main_framebuffer_texture.set_data::<ColorU8>(None, render_resolution.0 as usize, render_resolution.1 as usize);
			
			match result {
				Ok(()) => {},
				Err(e) => eprintln!("warning:{e}"),
			}
			
			self.main_framebuffer.set_texture(
				win::FramebufferAttachment::Color0,
				&self.main_framebuffer_texture,
			);
			self.main_framebuffer_size = render_resolution;
		}
		
		let window = &mut self.window;
		let view = &self.view;
		window.viewport(0, 0, render_resolution.0, render_resolution.1);

		window.clear_screen(win::ColorF32::BLACK);
		window.use_program(&self.programs.main);
		window.bind_framebuffer(Some(&self.main_framebuffer));
		window.uniform1f("u_aspect_ratio", render_resolution.0 as f32 / render_resolution.1 as f32);
		{
			let (w, h) = window.size();
			window.uniform2f("u_screen_size", w as f32, h as f32);
		}
		window.uniform1f("u_time", view.time as f32);
		window.uniform1f(
			"u_fov",
			self.settings.get_f32("fov").unwrap_or(45.0).to_radians(),
		);
		window.uniform1f(
			"u_focal_length",
			self.settings.get_f32("focal-length").unwrap_or(1.0),
		);
		window.uniform1f("u_level_set", view.level_set);
		window.uniform1i("u_hsv", self.settings.get_i32("hsv").unwrap_or(0));
		let antialiasing = self.settings.get_i32("antialiasing").unwrap_or(1);
		window.uniform2i("u_antialiasing", antialiasing, antialiasing);
		window.uniform1i(
			"u_iterations",
			self.settings.get_i32("max-iterations").unwrap_or(30),
		);
		window.uniform1f(
			"u_distance_threshold",
			self.settings.get_f32("distance-threshold").unwrap_or(0.02),
		);
		window.uniform3x3f("u_rotation", view.rotation().as_slice());
		window.uniform3f_slice("u_translation", view.pos.as_slice());
		window.uniform4f_color("u_flash", self.flash);
		window.uniform1i("u_flash_icon", self.flash_icon as i32);

		self.flash.a = f32::max(self.flash.a - frame_dt * (2.0 - 1.0 * self.flash.a), 0.0);
		if self.flash.a <= 0.0 {
			// icon is no longer visible
			self.flash_icon = Icon::None;
		}

		self.main_array.draw();
		
		window.bind_framebuffer(None);
		window.viewport_full_screen();
		window.use_program(&self.programs.post);
		window.active_texture(0, &self.main_framebuffer_texture);
		window.uniform_texture("u_texture", 0);
		self.post_array.draw();

		window.swap();
		
		if self.show_debug_info {
			println!("frame time = {:?}ms", frame_dt * 1000.0);
		}

		true
	}
}

fn try_main() -> Result<(), String> {
	let settings =
		Settings::load("settings.txt").map_err(|e| format!("Error loading settings.txt: {e}"))?;
	let mut state = State::new(settings)?;
	while state.frame() {}

	Ok(())
}

fn main() {
	if let Err(e) = try_main() {
		win::display_error_message(&e);
	}
}
