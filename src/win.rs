// all OpenGL calls are done through the Window.
// this is because OpenGL is not thread safe.
use crate::sdl;
use gl::types::{GLchar, GLenum, GLint, GLsizei, GLuint, GLvoid};
use mem::size_of;
#[allow(unused_imports)]
use std::ffi::{c_char, c_int, c_uint, c_void, CStr, CString};
use std::sync::Mutex;
use std::{fmt, mem};

pub type AudioCallback = fn(sample_rate: u32, samples: &mut [f32]);

/// dammit rust why wont you stabilize negative_impls
type NoSendSync = *const u8;

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
	Minus,
	Equals,
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
	NumPadPeriod,
	NumPadPlus,
	NumPadMinus,
	NumPadDivide,
	NumPadMultiply,
	NumPadEnter,
	NumLock,
	Up,
	Left,
	Right,
	Down,
	PrintScreen,
	ScrollLock,
	Pause,
	Home,
	End,
	Insert,
	Delete,
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
	LGui,
	RGui,
	LeftBracket,
	RightBracket,
	Backslash,
	Semicolon,
	Quote,
	Comma,
	Period,
	Slash,
	Backtick,
	Backspace,
	Tab,
	CapsLock,
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
			MINUS => Key::Minus,
			EQUALS => Key::Equals,
			UP => Key::Up,
			LEFT => Key::Left,
			RIGHT => Key::Right,
			DOWN => Key::Down,
			PRINTSCREEN => Key::PrintScreen,
			SCROLLLOCK => Key::ScrollLock,
			PAUSE => Key::Pause,
			HOME => Key::Home,
			END => Key::End,
			INSERT => Key::Insert,
			DELETE => Key::Delete,
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
			KP_PERIOD => Key::NumPadPeriod,
			KP_PLUS => Key::NumPadPlus,
			KP_MINUS => Key::NumPadMinus,
			KP_DIVIDE => Key::NumPadDivide,
			KP_MULTIPLY => Key::NumPadMultiply,
			KP_ENTER => Key::NumPadEnter,
			NUMLOCKCLEAR => Key::NumLock,
			LSHIFT => Key::LShift,
			RSHIFT => Key::RShift,
			LCTRL => Key::LCtrl,
			RCTRL => Key::RCtrl,
			LALT => Key::LAlt,
			RALT => Key::RAlt,
			LGUI => Key::LGui,
			RGUI => Key::RGui,
			LEFTBRACKET => Key::LeftBracket,
			RIGHTBRACKET => Key::RightBracket,
			BACKSLASH => Key::Backslash,
			SEMICOLON => Key::Semicolon,
			APOSTROPHE => Key::Quote,
			COMMA => Key::Comma,
			PERIOD => Key::Period,
			SLASH => Key::Slash,
			GRAVE => Key::Backtick,
			BACKSPACE => Key::Backspace,
			TAB => Key::Tab,
			CAPSLOCK => Key::CapsLock,
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
			Key::Minus => MINUS,
			Key::Equals => EQUALS,
			Key::LeftBracket => LEFTBRACKET,
			Key::RightBracket => RIGHTBRACKET,
			Key::Backslash => BACKSLASH,
			Key::Semicolon => SEMICOLON,
			Key::Comma => COMMA,
			Key::Period => PERIOD,
			Key::Slash => SLASH,
			Key::Quote => APOSTROPHE,
			Key::Backtick => GRAVE,
			Key::Backspace => BACKSPACE,
			Key::Up => UP,
			Key::Left => LEFT,
			Key::Right => RIGHT,
			Key::Down => DOWN,
			Key::Escape => ESCAPE,
			Key::PageUp => PAGEUP,
			Key::PageDown => PAGEDOWN,
			Key::Tab => TAB,
			Key::CapsLock => CAPSLOCK,
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
			Key::NumPadPeriod => KP_PERIOD,
			Key::NumPadPlus => KP_PLUS,
			Key::NumPadMinus => KP_MINUS,
			Key::NumPadMultiply => KP_MULTIPLY,
			Key::NumPadDivide => KP_DIVIDE,
			Key::NumPadEnter => KP_ENTER,
			Key::NumLock => NUMLOCKCLEAR,
			Key::ScrollLock => SCROLLLOCK,
			Key::PrintScreen => PRINTSCREEN,
			Key::Pause => PAUSE,
			Key::Home => HOME,
			Key::End => END,
			Key::Insert => INSERT,
			Key::Delete => DELETE,
			Key::LShift => LSHIFT,
			Key::RShift => RSHIFT,
			Key::LCtrl => LCTRL,
			Key::RCtrl => RCTRL,
			Key::LAlt => LALT,
			Key::RAlt => RALT,
			Key::LGui => LGUI,
			Key::RGui => RGUI,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyModifier {
	lctrl: bool,
	lshift: bool,
	lalt: bool,
	lgui: bool,
	rctrl: bool,
	rshift: bool,
	ralt: bool,
	rgui: bool,
	numlock: bool,
	capslock: bool,
}

impl KeyModifier {
	pub fn lctrl(&self) -> bool {
		self.lctrl
	}
	pub fn rctrl(&self) -> bool {
		self.rctrl
	}
	pub fn lshift(&self) -> bool {
		self.lshift
	}
	pub fn rshift(&self) -> bool {
		self.rshift
	}
	pub fn lalt(&self) -> bool {
		self.lalt
	}
	pub fn ralt(&self) -> bool {
		self.ralt
	}
	pub fn lgui(&self) -> bool {
		self.lgui
	}
	pub fn rgui(&self) -> bool {
		self.rgui
	}
	pub fn ctrl(&self) -> bool {
		self.lctrl || self.rctrl
	}
	pub fn shift(&self) -> bool {
		self.lshift || self.rshift
	}
	pub fn alt(&self) -> bool {
		self.lalt || self.ralt
	}
	pub fn gui(&self) -> bool {
		self.lgui || self.rgui
	}
	pub fn capslock(&self) -> bool {
		self.capslock
	}
	pub fn numlock(&self) -> bool {
		self.numlock
	}

	fn from_sdl(keymod: u16) -> Self {
		Self {
			lctrl: (keymod & sdl::KMOD_LCTRL) != 0,
			rctrl: (keymod & sdl::KMOD_RCTRL) != 0,
			lshift: (keymod & sdl::KMOD_LSHIFT) != 0,
			rshift: (keymod & sdl::KMOD_RSHIFT) != 0,
			lalt: (keymod & sdl::KMOD_LALT) != 0,
			ralt: (keymod & sdl::KMOD_RALT) != 0,
			lgui: (keymod & sdl::KMOD_LGUI) != 0,
			rgui: (keymod & sdl::KMOD_RGUI) != 0,
			capslock: (keymod & sdl::KMOD_CAPS) != 0,
			numlock: (keymod & sdl::KMOD_NUM) != 0,
		}
	}
}

#[derive(Debug, Clone)]
pub enum Event {
	Quit,
	KeyDown {
		key: Key,
		modifier: KeyModifier,
	},
	KeyUp {
		key: Key,
		modifier: KeyModifier,
	},
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

/// `Color` trait for dealing with opengl
///
/// # Safety
/// Putting the wrong value for the constants may
/// result in bad memory reads/writes. Specifically, you must ensure that
/// the `Color` object is what OpenGL expects for the given format and type.
///
/// ideally we'd have `GL_INTERNAL_FORMAT` as well, but `glGetTexImage` doesn't have it,
/// so best not to.
pub unsafe trait Color: Default + Copy {
	const GL_FORMAT: GLenum;
	const GL_TYPE: GLenum;
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ColorF32 {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
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

unsafe impl Color for ColorU8 {
	const GL_FORMAT: GLenum = gl::RGBA;
	const GL_TYPE: GLenum = gl::UNSIGNED_BYTE;
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

unsafe impl Color for ColorF32 {
	const GL_FORMAT: GLenum = gl::RGBA;
	const GL_TYPE: GLenum = gl::FLOAT;
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ColorGrayscaleF32 {
	pub value: f32
}

impl ColorGrayscaleF32 {
	pub const fn new(value: f32) -> Self {
		Self { value }
	}
}

unsafe impl Color for ColorGrayscaleF32 {
	const GL_FORMAT: GLenum = gl::RED;
	const GL_TYPE: GLenum = gl::FLOAT;
}

pub struct Shader {
	id: GLuint,
	/// shaders should not be sent across threads because of the drop function.
	_unused: NoSendSync,
}

impl Shader {
	unsafe fn new(r#type: GLenum, source: &str) -> Result<Self, String> {
		let id = gl::CreateShader(r#type);
		let result = Self::new_with_id(id, r#type, source);
		if result.is_err() {
			gl::DeleteShader(id);
		}
		result
	}

	unsafe fn new_with_id(id: GLuint, r#type: GLenum, source: &str) -> Result<Self, String> {
		if id == 0 {
			return Err(format!(
				"couldn't create shader (GL error {})",
				gl::GetError()
			));
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

			gl::ShaderSource(id, sources.len() as _, sources_ptr, lengths_ptr);
		}

		gl::CompileShader(id);
		{
			//check log
			let mut log = [0u8; 1024];
			let mut len: GLsizei = 0;
			let logp = &mut log as *mut u8 as *mut GLchar;
			let lenp = &mut len as *mut GLsizei;
			gl::GetShaderInfoLog(id, log.len() as GLsizei, lenp, logp);
			if len > 0 {
				eprintln!("{}", String::from_utf8_lossy(&log[..len as usize]));
			}
		}
		{
			let mut status: GLint = 0;
			gl::GetShaderiv(id, gl::COMPILE_STATUS, (&mut status) as _);
			if status == 0 {
				return Err("failed to compile".to_string());
			}
		}

		Ok(Self {
			id,
			_unused: 0 as _,
		})
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe { gl::DeleteShader(self.id) };
	}
}

pub struct Program {
	id: GLuint,
	/// programs should not be sent across threads because of the drop function.
	_unused: NoSendSync,
}

impl Program {
	unsafe fn new() -> Self {
		let id = gl::CreateProgram();
		Self {
			id,
			_unused: 0 as _,
		}
	}

	unsafe fn new_with_shaders(shaders: &[Shader]) -> Result<Self, String> {
		let mut program = Self::new();
		program.relink(shaders)?;
		Ok(program)
	}

	unsafe fn relink(&mut self, shaders: &[Shader]) -> Result<(), String> {
		let id = self.id;
		for shader in shaders {
			gl::AttachShader(id, shader.id);
		}
		gl::LinkProgram(id);
		{
			// check log
			let mut log = [0u8; 1024];
			let mut len: GLsizei = 0;
			let logp = &mut log as *mut u8 as *mut GLchar;
			let lenp = &mut len as *mut GLsizei;
			gl::GetProgramInfoLog(id, log.len() as GLsizei, lenp, logp);
			if len > 0 {
				eprintln!("{}", String::from_utf8_lossy(&log[..len as usize]));
			}
		}

		{
			let mut status: GLint = 0;
			gl::GetProgramiv(id, gl::LINK_STATUS, (&mut status) as _);
			if status == 0 {
				return Err("failed to link".to_string());
			}
		}

		for shader in shaders {
			gl::DetachShader(id, shader.id);
		}

		Ok(())
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
	/// buffers should not be sent across threads because of the drop function.
	_unused: NoSendSync,
}

impl Buffer {
	unsafe fn new() -> Self {
		let mut id = 0;
		gl::CreateBuffers(1, &mut id as *mut GLuint);
		Self {
			id,
			stride: 0,
			count: 0,
			_unused: 0 as _,
		}
	}

	unsafe fn bind(&self) {
		gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
	}

	unsafe fn set_data<T>(&mut self, data: &[T]) {
		gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
		self.count = data.len() as u32;
		self.stride = mem::size_of::<T>() as u32;

		gl::BufferData(
			gl::ARRAY_BUFFER,
			(self.count * self.stride) as _,
			data.as_ptr() as _,
			gl::STATIC_DRAW,
		);
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
	/// vertex arrays should not be sent across threads because of the drop function.
	_unused: NoSendSync,
}

impl VertexArray {
	unsafe fn new(buffer: Buffer, program: &Program) -> Self {
		let mut id: GLuint = 0;

		gl::GenVertexArrays(1, &mut id as *mut GLuint);

		Self {
			id,
			buffer,
			program: program.id,
			_unused: 0 as _,
		}
	}

	unsafe fn bind(&self) {
		gl::BindVertexArray(self.id);
	}

	unsafe fn attribnf(&mut self, n: u8, name: &str, offset: usize) -> bool {
		let Ok(cstring) = CString::new(name) else { return false };
		let cstr = cstring.as_ptr() as *const GLchar;
		let loc = gl::GetAttribLocation(self.program, cstr);
		let Ok(loc) = loc.try_into() else { return false };

		if offset + usize::from(n) * size_of::<f32>() > self.buffer.stride as usize {
			// offset too large
			return false;
		}

		self.bind();
		self.buffer.bind();
		gl::VertexAttribPointer(
			loc,
			n.into(),
			gl::FLOAT,
			0,
			self.buffer.stride as _,
			offset as _,
		);
		gl::EnableVertexAttribArray(loc);
		true
	}

	unsafe fn draw(&self) {
		self.bind();
		gl::DrawArrays(gl::TRIANGLES, 0, self.buffer.count as i32);
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
	width: usize,
	height: usize,
	/// textures should not be sent across threads because of the drop function.
	_unused: NoSendSync,
}

impl Texture {
	unsafe fn new(params: &TextureParams) -> Self {
		let mut id: GLuint = 0;
		gl::GenTextures(1, (&mut id) as *mut GLuint);
		Self {
			id,
			params: params.clone(),
			width: 0,
			height: 0,
			_unused: 0 as _,
		}
	}

	unsafe fn bind(&self) {
		gl::BindTexture(gl::TEXTURE_2D, self.id);
	}

	unsafe fn set_data<T: Color>(
		&mut self,
		data: Option<&[T]>,
		width: usize,
		height: usize,
	) -> Result<(), String> {
		self.width = width;
		self.height = height;
		let width: GLsizei = width.try_into().map_err(|_| "width too large")?;
		let height: GLsizei = height.try_into().map_err(|_| "height too large")?;
		let expected_len = width * height;

		let ptr = match data {
			Some(data) => {
				if data.len() as GLsizei != expected_len {
					return Err(format!(
						"bad data length (expected {}, got {})",
						expected_len,
						data.len()
					));
				}
				data.as_ptr()
			}
			None => std::ptr::null(),
		};

		let params = &self.params;
		self.bind();
		gl::TexImage2D(
			gl::TEXTURE_2D,
			0,
			T::GL_FORMAT as GLint,
			width,
			height,
			0,
			T::GL_FORMAT,
			T::GL_TYPE,
			ptr.cast(),
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
		Ok(())
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height
	}

	/// panicks if `data` is the wrong length (should be exactly `self.width() * self.height()`).
	unsafe fn get_data<T: Color>(&self, data: &mut [T]) {
		assert_eq!(data.len(), self.width * self.height, "Bad data size.");
		self.bind();
		gl::GetTexImage(
			gl::TEXTURE_2D,
			0,
			T::GL_FORMAT,
			T::GL_TYPE,
			data.as_ptr() as *mut GLvoid,
		);
	}

	unsafe fn get_data_vec<T: Color>(&self) -> Vec<T> {
		let mut data = vec![T::default(); self.width * self.height];
		self.get_data(&mut data);
		data
	}
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

#[derive(Clone, Copy)]
pub enum FramebufferAttachment {
	// 8 color attachments ought to be enough for anyone.
	Color0,
	Color1,
	Color2,
	Color3,
	Color4,
	Color5,
	Color6,
	Color7,
	Depth,
	Stencil,
	DepthStencil,
}

impl FramebufferAttachment {
	fn to_gl(self) -> GLenum {
		use FramebufferAttachment::*;
		match self {
			Color0 => gl::COLOR_ATTACHMENT0,
			Color1 => gl::COLOR_ATTACHMENT1,
			Color2 => gl::COLOR_ATTACHMENT2,
			Color3 => gl::COLOR_ATTACHMENT3,
			Color4 => gl::COLOR_ATTACHMENT4,
			Color5 => gl::COLOR_ATTACHMENT5,
			Color6 => gl::COLOR_ATTACHMENT6,
			Color7 => gl::COLOR_ATTACHMENT7,
			Depth => gl::DEPTH_ATTACHMENT,
			Stencil => gl::STENCIL_ATTACHMENT,
			DepthStencil => gl::DEPTH_STENCIL_ATTACHMENT,
		}
	}
}

pub struct Framebuffer {
	id: GLuint,
	_unused: NoSendSync,
}

impl Framebuffer {
	unsafe fn new() -> Self {
		let mut id: GLuint = 0;
		gl::GenFramebuffers(1, (&mut id) as *mut GLuint);
		Self {
			id,
			_unused: 0 as _,
		}
	}

	unsafe fn bind(&self) {
		gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
	}

	unsafe fn unbind() {
		gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
	}

	unsafe fn set_texture(&mut self, attachment: FramebufferAttachment, texture: &Texture) {
		self.bind();
		texture.bind();
		gl::FramebufferTexture2D(
			gl::FRAMEBUFFER,
			attachment.to_gl(),
			gl::TEXTURE_2D,
			texture.id,
			0,
		);
		Self::unbind();
	}
}

impl Drop for Framebuffer {
	fn drop(&mut self) {
		unsafe { gl::DeleteFramebuffers(1, (&self.id) as *const GLuint) };
	}
}

pub struct WindowProperties {
	shown: bool,
	resizable: bool,
}

impl Default for WindowProperties {
	fn default() -> Self {
		Self {
			shown: true,
			resizable: true,
		}
	}
}

impl Window {
	pub fn new(
		title: &str,
		width: i32,
		height: i32,
		properties: &WindowProperties,
	) -> Result<Self, String> {
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
			sdl::set_main_ready();
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
		if !properties.shown {
			flags |= sdl::SDL_WINDOW_HIDDEN;
		}
		if properties.resizable {
			flags |= sdl::SDL_WINDOW_RESIZABLE;
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

	pub fn set_vsync(&mut self, vsync: bool) {
		unsafe {
			sdl::gl_set_swap_interval(vsync.into());
		}
	}

	pub fn set_fullscreen(&mut self, fullscreen: bool) {
		unsafe {
			// i dont care if going fullscreen fails
			let _ = sdl::set_window_fullscreen(
				self.sdlwin,
				if fullscreen {
					sdl::SDL_WINDOW_FULLSCREEN_DESKTOP
				} else {
					0
				},
			);
		}
	}

	pub fn show(&mut self) {
		unsafe { sdl::show_window(self.sdlwin) };
	}

	pub fn set_mouse_relative(&mut self, relative: bool) {
		unsafe {
			sdl::set_relative_mouse_mode(relative);
		}
	}

	/// new empty shader program
	pub fn new_program(&mut self) -> Program {
		unsafe { Program::new() }
	}

	pub fn create_program(
		&mut self,
		source_vshader: &str,
		source_fshader: &str,
	) -> Result<Program, String> {
		let vshader = unsafe { Shader::new(gl::VERTEX_SHADER, source_vshader) }?;
		let fshader = unsafe { Shader::new(gl::FRAGMENT_SHADER, source_fshader) }?;
		unsafe { Program::new_with_shaders(&[vshader, fshader]) }
	}

	pub fn link_program(
		&mut self,
		program: &mut Program,
		source_vshader: &str,
		source_fshader: &str,
	) -> Result<(), String> {
		let vshader = unsafe { Shader::new(gl::VERTEX_SHADER, source_vshader) }?;
		let fshader = unsafe { Shader::new(gl::FRAGMENT_SHADER, source_fshader) }?;
		unsafe { program.relink(&[vshader, fshader]) }
	}

	pub fn create_buffer(&mut self) -> Buffer {
		unsafe { Buffer::new() }
	}

	pub fn set_buffer_data<T>(&mut self, buffer: &mut Buffer, data: &[T]) {
		unsafe { buffer.set_data(data) };
	}

	pub fn create_vertex_array(&mut self, buffer: Buffer, program: &Program) -> VertexArray {
		unsafe { VertexArray::new(buffer, program) }
	}

	fn array_attribnf(
		&mut self,
		array: &mut VertexArray,
		n: u8,
		name: &str,
		offset: usize,
	) -> bool {
		unsafe { array.attribnf(n, name, offset) }
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

	pub fn create_framebuffer(&mut self) -> Framebuffer {
		unsafe { Framebuffer::new() }
	}

	/// Attach texture to framebuffer.
	/// In theory this should check that `framebuffer` does not outlive `texture`,
	/// but that would be difficult to do in a nice way.
	pub fn set_framebuffer_texture(
		&mut self,
		framebuffer: &mut Framebuffer,
		attachment: FramebufferAttachment,
		texture: &Texture,
	) {
		unsafe { framebuffer.set_texture(attachment, texture) };
	}

	pub fn bind_framebuffer(&mut self, framebuffer: Option<&Framebuffer>) {
		match framebuffer {
			Some(f) => unsafe { f.bind() },
			None => unsafe { Framebuffer::unbind() },
		}
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
					let keysym = unsafe { sdl.key }.keysym;
					let scancode = keysym.scancode;
					if let Some(key) = Key::from_sdl(scancode) {
						let modifier = KeyModifier::from_sdl(keysym.r#mod);
						if r#type == sdl::SDL_KEYDOWN {
							return Some(Event::KeyDown { key, modifier });
						} else {
							return Some(Event::KeyUp { key, modifier });
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

	pub fn create_texture(&mut self, params: &TextureParams) -> Texture {
		unsafe { Texture::new(params) }
	}

	pub fn set_texture_data<T: Color>(
		&mut self,
		texture: &mut Texture,
		data: &[T],
		width: usize,
		height: usize,
	) -> Result<(), String> {
		unsafe { texture.set_data(Some(data), width, height) }?;
		Ok(())
	}

	/// sets texture width + height but not data.
	///
	/// NOTE: you must still specify the color type!
	/// for framebuffers, etc.
	pub fn set_texture_no_data<T: Color>(
		&mut self,
		texture: &mut Texture,
		width: usize,
		height: usize,
	) -> Result<(), String> {
		unsafe { texture.set_data::<T>(None, width, height) }?;
		Ok(())
	}

	/// get texture image
	///
	/// panicks if `data.len() != texture.width() * texture.height()`
	pub fn get_texture_data<T: Color>(&mut self, texture: &Texture, data: &mut [T]) {
		unsafe { texture.get_data(data) };
	}

	/// get texture image as a newly-allocated `Vec`
	pub fn get_texture_data_vec<T: Color>(&mut self, texture: &Texture) -> Vec<T> {
		unsafe { texture.get_data_vec() }
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
		unsafe { array.draw() };
	}

	pub fn is_key_down(&self, key: Key) -> bool {
		let kbd_state = unsafe { sdl::get_keyboard_state() };
		kbd_state[key.to_sdl() as usize] != 0
	}

	pub fn any_key_down(&self, keys: &[Key]) -> bool {
		keys.iter().any(|&k| self.is_key_down(k))
	}

	pub fn is_shift_down(&self) -> bool {
		self.is_key_down(Key::LShift) || self.is_key_down(Key::RShift)
	}

	pub fn is_ctrl_down(&self) -> bool {
		self.is_key_down(Key::LCtrl) || self.is_key_down(Key::RCtrl)
	}

	pub fn is_alt_down(&self) -> bool {
		self.is_key_down(Key::LAlt) || self.is_key_down(Key::RAlt)
	}

	pub fn swap(&mut self) {
		unsafe { sdl::gl_swap_window(self.sdlwin) };
	}

	pub fn get_clipboard_text(&mut self) -> Result<String, String> {
		unsafe { sdl::get_clipboard_text() }
	}

	pub fn set_clipboard_text(&mut self, s: &str) -> Result<(), String> {
		unsafe { sdl::set_clipboard_text(s) }
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
