// all OpenGL calls are done through the Window.
// this is because OpenGL is not thread safe.
use crate::sdl;
use gl::types::{GLchar, GLenum, GLint, GLsizei, GLuint};
use mem::size_of;
#[allow(unused_imports)]
use std::ffi::{c_char, c_int, c_uint, c_void, CStr, CString};
use std::sync::Mutex;
use std::{fmt, mem};

pub type AudioCallback = fn(sample_rate: u32, samples: &mut [f32]);

struct AudioData {
	callback: AudioCallback,
	device: sdl::SDL_AudioDeviceID,
	sample_rate: u32,
}

pub struct Window {
	sdlwin: *mut sdl::SDL_Window,
	glctx: sdl::SDL_GLContext,
	used_program: GLuint,
	audio_data: Option<Box<AudioData>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
	N0,
	N1,
	N2,
	N3,
	N4,
	N5,
	N6,
	N7,
	N8,
	N9,
	NumPad0,
	NumPad1,
	NumPad2,
	NumPad3,
	NumPad4,
	NumPad5,
	NumPad6,
	NumPad7,
	NumPad8,
	NumPad9,
	Up,
	Left,
	Right,
	Down,
	PageUp,
	PageDown,
	Space,
	Enter,
	Escape,
	LShift,
	RShift,
	LCtrl,
	RCtrl,
	LAlt,
	RAlt,
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
}

impl Key {
	fn from_sdl(scancode: sdl::SDL_Scancode) -> Option<Self> {
		use sdl::scancode::*;
		Some(match scancode {
			A => Key::A,
			B => Key::B,
			C => Key::C,
			D => Key::D,
			E => Key::E,
			F => Key::F,
			G => Key::G,
			H => Key::H,
			I => Key::I,
			J => Key::J,
			K => Key::K,
			L => Key::L,
			M => Key::M,
			N => Key::N,
			O => Key::O,
			P => Key::P,
			Q => Key::Q,
			R => Key::R,
			S => Key::S,
			T => Key::T,
			U => Key::U,
			V => Key::V,
			W => Key::W,
			X => Key::X,
			Y => Key::Y,
			Z => Key::Z,
			RETURN => Key::Enter,
			SPACE => Key::Space,
			N0 => Key::N0,
			N1 => Key::N1,
			N2 => Key::N2,
			N3 => Key::N3,
			N4 => Key::N4,
			N5 => Key::N5,
			N6 => Key::N6,
			N7 => Key::N7,
			N8 => Key::N8,
			N9 => Key::N9,
			UP => Key::Up,
			LEFT => Key::Left,
			RIGHT => Key::Right,
			DOWN => Key::Down,
			ESCAPE => Key::Escape,
			PAGEUP => Key::PageUp,
			PAGEDOWN => Key::PageDown,
			F1 => Key::F1,
			F2 => Key::F2,
			F3 => Key::F3,
			F4 => Key::F4,
			F5 => Key::F5,
			F6 => Key::F6,
			F7 => Key::F7,
			F8 => Key::F8,
			F9 => Key::F9,
			F10 => Key::F10,
			F11 => Key::F11,
			F12 => Key::F12,
			KP_0 => Key::NumPad0,
			KP_1 => Key::NumPad1,
			KP_2 => Key::NumPad2,
			KP_3 => Key::NumPad3,
			KP_4 => Key::NumPad4,
			KP_5 => Key::NumPad5,
			KP_6 => Key::NumPad6,
			KP_7 => Key::NumPad7,
			KP_8 => Key::NumPad8,
			KP_9 => Key::NumPad9,
			LSHIFT => Key::LShift,
			RSHIFT => Key::RShift,
			LCTRL => Key::LCtrl,
			RCTRL => Key::RCtrl,
			LALT => Key::LAlt,
			RALT => Key::RAlt,
			_ => return None,
		})
	}

	fn to_sdl(self) -> sdl::SDL_Scancode {
		use sdl::scancode::*;
		match self {
			Key::A => A,
			Key::B => B,
			Key::C => C,
			Key::D => D,
			Key::E => E,
			Key::F => F,
			Key::G => G,
			Key::H => H,
			Key::I => I,
			Key::J => J,
			Key::K => K,
			Key::L => L,
			Key::M => M,
			Key::N => N,
			Key::O => O,
			Key::P => P,
			Key::Q => Q,
			Key::R => R,
			Key::S => S,
			Key::T => T,
			Key::U => U,
			Key::V => V,
			Key::W => W,
			Key::X => X,
			Key::Y => Y,
			Key::Z => Z,
			Key::Enter => RETURN,
			Key::Space => SPACE,
			Key::N0 => N0,
			Key::N1 => N1,
			Key::N2 => N2,
			Key::N3 => N3,
			Key::N4 => N4,
			Key::N5 => N5,
			Key::N6 => N6,
			Key::N7 => N7,
			Key::N8 => N8,
			Key::N9 => N9,
			Key::Up => UP,
			Key::Left => LEFT,
			Key::Right => RIGHT,
			Key::Down => DOWN,
			Key::Escape => ESCAPE,
			Key::PageUp => PAGEUP,
			Key::PageDown => PAGEDOWN,
			Key::F1 => F1,
			Key::F2 => F2,
			Key::F3 => F3,
			Key::F4 => F4,
			Key::F5 => F5,
			Key::F6 => F6,
			Key::F7 => F7,
			Key::F8 => F8,
			Key::F9 => F9,
			Key::F10 => F10,
			Key::F11 => F11,
			Key::F12 => F12,
			Key::NumPad0 => KP_0,
			Key::NumPad1 => KP_1,
			Key::NumPad2 => KP_2,
			Key::NumPad3 => KP_3,
			Key::NumPad4 => KP_4,
			Key::NumPad5 => KP_5,
			Key::NumPad6 => KP_6,
			Key::NumPad7 => KP_7,
			Key::NumPad8 => KP_8,
			Key::NumPad9 => KP_9,
			Key::LShift => LSHIFT,
			Key::RShift => RSHIFT,
			Key::LCtrl => LCTRL,
			Key::RCtrl => RCTRL,
			Key::LAlt => LALT,
			Key::RAlt => RALT,
		}
	}
}

#[derive(Debug)]
pub enum Event {
	Quit,
	KeyDown(Key),
	KeyUp(Key),
	MouseMotion {
		x: i32,
		y: i32,
		xrel: i32,
		yrel: i32,
	},
}

pub fn display_error_message(message: &str) {
	let result = unsafe {
		sdl::show_simple_message_box(sdl::SDL_MESSAGEBOX_ERROR, "Error", message, 0 as _)
	};
	if result.is_err() {
		eprintln!("{}", message);
	}
}

#[derive(Clone, Copy)]
pub struct ColorF32 {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

#[derive(Clone, Copy)]
pub struct ColorU8 {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl fmt::Display for ColorU8 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"#{:02x}{:02x}{:02x}{:02x}",
			self.r, self.g, self.b, self.a
		)
	}
}

impl fmt::Debug for ColorU8 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self)
	}
}

impl ColorU8 {
	pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
		ColorU8 { r, g, b, a }
	}

	pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
		ColorU8 { r, g, b, a: 255 }
	}

	pub fn tint(&mut self, other: ColorU8) {
		self.r = ((self.r as u32 * other.r as u32) >> 8) as u8;
		self.g = ((self.g as u32 * other.g as u32) >> 8) as u8;
		self.b = ((self.b as u32 * other.b as u32) >> 8) as u8;
		self.a = ((self.a as u32 * other.a as u32) >> 8) as u8;
	}

	pub fn from_bytes(bytes: &[u8]) -> Self {
		assert!(bytes.len() >= 4);
		Self {
			r: bytes[0],
			g: bytes[1],
			b: bytes[2],
			a: bytes[3],
		}
	}

	pub fn as_bytes(self, bytes: &mut [u8]) {
		assert!(bytes.len() >= 4);
		bytes[0] = self.r;
		bytes[1] = self.g;
		bytes[2] = self.b;
		bytes[3] = self.a;
	}

	pub fn as_tuple(self) -> (u8, u8, u8, u8) {
		(self.r, self.g, self.b, self.a)
	}
}

impl From<u32> for ColorU8 {
	fn from(color: u32) -> ColorU8 {
		ColorU8::new(
			(color >> 24) as u8,
			(color >> 16) as u8,
			(color >> 8) as u8,
			color as u8,
		)
	}
}

impl ColorF32 {
	pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);

	pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
		ColorF32 { r, g, b, a: 1.0 }
	}

	pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
		ColorF32 { r, g, b, a }
	}
}

pub struct Shader {
	id: GLuint,
}

impl Shader {
	fn new(r#type: GLenum, source: &str) -> Result<Self, String> {
		let id = unsafe { gl::CreateShader(r#type) };
		let result = Self::new_with_id(id, r#type, source);
		if result.is_err() {
			unsafe { gl::DeleteShader(id) };
		}
		result
	}

	fn new_with_id(id: GLuint, r#type: GLenum, source: &str) -> Result<Self, String> {
		if id == 0 {
			return Err(format!("couldn't create shader (GL error {})", unsafe {
				gl::GetError()
			}));
		}

		{
			//set source
			// @TODO(eventually): support for older versions of GLSL
			let header = if r#type == gl::FRAGMENT_SHADER {
				"#version 130
#define IN in
#define OUT out
#define gl_FragColor o_color
out vec4 o_color;
#line 1
"
			} else {
				"#version 130
#define IN in
#define OUT out
#define ATTRIBUTE in
#line 1
"
			};
			let hdrptr = header.as_bytes().as_ptr() as *const GLchar;
			let srcptr = source.as_bytes().as_ptr() as *const GLchar;
			let sources = [hdrptr, srcptr];
			let lengths = [header.len() as GLint, source.len() as GLint];

			let sources_ptr = &sources[0] as *const *const GLchar;
			let lengths_ptr = &lengths[0] as *const GLint;

			unsafe { gl::ShaderSource(id, sources.len() as _, sources_ptr, lengths_ptr) };
		}

		unsafe { gl::CompileShader(id) };
		{
			//check log
			let mut log = [0u8; 1024];
			let mut len: GLsizei = 0;
			let logp = &mut log as *mut u8 as *mut GLchar;
			let lenp = &mut len as *mut GLsizei;
			unsafe { gl::GetShaderInfoLog(id, log.len() as GLsizei, lenp, logp) };
			if len > 0 {
				eprintln!("{}", String::from_utf8_lossy(&log[..len as usize]));
			}
		}
		{
			let mut status: GLint = 0;
			unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, (&mut status) as _) };
			if status == 0 {
				return Err("failed to compile".to_string());
			}
		}

		Ok(Self { id })
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe { gl::DeleteShader(self.id) };
	}
}

pub struct Program {
	id: GLuint,
}

impl Program {
	fn new(shaders: &[Shader]) -> Result<Self, String> {
		let id = unsafe { gl::CreateProgram() };
		let result = Self::new_with_id(id, shaders);
		if result.is_err() {
			unsafe { gl::DeleteShader(id) };
		}
		result
	}

	fn new_with_id(id: GLuint, shaders: &[Shader]) -> Result<Self, String> {
		for shader in shaders {
			unsafe { gl::AttachShader(id, shader.id) };
		}
		unsafe { gl::LinkProgram(id) };
		{
			// check log
			let mut log = [0u8; 1024];
			let mut len: GLsizei = 0;
			let logp = &mut log as *mut u8 as *mut GLchar;
			let lenp = &mut len as *mut GLsizei;
			unsafe { gl::GetProgramInfoLog(id, log.len() as GLsizei, lenp, logp) };
			if len > 0 {
				eprintln!("{}", String::from_utf8_lossy(&log[..len as usize]));
			}
		}

		{
			let mut status: GLint = 0;
			unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, (&mut status) as _) };
			if status == 0 {
				return Err("failed to link".to_string());
			}
		}

		Ok(Self { id })
	}
}

impl Drop for Program {
	fn drop(&mut self) {
		unsafe { gl::DeleteProgram(self.id) };
	}
}

pub struct Buffer {
	id: GLuint,
	stride: u32,
	count: u32,
}

impl Buffer {
	fn new() -> Self {
		let mut id = 0;
		unsafe { gl::CreateBuffers(1, &mut id as *mut GLuint) };
		Self {
			id,
			stride: 0,
			count: 0,
		}
	}

	fn bind(&self) {
		unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id) };
	}

	fn set_data<T>(&mut self, data: &[T]) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
		}
		self.count = data.len() as u32;
		self.stride = mem::size_of::<T>() as u32;

		unsafe {
			gl::BufferData(
				gl::ARRAY_BUFFER,
				(self.count * self.stride) as _,
				data.as_ptr() as _,
				gl::STATIC_DRAW,
			);
		}
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		unsafe { gl::DeleteBuffers(1, &self.id as *const GLuint) };
	}
}

pub struct VertexArray {
	buffer: Buffer,
	id: GLuint,
	program: GLuint,
}

impl VertexArray {
	fn new(buffer: Buffer, program: &Program) -> Self {
		let mut id: GLuint = 0;

		unsafe { gl::GenVertexArrays(1, &mut id as *mut GLuint) };

		Self {
			id,
			buffer,
			program: program.id,
		}
	}

	fn bind(&self) {
		unsafe { gl::BindVertexArray(self.id) };
	}

	fn attribnf(&mut self, n: u8, name: &str, offset: usize) -> bool {
		let Ok(cstring) = CString::new(name) else { return false };
		let cstr = cstring.as_ptr() as *const GLchar;
		let loc = unsafe { gl::GetAttribLocation(self.program, cstr) };
		let Ok(loc) = loc.try_into() else { return false };

		if offset + usize::from(n) * size_of::<f32>() > self.buffer.stride as usize {
			// offset too large
			return false;
		}

		self.bind();
		self.buffer.bind();
		unsafe {
			gl::VertexAttribPointer(
				loc,
				n.into(),
				gl::FLOAT,
				0,
				self.buffer.stride as _,
				offset as _,
			)
		};
		unsafe { gl::EnableVertexAttribArray(loc) };
		true
	}

	fn draw(&self) {
		self.bind();
		unsafe { gl::DrawArrays(gl::TRIANGLES, 0, self.buffer.count as i32) };
	}
}

impl Drop for VertexArray {
	fn drop(&mut self) {
		unsafe { gl::DeleteVertexArrays(1, &self.id as *const GLuint) };
	}
}

#[cfg(debug_assertions)]
extern "system" fn gl_message_callback(
	_source: GLenum,
	_type: GLenum,
	_id: c_uint,
	severity: GLenum,
	_length: GLsizei,
	message: *const c_char,
	_user_param: *mut c_void,
) {
	let message = String::from_utf8_lossy(unsafe { CStr::from_ptr(message) }.to_bytes());
	if severity == gl::DEBUG_SEVERITY_NOTIFICATION {
		return;
	}
	println!("Message from opengl: {message}");
}

pub struct Texture {
	id: GLuint,
	params: TextureParams,
}

impl Drop for Texture {
	fn drop(&mut self) {
		unsafe { gl::DeleteTextures(1, (&self.id) as *const GLuint) };
	}
}

#[derive(Copy, Clone)]
pub enum TextureFilter {
	Nearest,
	Linear,
}

impl TextureFilter {
	fn to_gl(self) -> GLint {
		use TextureFilter::*;
		match self {
			Nearest => gl::NEAREST as _,
			Linear => gl::LINEAR as _,
		}
	}
}

#[derive(Clone)]
pub struct TextureParams {
	pub min_filter: TextureFilter,
	pub mag_filter: TextureFilter,
}

impl Default for TextureParams {
	fn default() -> Self {
		Self {
			min_filter: TextureFilter::Nearest,
			mag_filter: TextureFilter::Linear,
		}
	}
}

impl Window {
	pub fn new(title: &str, width: i32, height: i32, shown: bool) -> Result<Self, String> {
		{
			static WINDOW_CREATED: Mutex<bool> = Mutex::new(false);
			let guard = WINDOW_CREATED.lock();
			match guard {
				Err(_) => return Err("couldn't lock mutex.".to_string()),
				Ok(x) if *x => return Err("window already created".to_string()),
				Ok(mut x) => *x = true,
			}
		}

		unsafe {
			sdl::set_hint("SDL_NO_SIGNAL_HANDLERS", "1"); // don't replace Ctrl+C, TERM with quit event
			sdl::init()?;
			#[cfg(debug_assertions)]
			{
				sdl::gl_set_context_version(4, 3);
				sdl::gl_set_attribute(sdl::SDL_GL_CONTEXT_FLAGS, sdl::SDL_GL_CONTEXT_DEBUG_FLAG);
			}
			#[cfg(not(debug_assertions))]
			sdl::gl_set_context_version(3, 0);
		}
		let mut flags = sdl::SDL_WINDOW_OPENGL;
		if !shown {
			flags |= sdl::SDL_WINDOW_HIDDEN;
		}
		let sdlwin = unsafe { sdl::create_window(title, width, height, flags) }?;
		let ctx = unsafe { sdl::gl_create_context(sdlwin) }?;
		gl::load_with(|name| unsafe { sdl::gl_get_proc_address(name) });
		unsafe {
			sdl::gl_set_swap_interval(1);
			let mut flags: GLint = 0;
			gl::GetIntegerv(gl::CONTEXT_FLAGS, (&mut flags) as _);
			#[cfg(debug_assertions)]
			if (flags as GLuint & gl::CONTEXT_FLAG_DEBUG_BIT) != 0 {
				gl::DebugMessageCallback(Some(gl_message_callback), 0 as _);
				gl::DebugMessageControl(
					gl::DONT_CARE,
					gl::DONT_CARE,
					gl::DONT_CARE,
					0,
					0 as _,
					gl::TRUE,
				);
				gl::Enable(gl::DEBUG_OUTPUT);
				gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
			}
		}

		Ok(Window {
			sdlwin,
			glctx: ctx,
			used_program: 0,
			audio_data: None,
		})
	}

	pub fn show(&mut self) {
		unsafe { sdl::show_window(self.sdlwin) };
	}

	pub fn set_mouse_relative(&mut self, relative: bool) {
		unsafe {
			sdl::set_relative_mouse_mode(relative);
		}
	}

	pub fn create_program(
		&mut self,
		source_vshader: &str,
		source_fshader: &str,
	) -> Result<Program, String> {
		let vshader = Shader::new(gl::VERTEX_SHADER, source_vshader)?;
		let fshader = Shader::new(gl::FRAGMENT_SHADER, source_fshader)?;
		Program::new(&[vshader, fshader])
	}

	pub fn create_buffer(&mut self) -> Buffer {
		Buffer::new()
	}

	pub fn set_buffer_data<T>(&mut self, buffer: &mut Buffer, data: &[T]) {
		buffer.set_data(data);
	}

	pub fn create_vertex_array(&mut self, buffer: Buffer, program: &Program) -> VertexArray {
		VertexArray::new(buffer, program)
	}

	fn array_attribnf(
		&mut self,
		array: &mut VertexArray,
		n: u8,
		name: &str,
		offset: usize,
	) -> bool {
		array.attribnf(n, name, offset)
	}

	pub fn array_attrib2f(&mut self, array: &mut VertexArray, name: &str, offset: usize) -> bool {
		self.array_attribnf(array, 2, name, offset)
	}

	pub fn array_attrib3f(&mut self, array: &mut VertexArray, name: &str, offset: usize) -> bool {
		self.array_attribnf(array, 3, name, offset)
	}

	pub fn array_attrib4f(&mut self, array: &mut VertexArray, name: &str, offset: usize) -> bool {
		self.array_attribnf(array, 4, name, offset)
	}

	pub fn size(&self) -> (i32, i32) {
		let mut x = 0;
		let mut y = 0;
		unsafe { sdl::get_window_size(self.sdlwin, &mut x, &mut y) };
		(x, y)
	}

	pub fn aspect_ratio(&self) -> f32 {
		let (w, h) = self.size();
		w as f32 / h as f32
	}

	pub fn viewport(&mut self, x: i32, y: i32, w: i32, h: i32) {
		unsafe { gl::Viewport(x, y, w, h) };
	}

	pub fn viewport_full_screen(&mut self) {
		let (w, h) = self.size();
		self.viewport(0, 0, w, h);
	}

	pub fn next_event(&mut self) -> Option<Event> {
		loop {
			let sdl = unsafe { sdl::poll_event() }?;
			let r#type = unsafe { sdl.r#type };
			match r#type {
				sdl::SDL_QUIT => return Some(Event::Quit),
				sdl::SDL_KEYDOWN | sdl::SDL_KEYUP => {
					let scancode = unsafe { sdl.key }.keysym.scancode;
					if let Some(k) = Key::from_sdl(scancode) {
						if r#type == sdl::SDL_KEYDOWN {
							return Some(Event::KeyDown(k));
						} else {
							return Some(Event::KeyUp(k));
						}
					}
				}
				sdl::SDL_MOUSEMOTION => {
					let motion = unsafe { sdl.motion };
					return Some(Event::MouseMotion {
						x: motion.x,
						y: motion.y,
						xrel: motion.xrel,
						yrel: motion.yrel,
					});
				}
				_ => {}
			}
		}
	}

	pub fn clear_screen(&mut self, color: ColorF32) {
		unsafe {
			gl::ClearColor(color.r, color.g, color.b, color.a);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
	}

	pub fn create_rgba_texture(&mut self, params: &TextureParams) -> Texture {
		let mut id: GLuint = 0;
		unsafe {
			gl::GenTextures(1, (&mut id) as *mut GLuint);
		}
		Texture {
			id,
			params: params.clone(),
		}
	}

	pub fn set_texture_data(
		&mut self,
		texture: &mut Texture,
		data: &[u8],
		width: usize,
		height: usize,
	) -> Result<(), String> {
		let width = width as GLsizei;
		let height = height as GLsizei;
		let expected_len = 4 * width * height;
		if data.len() as GLsizei != expected_len {
			return Err(format!(
				"bad data length (expected {}, got {})",
				expected_len,
				data.len()
			));
		}
		let params = &texture.params;
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, texture.id);
			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				gl::RGBA as _,
				width,
				height,
				0,
				gl::RGBA,
				gl::UNSIGNED_BYTE,
				data.as_ptr() as _,
			);
			gl::TexParameteri(
				gl::TEXTURE_2D,
				gl::TEXTURE_MIN_FILTER,
				params.min_filter.to_gl(),
			);
			gl::TexParameteri(
				gl::TEXTURE_2D,
				gl::TEXTURE_MAG_FILTER,
				params.mag_filter.to_gl(),
			);
		}
		Ok(())
	}

	pub fn set_audio_callback(&mut self, callback: AudioCallback) -> Result<(), String> {
		if self.audio_data.is_some() {
			return Err("audio callback already set.".into());
		}

		extern "C" fn sdl_callback(userdata: *mut c_void, stream: *mut u8, len: c_int) {
			let data = unsafe { (userdata as *const AudioData).as_ref() }.unwrap();
			// this should never panick, since SDL shouldn't pass us a negative length.
			let samples: usize = (len / 4).try_into().unwrap();
			let slice = unsafe { std::slice::from_raw_parts_mut(stream as *mut f32, samples) };
			slice.fill(0.0);
			(data.callback)(data.sample_rate, slice);
		}

		let mut data = Box::new(AudioData {
			callback,
			device: 0,
			sample_rate: 0,
		});

		let desired = sdl::SDL_AudioSpec::new(
			sdl_callback,
			&*data as *const AudioData as *mut c_void,
			2,
			sdl::AUDIO_F32,
			44100,
			4096,
		);

		let (obtained, id) = unsafe {
			sdl::open_audio_device(
				None,
				false,
				&desired,
				sdl::SDL_AUDIO_ALLOW_FREQUENCY_CHANGE | sdl::SDL_AUDIO_ALLOW_SAMPLES_CHANGE,
			)?
		};
		data.sample_rate = obtained.freq.try_into().unwrap();
		data.device = id;

		self.audio_data = Some(data);

		unsafe { sdl::pause_audio_device(id, false) };

		Ok(())
	}

	pub fn use_program(&mut self, program: &Program) {
		if self.used_program != program.id {
			unsafe { gl::UseProgram(program.id) };
			self.used_program = program.id;
		}
	}

	fn get_uniform_location(&self, name: &str) -> Option<GLint> {
		if self.used_program == 0 {
			return None;
		}
		let cstring = CString::new(name).ok()?;
		let cstr = cstring.as_ptr() as *const GLchar;
		let loc = unsafe { gl::GetUniformLocation(self.used_program, cstr) };
		if loc == -1 {
			None
		} else {
			Some(loc)
		}
	}

	pub fn active_texture(&mut self, slot: u32, texture: &Texture) {
		unsafe {
			gl::ActiveTexture(gl::TEXTURE0 + slot);
			gl::BindTexture(gl::TEXTURE_2D, texture.id);
		}
	}

	pub fn uniform1i(&mut self, name: &str, x: i32) {
		let loc = self.get_uniform_location(name).unwrap_or(-1);
		unsafe {
			gl::Uniform1i(loc, x);
		}
	}

	pub fn uniform2i(&mut self, name: &str, x: i32, y: i32) {
		let loc = self.get_uniform_location(name).unwrap_or(-1);
		unsafe {
			gl::Uniform2i(loc, x, y);
		}
	}

	pub fn uniform3i(&mut self, name: &str, x: i32, y: i32, z: i32) {
		let loc = self.get_uniform_location(name).unwrap_or(-1);
		unsafe {
			gl::Uniform3i(loc, x, y, z);
		}
	}

	pub fn uniform4i(&mut self, name: &str, x: i32, y: i32, z: i32, w: i32) {
		let loc = self.get_uniform_location(name).unwrap_or(-1);
		unsafe {
			gl::Uniform4i(loc, x, y, z, w);
		}
	}

	pub fn uniform1f(&mut self, name: &str, x: f32) {
		let loc = self.get_uniform_location(name).unwrap_or(-1);
		unsafe {
			gl::Uniform1f(loc, x);
		}
	}

	pub fn uniform2f(&mut self, name: &str, x: f32, y: f32) {
		let loc = self.get_uniform_location(name).unwrap_or(-1);
		unsafe {
			gl::Uniform2f(loc, x, y);
		}
	}

	pub fn uniform2f_slice(&mut self, name: &str, xy: &[f32]) {
		assert_eq!(xy.len(), 2);
		self.uniform2f(name, xy[0], xy[1])
	}

	pub fn uniform3f(&mut self, name: &str, x: f32, y: f32, z: f32) {
		let loc = self.get_uniform_location(name).unwrap_or(-1);
		unsafe {
			gl::Uniform3f(loc, x, y, z);
		}
	}

	pub fn uniform3f_slice(&mut self, name: &str, xyz: &[f32]) {
		assert_eq!(xyz.len(), 3);
		self.uniform3f(name, xyz[0], xyz[1], xyz[2])
	}

	pub fn uniform4f(&mut self, name: &str, x: f32, y: f32, z: f32, w: f32) {
		let loc = self.get_uniform_location(name).unwrap_or(-1);
		unsafe {
			gl::Uniform4f(loc, x, y, z, w);
		}
	}

	pub fn uniform4f_slice(&mut self, name: &str, xyzw: &[f32]) {
		assert_eq!(xyzw.len(), 4);
		self.uniform4f(name, xyzw[0], xyzw[1], xyzw[2], xyzw[3])
	}

	pub fn uniform3x3f(&mut self, name: &str, matrix: &[f32]) {
		assert_eq!(matrix.len(), 9);
		let loc = self.get_uniform_location(name).unwrap_or(-1);
		unsafe {
			gl::UniformMatrix3fv(loc, 1, 0, matrix.as_ptr());
		}
	}

	pub fn uniform4x4f(&mut self, name: &str, matrix: &[f32]) {
		assert_eq!(matrix.len(), 16);
		let loc = self.get_uniform_location(name).unwrap_or(-1);
		unsafe {
			gl::UniformMatrix4fv(loc, 1, 0, matrix.as_ptr());
		}
	}

	pub fn uniform_texture(&mut self, name: &str, slot: u32) {
		self.uniform1i(name, slot as i32);
	}

	pub fn draw_array(&mut self, array: &VertexArray) {
		array.draw();
	}

	pub fn is_key_down(&mut self, key: Key) -> bool {
		let kbd_state = unsafe { sdl::get_keyboard_state() };
		kbd_state[key.to_sdl() as usize] != 0
	}

	pub fn any_key_down(&mut self, keys: &[Key]) -> bool {
		keys.iter().any(|&k| self.is_key_down(k))
	}

	pub fn is_shift_down(&mut self) -> bool {
		self.is_key_down(Key::LShift) || self.is_key_down(Key::RShift)
	}

	pub fn is_ctrl_down(&mut self) -> bool {
		self.is_key_down(Key::LCtrl) || self.is_key_down(Key::RCtrl)
	}

	pub fn is_alt_down(&mut self) -> bool {
		self.is_key_down(Key::LAlt) || self.is_key_down(Key::RAlt)
	}

	pub fn swap(&mut self) {
		unsafe { sdl::gl_swap_window(self.sdlwin) };
	}
}

impl Drop for Window {
	fn drop(&mut self) {
		unsafe {
			if let Some(audio_data) = &self.audio_data {
				sdl::close_audio_device(audio_data.device);
			}
			sdl::gl_delete_context(self.glctx);
			sdl::destroy_window(self.sdlwin);
		}
	}
}
