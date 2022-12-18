/*
@TODO:
- publish git repo
- replace *mut SDL_Window with Window
- options for:
	- max framerate
- come up with twisty lipschitz continuous function, & add it
- feedback for copy/paste (flash screen or something)
- feedback for pause/unpause/rewind (flash icons)
- Params instead of depth for GenRandom
   - allow multiple endpoints (cube & sphere & ...)
- let user go back&forth through past sdfs using scenes.txt file
- mathematical analysis
- documentation
- GenRandom integers (+ gen_random_scale_bias)
- record a video
- better SDL api: Context  +  Window<'a> impl !Send+!Sync
- gallery view
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
a263736466a169536d6f6f74684d696e82a1634d697883a167436f6d706f736583a16641726374616ea16454696d6582fa3da97b73fa3eddf3cca167436f6d706f736583a16f496e66696e6974654d6972726f7273a16454696d6582fabb6c5400fa3c230980a167436f6d706f736583675369676d6f6964a167436f6d706f736583a167436f6d706f736582684964656e74697479684964656e74697479a16443756265a163463332fa3edf6864684964656e74697479684964656e74697479684964656e74697479684964656e74697479a166537068657265a163463332fa3ee32af4a16454696d6582fa3db4b603fa3ea79394a1634d697883a167436f6d706f736583a16353696ea163463332fa3de4e160a167436f6d706f736583a16f496e66696e6974654d6972726f7273a163463332fa3f7493fea168426f784672616d65a26473697a65a16454696d6582fa3e92c08dfa3ee156b869746869636b6e657373a16454696d6582fa3c74f202fa3df09e1d684964656e74697479684964656e74697479a1634d697883a16443756265a163463332fa3f5f8194a1634d696e82a1634d697883a167436f6d706f736583675369676d6f6964a168426f784672616d65a26473697a65a163463332fa3fe3873769746869636b6e657373a163463332fa3e3524e7684964656e74697479a1634d696e82a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3f50ffb8684964656e74697479a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3c693200684964656e74697479a163463332fa3e5d3250a168426f784672616d65a26473697a65a163463332fa3fb01f6669746869636b6e657373a16454696d6582fabc5d9d48fa3d1cc993a163463332fa3f394d40a16454696d6582fabda7629afa3eefd35c6e636f6c6f725f66756e6374696f6ea16f496e66696e6974654d6972726f7273a163463332fa3d8dc730
a263736466a1634d697883a167436f6d706f736583a1695472616e736c61746583a16454696d6582fa3c1c5ee8fa3f1f076ea16454696d6582fabc0cdbe8fa3ea36374a16454696d6582fa3cdd78a4fa3ef7d550a167436f6d706f736583a16641726374616ea16454696d6582fabbdfeb60fa3dc83a60a167436f6d706f736583a16f496e66696e6974654d6972726f7273a16454696d6582fabd054634fa3e4c5448a167436f6d706f736583a16f496e66696e6974654d6972726f7273a163463332fa3f0b5ffaa168426f784672616d65a26473697a65a163463332fa3f25fee269746869636b6e657373a16454696d6582fabc18bba9fa3e319347684964656e74697479684964656e74697479684964656e74697479684964656e74697479a1634d697883a168426f784672616d65a26473697a65a163463332fa40190a4e69746869636b6e657373a16454696d6582fa3b46bd33fa3cec529aa167436f6d706f736583a1695472616e736c61746583a16454696d6582fa3c91b74cfa3d0ece20a16454696d6582fa3da30eedfa3f261c72a163463332fa3f295ec6a167436f6d706f736583a167436f6d706f736582a16353696ea163463332fa3d069a00a16f496e66696e6974654d6972726f7273a16454696d6582fa3dae4de1fa3d837a30a1634d697883a167436f6d706f736583a16353696ea163463332fa3f7730b6a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3e01c7a8684964656e74697479684964656e74697479a16443756265a16454696d6582fabcfb0758fa3e7763f0a16454696d6582fabd650fcdfa3f0d466e684964656e74697479684964656e74697479a16454696d6582fabd78a3fafa3dd86e20a16454696d6582fabd6ccb9afa3f1f06e46e636f6c6f725f66756e6374696f6ea167436f6d706f736582a167436f6d706f736582a16641726374616ea163463332fa3f67a3a2a16641726374616ea163463332fa3f287b4e675369676d6f6964
a263736466a1634d697883a169536d6f6f74684d696e82a167436f6d706f736583a166526f7461746583a16454696d6582fa3ce93344fa3f037de0a16454696d6582fabcf59274fa3f605e9ca16454696d6582fabd5d556dfa3f13891ca167436f6d706f736583a167436f6d706f736582a167436f6d706f736582a16641726374616ea163463332fa3da39b50a167436f6d706f736582684964656e74697479684964656e74697479a167436f6d706f736582a16353696ea163463332fa3e785a08a167436f6d706f736582684964656e74697479684964656e74697479a167436f6d706f736583a166526f7461746583a163463332fa3ec36808a163463332fa3e1fdf58a163463332fa3e69ed18a165546f727573a266726164697573a163463332fa3fb2f5de69746869636b6e657373a163463332fa3e22de5d684964656e74697479684964656e74697479684964656e74697479a167436f6d706f736583a16353696ea163463332fa3ea5ea70a168426f784672616d65a26473697a65a16454696d6582fabe10ed3efa3eb1cfd869746869636b6e657373a163463332fa3e0dea57684964656e74697479a168426f784672616d65a26473697a65a16454696d6582fabd741592fa401a9ae469746869636b6e657373a16454696d6582fa3c677072fa3e3bc95da16454696d6582fa3b24dd40fa3e816f846e636f6c6f725f66756e6374696f6ea165537153696ea16454696d6582fabc7f2b18fa3f6e73c2
a263736466a167436f6d706f736583a1695472616e736c61746583a163463332fa3f4811e4a163463332fa3f78c51ea16454696d6582fa3d90de6dfa3f596b98a1634d697883a167436f6d706f736583a167436f6d706f736582684964656e74697479684964656e74697479a167436f6d706f736583684964656e74697479a166537068657265a163463332fa3eef3a44684964656e74697479684964656e74697479a167436f6d706f736583a16f496e66696e6974654d6972726f7273a163463332fa3f6ad464a165546f727573a266726164697573a163463332fa3fcd633269746869636b6e657373a163463332fa3d887747684964656e74697479a16454696d6582fabcad7e74fa3f444f5c684964656e746974796e636f6c6f725f66756e6374696f6ea167436f6d706f736582a167436f6d706f736582a167436f6d706f736582a166526f7461746583a163463332fa3f139110a163463332fa3e8c6384a163463332fa3edc7998675369676d6f6964a16353696ea16454696d6582fa3ce10f8cfa3e42f2c8a167436f6d706f736582a16f496e66696e6974654d6972726f7273a163463332fa3f4e41c8a167436f6d706f736582a16641726374616ea16454696d6582fa3dc7acedfa3f76784ca16353696ea163463332fa3f62c45a
a263736466a169536d6f6f74684d696e82a167436f6d706f736583a167436f6d706f736582a16353696ea163463332fa3f0cf78ea166526f7461746583a163463332fa3f1a98baa163463332fa3f51c216a163463332fa3f70188aa16443756265a16454696d6582fa3d12c20efa3f7aa514684964656e74697479a167436f6d706f736583a166526f7461746583a163463332fa3e7c36f8a163463332fa3e13dac8a163463332fa3f5f7a24a167436f6d706f736583a16353696ea163463332fa3f2b95bca1634d697883a166537068657265a163463332fa3f50ea86a166537068657265a163463332fa3f6af350a163463332fa3f4a1b6a684964656e74697479684964656e746974796e636f6c6f725f66756e6374696f6ea167436f6d706f736582a167436f6d706f736582a166526f7461746583a16454696d6582fa3cafe358fa3f27e6e6a163463332fa3d5c6fc0a163463332fa3eef1c50a16353696ea163463332fa3f7f60f8a167436f6d706f736582a166526f7461746583a163463332fa3f594124a163463332fa3f176438a16454696d6582fa3a355d00fa3e35c550a166526f7461746583a16454696d6582fa3d94d98dfa3f64c122a163463332fa3cdbef00a163463332fa3f39d206
*/

//  LICENSE: i'm not gonna sue you for "copyright infringement". go wild.

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
use win::ColorGrayscaleF32;

type Vec3 = Vector3<f32>;
type Mat3 = Matrix3<f32>;
type Mat4 = Matrix4<f32>;
type Rot3 = Rotation3<f32>;

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
	framebuffer_texture: win::Texture,
	framebuffer: win::Framebuffer,
	main_array: win::VertexArray,
	test_array: win::VertexArray,
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

		let mut framebuffer_texture = window.create_texture(&Default::default());
		// we don't really care if there's an error. not much bad will happen.
		let _ = window.set_texture_no_data::<ColorGrayscaleF32>(
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
			framebuffer_texture,
			framebuffer,
			main_array,
			test_array,
			scene_list,
			settings,
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
					.get_texture_data_vec::<ColorGrayscaleF32>(&self.framebuffer_texture)
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

	// returns false if we should quit
	fn frame(&mut self) -> bool {
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
					} else if modifier.shift() {
						self.view.unpause(true);
					} else {
						self.view.unpause(false);
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

		let window = &mut self.window;
		let view = &self.view;
		window.viewport_full_screen();

		window.clear_screen(win::ColorF32::BLACK);
		window.use_program(&self.programs.main);
		window.uniform1f("u_aspect_ratio", window.aspect_ratio());
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

		window.draw_array(&self.main_array);

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
