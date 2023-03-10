#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
/// this module provides SDL type definitions, and more rust-y wrappers around
/// SDL functions.
use std::ffi::{c_char, c_float, c_int, c_void, CStr, CString};
use std::mem;

// not a real type, just here to differentiate *mut () from SDL_GLContext
#[repr(C)]
pub struct SDL_GLContextData(u8);
pub type SDL_bool = c_int;
pub type SDL_EventType = u32;
#[repr(C)]
pub struct SDL_Window(u8);
#[repr(transparent)]
pub struct SDL_SysWMmsg(c_void); // opaque type
pub type SDL_Keycode = u32;
pub type SDL_Scancode = i32;
pub type SDL_JoystickID = i32;
pub type SDL_JoystickPowerLevel = c_int;
pub type SDL_GestureID = i64;
pub type SDL_TouchID = i64;
pub type SDL_FingerID = i64;
pub type SDL_AudioFormat = u16;
pub type SDL_AudioCallback = extern "C" fn(*mut c_void, *mut u8, c_int);
pub type SDL_AudioDeviceID = u32;
pub type SDL_WindowEventID = c_int;
pub type SDL_GLContext = *mut SDL_GLContextData;
pub type SDL_GLattr = c_int;
#[repr(transparent)]
pub struct SDL_BlitMap(c_void); // opaque type
#[repr(transparent)]
pub struct SDL_RWops(c_void); // for most purposes, you don't care about the internals of this struct

pub const SDL_WINDOWPOS_UNDEFINED: c_int = 0x1FFF0000;
pub const SDL_QUIT: SDL_EventType = 0x100;
pub const SDL_APP_TERMINATING: SDL_EventType = 0x101;
pub const SDL_APP_LOWMEMORY: SDL_EventType = 0x102;
pub const SDL_APP_WILLENTERBACKGROUND: SDL_EventType = 0x103;
pub const SDL_APP_DIDENTERBACKGROUND: SDL_EventType = 0x104;
pub const SDL_APP_WILLENTERFOREGROUND: SDL_EventType = 0x105;
pub const SDL_APP_DIDENTERFOREGROUND: SDL_EventType = 0x106;
pub const SDL_LOCALECHANGED: SDL_EventType = 0x107;
pub const SDL_DISPLAYEVENT: SDL_EventType = 0x150;
pub const SDL_WINDOWEVENT: SDL_EventType = 0x200;
pub const SDL_SYSWMEVENT: SDL_EventType = 0x201;
pub const SDL_KEYDOWN: SDL_EventType = 0x300;
pub const SDL_KEYUP: SDL_EventType = 0x301;
pub const SDL_TEXTEDITING: SDL_EventType = 0x302;
pub const SDL_TEXTINPUT: SDL_EventType = 0x303;
pub const SDL_KEYMAPCHANGED: SDL_EventType = 0x304;
pub const SDL_TEXTEDITING_EXT: SDL_EventType = 0x305;
pub const SDL_MOUSEMOTION: SDL_EventType = 0x400;
pub const SDL_MOUSEBUTTONDOWN: SDL_EventType = 0x401;
pub const SDL_MOUSEBUTTONUP: SDL_EventType = 0x402;
pub const SDL_MOUSEWHEEL: SDL_EventType = 0x403;
pub const SDL_JOYAXISMOTION: SDL_EventType = 0x600;
pub const SDL_JOYBALLMOTION: SDL_EventType = 0x601;
pub const SDL_JOYHATMOTION: SDL_EventType = 0x602;
pub const SDL_JOYBUTTONDOWN: SDL_EventType = 0x603;
pub const SDL_JOYBUTTONUP: SDL_EventType = 0x604;
pub const SDL_JOYDEVICEADDED: SDL_EventType = 0x605;
pub const SDL_JOYDEVICEREMOVED: SDL_EventType = 0x606;
pub const SDL_JOYBATTERYUPDATED: SDL_EventType = 0x607;
pub const SDL_CONTROLLERAXISMOTION: SDL_EventType = 0x650;
pub const SDL_CONTROLLERBUTTONDOWN: SDL_EventType = 0x651;
pub const SDL_CONTROLLERBUTTONUP: SDL_EventType = 0x652;
pub const SDL_CONTROLLERDEVICEADDED: SDL_EventType = 0x653;
pub const SDL_CONTROLLERDEVICEREMOVED: SDL_EventType = 0x654;
pub const SDL_CONTROLLERDEVICEREMAPPED: SDL_EventType = 0x655;
pub const SDL_CONTROLLERTOUCHPADDOWN: SDL_EventType = 0x656;
pub const SDL_CONTROLLERTOUCHPADMOTION: SDL_EventType = 0x657;
pub const SDL_CONTROLLERTOUCHPADUP: SDL_EventType = 0x658;
pub const SDL_CONTROLLERSENSORUPDATE: SDL_EventType = 0x659;
pub const SDL_FINGERDOWN: SDL_EventType = 0x700;
pub const SDL_FINGERUP: SDL_EventType = 0x701;
pub const SDL_FINGERMOTION: SDL_EventType = 0x702;
pub const SDL_DOLLARGESTURE: SDL_EventType = 0x800;
pub const SDL_DOLLARRECORD: SDL_EventType = 0x801;
pub const SDL_MULTIGESTURE: SDL_EventType = 0x802;
pub const SDL_CLIPBOARDUPDATE: SDL_EventType = 0x900;
pub const SDL_DROPFILE: SDL_EventType = 0x1000;
pub const SDL_DROPTEXT: SDL_EventType = 0x1001;
pub const SDL_DROPBEGIN: SDL_EventType = 0x1002;
pub const SDL_DROPCOMPLETE: SDL_EventType = 0x1003;
pub const SDL_AUDIODEVICEADDED: SDL_EventType = 0x1100;
pub const SDL_AUDIODEVICEREMOVED: SDL_EventType = 0x1101;
pub const SDL_SENSORUPDATE: SDL_EventType = 0x1200;
pub const SDL_RENDER_TARGETS_RESET: SDL_EventType = 0x2000;
pub const SDL_RENDER_DEVICE_RESET: SDL_EventType = 0x2001;
pub const SDL_POLLSENTINEL: SDL_EventType = 0x7F00;
pub const SDL_USEREVENT: SDL_EventType = 0x8000;

pub const SDL_WINDOWEVENT_SHOWN: SDL_WindowEventID = 1;
pub const SDL_WINDOWEVENT_HIDDEN: SDL_WindowEventID = 2;
pub const SDL_WINDOWEVENT_EXPOSED: SDL_WindowEventID = 3;
pub const SDL_WINDOWEVENT_MOVED: SDL_WindowEventID = 4;
pub const SDL_WINDOWEVENT_RESIZED: SDL_WindowEventID = 5;
pub const SDL_WINDOWEVENT_SIZE_CHANGED: SDL_WindowEventID = 6;
pub const SDL_WINDOWEVENT_MINIMIZED: SDL_WindowEventID = 7;
pub const SDL_WINDOWEVENT_MAXIMIZED: SDL_WindowEventID = 8;
pub const SDL_WINDOWEVENT_RESTORED: SDL_WindowEventID = 9;
pub const SDL_WINDOWEVENT_ENTER: SDL_WindowEventID = 10;
pub const SDL_WINDOWEVENT_LEAVE: SDL_WindowEventID = 11;
pub const SDL_WINDOWEVENT_FOCUS_GAINED: SDL_WindowEventID = 12;
pub const SDL_WINDOWEVENT_FOCUS_LOST: SDL_WindowEventID = 13;
pub const SDL_WINDOWEVENT_CLOSE: SDL_WindowEventID = 14;
pub const SDL_WINDOWEVENT_TAKE_FOCUS: SDL_WindowEventID = 15;
pub const SDL_WINDOWEVENT_HIT_TEST: SDL_WindowEventID = 16;
pub const SDL_WINDOWEVENT_ICCPROF_CHANGED: SDL_WindowEventID = 17;
pub const SDL_WINDOWEVENT_DISPLAY_CHANGED: SDL_WindowEventID = 18;

pub const SDL_INIT_TIMER: u32 = 0x00000001;
pub const SDL_INIT_AUDIO: u32 = 0x00000010;
pub const SDL_INIT_VIDEO: u32 = 0x00000020;
pub const SDL_INIT_JOYSTICK: u32 = 0x00000200;
pub const SDL_INIT_HAPTIC: u32 = 0x00001000;
pub const SDL_INIT_GAMECONTROLLER: u32 = 0x00002000;
pub const SDL_INIT_EVENTS: u32 = 0x00004000;
pub const SDL_INIT_SENSOR: u32 = 0x00008000;
pub const SDL_INIT_EVERYTHING: u32 = SDL_INIT_TIMER
	| SDL_INIT_AUDIO
	| SDL_INIT_VIDEO
	| SDL_INIT_EVENTS
	| SDL_INIT_JOYSTICK
	| SDL_INIT_HAPTIC
	| SDL_INIT_GAMECONTROLLER
	| SDL_INIT_SENSOR;

pub const SDL_WINDOW_FULLSCREEN: u32 = 0x00000001;
pub const SDL_WINDOW_OPENGL: u32 = 0x00000002;
pub const SDL_WINDOW_SHOWN: u32 = 0x00000004;
pub const SDL_WINDOW_HIDDEN: u32 = 0x00000008;
pub const SDL_WINDOW_BORDERLESS: u32 = 0x00000010;
pub const SDL_WINDOW_RESIZABLE: u32 = 0x00000020;
pub const SDL_WINDOW_MINIMIZED: u32 = 0x00000040;
pub const SDL_WINDOW_MAXIMIZED: u32 = 0x00000080;
pub const SDL_WINDOW_MOUSE_GRABBED: u32 = 0x00000100;
pub const SDL_WINDOW_INPUT_FOCUS: u32 = 0x00000200;
pub const SDL_WINDOW_MOUSE_FOCUS: u32 = 0x00000400;
pub const SDL_WINDOW_FULLSCREEN_DESKTOP: u32 = SDL_WINDOW_FULLSCREEN | 0x00001000;
pub const SDL_WINDOW_FOREIGN: u32 = 0x00000800;
pub const SDL_WINDOW_ALLOW_HIGHDPI: u32 = 0x00002000;
pub const SDL_WINDOW_MOUSE_CAPTURE: u32 = 0x00004000;
pub const SDL_WINDOW_ALWAYS_ON_TOP: u32 = 0x00008000;
pub const SDL_WINDOW_SKIP_TASKBAR: u32 = 0x00010000;
pub const SDL_WINDOW_UTILITY: u32 = 0x00020000;
pub const SDL_WINDOW_TOOLTIP: u32 = 0x00040000;
pub const SDL_WINDOW_POPUP_MENU: u32 = 0x00080000;
pub const SDL_WINDOW_KEYBOARD_GRABBED: u32 = 0x00100000;
pub const SDL_WINDOW_VULKAN: u32 = 0x10000000;
pub const SDL_WINDOW_METAL: u32 = 0x20000000;

pub const fn sdl_button_mask(x: u8) -> u32 {
	1u32 << (x - 1)
}

pub const SDL_BUTTON_LEFT: u8 = 1;
pub const SDL_BUTTON_MIDDLE: u8 = 2;
pub const SDL_BUTTON_RIGHT: u8 = 3;
pub const SDL_BUTTON_X1: u8 = 4;
pub const SDL_BUTTON_X2: u8 = 5;
pub const SDL_BUTTON_LMASK: u32 = sdl_button_mask(SDL_BUTTON_LEFT);
pub const SDL_BUTTON_MMASK: u32 = sdl_button_mask(SDL_BUTTON_MIDDLE);
pub const SDL_BUTTON_RMASK: u32 = sdl_button_mask(SDL_BUTTON_RIGHT);
pub const SDL_BUTTON_X1MASK: u32 = sdl_button_mask(SDL_BUTTON_X1);
pub const SDL_BUTTON_X2MASK: u32 = sdl_button_mask(SDL_BUTTON_X2);

pub const SDL_RELEASED: u8 = 0;
pub const SDL_PRESSED: u8 = 1;

pub const SDL_GL_RED_SIZE: SDL_GLattr = 0;
pub const SDL_GL_GREEN_SIZE: SDL_GLattr = 1;
pub const SDL_GL_BLUE_SIZE: SDL_GLattr = 2;
pub const SDL_GL_ALPHA_SIZE: SDL_GLattr = 3;
pub const SDL_GL_BUFFER_SIZE: SDL_GLattr = 4;
pub const SDL_GL_DOUBLEBUFFER: SDL_GLattr = 5;
pub const SDL_GL_DEPTH_SIZE: SDL_GLattr = 6;
pub const SDL_GL_STENCIL_SIZE: SDL_GLattr = 7;
pub const SDL_GL_ACCUM_RED_SIZE: SDL_GLattr = 8;
pub const SDL_GL_ACCUM_GREEN_SIZE: SDL_GLattr = 9;
pub const SDL_GL_ACCUM_BLUE_SIZE: SDL_GLattr = 10;
pub const SDL_GL_ACCUM_ALPHA_SIZE: SDL_GLattr = 11;
pub const SDL_GL_STEREO: SDL_GLattr = 12;
pub const SDL_GL_MULTISAMPLEBUFFERS: SDL_GLattr = 13;
pub const SDL_GL_MULTISAMPLESAMPLES: SDL_GLattr = 14;
pub const SDL_GL_ACCELERATED_VISUAL: SDL_GLattr = 15;
pub const SDL_GL_RETAINED_BACKING: SDL_GLattr = 16;
pub const SDL_GL_CONTEXT_MAJOR_VERSION: SDL_GLattr = 17;
pub const SDL_GL_CONTEXT_MINOR_VERSION: SDL_GLattr = 18;
pub const SDL_GL_CONTEXT_DEBUG_FLAG: i32 = 0x0001;
pub const SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG: i32 = 0x0002;
pub const SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG: i32 = 0x0004;
pub const SDL_GL_CONTEXT_RESET_ISOLATION_FLAG: i32 = 0x0008;
pub const SDL_GL_CONTEXT_EGL: SDL_GLattr = 19;
pub const SDL_GL_CONTEXT_FLAGS: SDL_GLattr = 20;
pub const SDL_GL_CONTEXT_PROFILE_MASK: SDL_GLattr = 21;
pub const SDL_GL_SHARE_WITH_CURRENT_CONTEXT: SDL_GLattr = 22;
pub const SDL_GL_FRAMEBUFFER_SRGB_CAPABLE: SDL_GLattr = 23;
pub const SDL_GL_CONTEXT_RELEASE_BEHAVIOR: SDL_GLattr = 24;
pub const SDL_GL_CONTEXT_RESET_NOTIFICATION: SDL_GLattr = 25;
pub const SDL_GL_CONTEXT_NO_ERROR: SDL_GLattr = 26;
pub const SDL_GL_FLOATBUFFERS: SDL_GLattr = 27;

pub const SDL_MESSAGEBOX_ERROR: u32 = 0x00000010;
pub const SDL_MESSAGEBOX_WARNING: u32 = 0x00000020;
pub const SDL_MESSAGEBOX_INFORMATION: u32 = 0x00000040;

pub const SDL_AUDIO_ALLOW_FREQUENCY_CHANGE: c_int = 0x00000001;
pub const SDL_AUDIO_ALLOW_FORMAT_CHANGE: c_int = 0x00000002;
pub const SDL_AUDIO_ALLOW_CHANNELS_CHANGE: c_int = 0x00000004;
pub const SDL_AUDIO_ALLOW_SAMPLES_CHANGE: c_int = 0x00000008;

pub const AUDIO_U8: u16 = 0x0008;
pub const AUDIO_S8: u16 = 0x8008;
pub const AUDIO_U16LSB: u16 = 0x0010;
pub const AUDIO_S16LSB: u16 = 0x8010;
pub const AUDIO_U16MSB: u16 = 0x1010;
pub const AUDIO_S16MSB: u16 = 0x9010;
pub const AUDIO_U16: u16 = AUDIO_U16LSB;
pub const AUDIO_S16: u16 = AUDIO_S16LSB;
pub const AUDIO_S32LSB: u16 = 0x8020;
pub const AUDIO_S32MSB: u16 = 0x9020;
pub const AUDIO_S32: u16 = AUDIO_S32LSB;
pub const AUDIO_F32LSB: u16 = 0x8120;
pub const AUDIO_F32MSB: u16 = 0x9120;
pub const AUDIO_F32: u16 = AUDIO_F32LSB;

pub const KMOD_NONE: u16 = 0x0000;
pub const KMOD_LSHIFT: u16 = 0x0001;
pub const KMOD_RSHIFT: u16 = 0x0002;
pub const KMOD_LCTRL: u16 = 0x0040;
pub const KMOD_RCTRL: u16 = 0x0080;
pub const KMOD_LALT: u16 = 0x0100;
pub const KMOD_RALT: u16 = 0x0200;
pub const KMOD_LGUI: u16 = 0x0400;
pub const KMOD_RGUI: u16 = 0x0800;
pub const KMOD_NUM: u16 = 0x1000;
pub const KMOD_CAPS: u16 = 0x2000;

// NOTE: ideally we wouldn't need Copy on all of these
// but otherwise we wouldn't be able to put them in a union

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_CommonEvent {
	pub r#type: u32,
	pub timestamp: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_DisplayEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub display: u32,
	pub event: u8,
	pub padding1: u8,
	pub padding2: u8,
	pub padding3: u8,
	pub data1: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_WindowEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub windowID: u32,
	pub event: u8,
	pub padding1: u8,
	pub padding2: u8,
	pub padding3: u8,
	pub data1: i32,
	pub data2: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_Keysym {
	pub scancode: SDL_Scancode,
	pub sym: SDL_Keycode,
	pub r#mod: u16,
	pub unused: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_KeyboardEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub windowID: u32,
	pub state: u8,
	pub repeat: u8,
	pub padding2: u8,
	pub padding3: u8,
	pub keysym: SDL_Keysym,
}

pub const SDL_TEXTEDITINGEVENT_TEXT_SIZE: usize = 32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_TextEditingEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub windowID: u32,
	pub text: [c_char; SDL_TEXTEDITINGEVENT_TEXT_SIZE],
	pub start: i32,
	pub length: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_TextEditingExtEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub windowID: u32,
	pub text: *mut c_char,
	pub start: i32,
	pub length: i32,
}

pub const SDL_TEXTINPUTEVENT_TEXT_SIZE: usize = 32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_TextInputEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub windowID: u32,
	pub text: [c_char; SDL_TEXTINPUTEVENT_TEXT_SIZE],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_MouseMotionEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub windowID: u32,
	pub which: u32,
	pub state: u32,
	pub x: i32,
	pub y: i32,
	pub xrel: i32,
	pub yrel: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_MouseButtonEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub windowID: u32,
	pub which: u32,
	pub button: u8,
	pub state: u8,
	pub clicks: u8,
	pub padding1: u8,
	pub x: i32,
	pub y: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_MouseWheelEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub windowID: u32,
	pub which: u32,
	pub x: i32,
	pub y: i32,
	pub direction: u32,
	pub preciseX: c_float,
	pub preciseY: c_float,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_JoyAxisEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub which: SDL_JoystickID,
	pub axis: u8,
	pub padding1: u8,
	pub padding2: u8,
	pub padding3: u8,
	pub value: i16,
	pub padding4: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_JoyBallEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub which: SDL_JoystickID,
	pub ball: u8,
	pub padding1: u8,
	pub padding2: u8,
	pub padding3: u8,
	pub xrel: i16,
	pub yrel: i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_JoyHatEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub which: SDL_JoystickID,
	pub hat: u8,
	pub value: u8,
	pub padding1: u8,
	pub padding2: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_JoyButtonEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub which: SDL_JoystickID,
	pub button: u8,
	pub state: u8,
	pub padding1: u8,
	pub padding2: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_JoyDeviceEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub which: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_JoyBatteryEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub which: SDL_JoystickID,
	pub level: SDL_JoystickPowerLevel,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_ControllerAxisEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub which: SDL_JoystickID,
	pub axis: u8,
	pub padding1: u8,
	pub padding2: u8,
	pub padding3: u8,
	pub value: i16,
	pub padding4: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_ControllerButtonEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub which: SDL_JoystickID,
	pub button: u8,
	pub state: u8,
	pub padding1: u8,
	pub padding2: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_ControllerDeviceEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub which: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_ControllerTouchpadEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub which: SDL_JoystickID,
	pub touchpad: i32,
	pub finger: i32,
	pub x: c_float,
	pub y: c_float,
	pub pressure: c_float,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_ControllerSensorEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub which: SDL_JoystickID,
	pub sensor: i32,
	pub data: [c_float; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_AudioDeviceEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub which: u32,
	pub iscapture: u8,
	pub padding1: u8,
	pub padding2: u8,
	pub padding3: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_TouchFingerEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub touchId: SDL_TouchID,
	pub fingerId: SDL_FingerID,
	pub x: c_float,
	pub y: c_float,
	pub dx: c_float,
	pub dy: c_float,
	pub pressure: c_float,
	pub windowID: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_MultiGestureEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub touchId: SDL_TouchID,
	pub dTheta: c_float,
	pub dDist: c_float,
	pub x: c_float,
	pub y: c_float,
	pub numFingers: u16,
	pub padding: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_DollarGestureEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub touchId: SDL_TouchID,
	pub gestureId: SDL_GestureID,
	pub numFingers: u32,
	pub error: c_float,
	pub x: c_float,
	pub y: c_float,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_DropEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub file: *mut c_char,
	pub windowID: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_SensorEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub which: i32,
	pub data: [c_float; 6],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_QuitEvent {
	pub r#type: u32,
	pub timestamp: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_OSEvent {
	pub r#type: u32,
	pub timestamp: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_UserEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub windowID: u32,
	pub code: i32,
	pub data1: *mut c_void,
	pub data2: *mut c_void,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDL_SysWMEvent {
	pub r#type: u32,
	pub timestamp: u32,
	pub msg: *mut SDL_SysWMmsg,
}

#[repr(C)]
pub union SDL_Event {
	pub r#type: u32,
	pub common: SDL_CommonEvent,
	pub display: SDL_DisplayEvent,
	pub window: SDL_WindowEvent,
	pub key: SDL_KeyboardEvent,
	pub edit: SDL_TextEditingEvent,
	pub editExt: SDL_TextEditingExtEvent,
	pub text: SDL_TextInputEvent,
	pub motion: SDL_MouseMotionEvent,
	pub button: SDL_MouseButtonEvent,
	pub wheel: SDL_MouseWheelEvent,
	pub jaxis: SDL_JoyAxisEvent,
	pub jball: SDL_JoyBallEvent,
	pub jhat: SDL_JoyHatEvent,
	pub jbutton: SDL_JoyButtonEvent,
	pub jdevice: SDL_JoyDeviceEvent,
	pub jbattery: SDL_JoyBatteryEvent,
	pub caxis: SDL_ControllerAxisEvent,
	pub cbutton: SDL_ControllerButtonEvent,
	pub cdevice: SDL_ControllerDeviceEvent,
	pub ctouchpad: SDL_ControllerTouchpadEvent,
	pub csensor: SDL_ControllerSensorEvent,
	pub adevice: SDL_AudioDeviceEvent,
	pub sensor: SDL_SensorEvent,
	pub quit: SDL_QuitEvent,
	pub user: SDL_UserEvent,
	pub syswm: SDL_SysWMEvent,
	pub tfinger: SDL_TouchFingerEvent,
	pub mgesture: SDL_MultiGestureEvent,
	pub dgesture: SDL_DollarGestureEvent,
	pub r#drop: SDL_DropEvent,
}

#[repr(C)]
pub struct SDL_AudioSpec {
	pub freq: c_int,
	pub format: SDL_AudioFormat,
	pub channels: u8,
	pub silence: u8,
	pub samples: u16,
	pub size: u32,
	pub callback: SDL_AudioCallback,
	pub userdata: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_Color {
	r: u8,
	g: u8,
	b: u8,
	a: u8,
}

pub type SDL_Colour = SDL_Color;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SDL_Rect {
	x: c_int,
	y: c_int,
	w: c_int,
	h: c_int,
}

#[repr(C)]
pub struct SDL_Palette {
	ncolors: c_int,
	colors: *mut SDL_Color,
	version: u32,
	refcount: c_int,
}

#[repr(C)]
pub struct SDL_PixelFormat {
	pub format: u32,
	palette: *mut SDL_Palette,
	pub BitsPerPixel: u8,
	pub BytesPerPixel: u8,
	pub padding: [u8; 2],
	pub Rmask: u32,
	pub Gmask: u32,
	pub Bmask: u32,
	pub Amask: u32,
	pub Rloss: u8,
	pub Gloss: u8,
	pub Bloss: u8,
	pub Aloss: u8,
	pub Rshift: u8,
	pub Gshift: u8,
	pub Bshift: u8,
	pub Ashift: u8,
	pub refcount: c_int,
	next: *mut SDL_PixelFormat,
}

impl SDL_PixelFormat {
	fn palette(&self) -> &SDL_Palette {
		// SAFETY: this should be a valid pointer as long as self is a valid SDL_PixelFormat
		unsafe { &*self.palette }
	}
	fn next(&self) -> Option<&SDL_PixelFormat> {
		if self.next.is_null() {
			None
		} else {
			// SAFETY: this should be a valid pointer as long as self is a valid SDL_PixelFormat
			Some(unsafe { &*self.next })
		}
	}
}

#[repr(C)]
pub struct SDL_Surface {
	flags: u32,
	format: *mut SDL_PixelFormat,
	w: c_int,
	h: c_int,
	pitch: c_int,
	pub pixels: *mut c_void,
	pub userdata: *mut c_void,
	locked: c_int,
	list_blitmap: *mut c_void,
	clip_rect: SDL_Rect,
	map: *mut SDL_BlitMap,
	refcount: c_int,
}

impl SDL_Surface {
	pub fn flags(&self) -> u32 {
		self.flags
	}
	pub fn width(&self) -> i32 {
		assert!(self.w >= 0);
		self.w
	}
	pub fn height(&self) -> i32 {
		assert!(self.h >= 0);
		self.h
	}
	pub fn pitch(&self) -> i32 {
		self.pitch
	}
	pub fn locked(&self) -> i32 {
		self.locked
	}
	pub fn pixel_format(&self) -> &SDL_PixelFormat {
		// SAFETY: this should be a valid pointer as long as self is a valid SDL_Surface
		unsafe { &*self.format }
	}
	pub fn clip_rect(&self) -> SDL_Rect {
		self.clip_rect
	}
	pub fn size(&self) -> (i32, i32) {
		(self.width(), self.height())
	}
}

impl SDL_AudioSpec {
	pub fn new(
		callback: SDL_AudioCallback,
		userdata: *mut c_void,
		channels: u8,
		format: u16,
		freq: c_int,
		samples: u16,
	) -> Self {
		Self {
			callback,
			userdata,
			channels,
			format,
			freq,
			samples,
			silence: 0,
			size: 0,
		}
	}
}

#[link(name = "SDL2", kind = "dylib")]
extern "C" {
	fn SDL_Init(flags: u32) -> c_int;
	fn SDL_SetMainReady();
	fn SDL_CreateWindow(
		title: *const c_char,
		x: c_int,
		y: c_int,
		w: c_int,
		h: c_int,
		flags: u32,
	) -> *mut SDL_Window;
	fn SDL_ShowWindow(window: *mut SDL_Window);
	fn SDL_ShowSimpleMessageBox(
		flags: u32,
		title: *const c_char,
		message: *const c_char,
		window: *mut SDL_Window,
	);
	fn SDL_DestroyWindow(window: *mut SDL_Window);
	fn SDL_GetWindowSize(window: *mut SDL_Window, w: *mut c_int, h: *mut c_int);
	fn SDL_GetWindowID(window: *mut SDL_Window) -> u32;
	fn SDL_SetWindowResizable(window: *mut SDL_Window, resizable: SDL_bool);
	fn SDL_SetWindowSize(window: *mut SDL_Window, w: c_int, h: c_int);
	fn SDL_SetRelativeMouseMode(enabled: SDL_bool) -> c_int;
	fn SDL_GetError() -> *const c_char;
	fn SDL_SetWindowIcon(window: *mut SDL_Window, icon: *mut SDL_Surface);
	fn SDL_SetHint(name: *const c_char, value: *const c_char) -> SDL_bool;
	fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: c_int);
	fn SDL_GL_SetSwapInterval(interval: c_int) -> c_int;
	fn SDL_GL_CreateContext(window: *mut SDL_Window) -> SDL_GLContext;
	fn SDL_GL_DeleteContext(ctx: SDL_GLContext);
	fn SDL_GL_MakeCurrent(window: *mut SDL_Window, context: SDL_GLContext) -> c_int;
	fn SDL_GL_SwapWindow(window: *mut SDL_Window);
	fn SDL_GL_GetProcAddress(proc: *const c_char) -> *mut c_void;
	fn SDL_PollEvent(event: *mut SDL_Event) -> c_int;
	fn SDL_GetMouseState(x: *mut c_int, y: *mut c_int) -> u32;
	fn SDL_GetKeyboardState(numkeys: *mut c_int) -> *const u8;
	fn SDL_GetKeyFromScancode(scancode: SDL_Scancode) -> SDL_Keycode;
	// NOTE: do NOT add SDL_GetScancodeFromKey !!! see get_scancodes_from_key for explanation.
	fn SDL_OpenAudioDevice(
		device: *const c_char,
		iscapture: c_int,
		desired: *const SDL_AudioSpec,
		obtained: *mut SDL_AudioSpec,
		allowed_changes: c_int,
	) -> SDL_AudioDeviceID;
	fn SDL_PauseAudioDevice(dev: SDL_AudioDeviceID, pause_on: c_int);
	fn SDL_CloseAudioDevice(dev: SDL_AudioDeviceID);
	fn SDL_GetClipboardText() -> *mut c_char;
	fn SDL_SetClipboardText(text: *const c_char) -> c_int;
	fn SDL_SetWindowFullscreen(window: *mut SDL_Window, flags: u32) -> c_int;
	fn SDL_LoadBMP(file: *const c_char) -> *mut SDL_Surface;
	fn SDL_free(mem: *mut c_void);
	fn SDL_FreeSurface(surface: *mut SDL_Surface);
	fn SDL_RWFromFile(file: *const c_char, mode: *const c_char) -> *mut SDL_RWops;
	fn SDL_LoadBMP_RW(src: *mut SDL_RWops, freesrc: c_int) -> *mut SDL_Surface;
}

pub mod scancode {
	use crate::sdl::SDL_Scancode;
	pub const UNKNOWN: SDL_Scancode = 0;
	pub const A: SDL_Scancode = 4;
	pub const B: SDL_Scancode = 5;
	pub const C: SDL_Scancode = 6;
	pub const D: SDL_Scancode = 7;
	pub const E: SDL_Scancode = 8;
	pub const F: SDL_Scancode = 9;
	pub const G: SDL_Scancode = 10;
	pub const H: SDL_Scancode = 11;
	pub const I: SDL_Scancode = 12;
	pub const J: SDL_Scancode = 13;
	pub const K: SDL_Scancode = 14;
	pub const L: SDL_Scancode = 15;
	pub const M: SDL_Scancode = 16;
	pub const N: SDL_Scancode = 17;
	pub const O: SDL_Scancode = 18;
	pub const P: SDL_Scancode = 19;
	pub const Q: SDL_Scancode = 20;
	pub const R: SDL_Scancode = 21;
	pub const S: SDL_Scancode = 22;
	pub const T: SDL_Scancode = 23;
	pub const U: SDL_Scancode = 24;
	pub const V: SDL_Scancode = 25;
	pub const W: SDL_Scancode = 26;
	pub const X: SDL_Scancode = 27;
	pub const Y: SDL_Scancode = 28;
	pub const Z: SDL_Scancode = 29;
	pub const N1: SDL_Scancode = 30;
	pub const N2: SDL_Scancode = 31;
	pub const N3: SDL_Scancode = 32;
	pub const N4: SDL_Scancode = 33;
	pub const N5: SDL_Scancode = 34;
	pub const N6: SDL_Scancode = 35;
	pub const N7: SDL_Scancode = 36;
	pub const N8: SDL_Scancode = 37;
	pub const N9: SDL_Scancode = 38;
	pub const N0: SDL_Scancode = 39;
	pub const RETURN: SDL_Scancode = 40;
	pub const ESCAPE: SDL_Scancode = 41;
	pub const BACKSPACE: SDL_Scancode = 42;
	pub const TAB: SDL_Scancode = 43;
	pub const SPACE: SDL_Scancode = 44;
	pub const MINUS: SDL_Scancode = 45;
	pub const EQUALS: SDL_Scancode = 46;
	pub const LEFTBRACKET: SDL_Scancode = 47;
	pub const RIGHTBRACKET: SDL_Scancode = 48;
	pub const BACKSLASH: SDL_Scancode = 49;
	pub const NONUSHASH: SDL_Scancode = 50;
	pub const SEMICOLON: SDL_Scancode = 51;
	pub const APOSTROPHE: SDL_Scancode = 52;
	pub const GRAVE: SDL_Scancode = 53;
	pub const COMMA: SDL_Scancode = 54;
	pub const PERIOD: SDL_Scancode = 55;
	pub const SLASH: SDL_Scancode = 56;
	pub const CAPSLOCK: SDL_Scancode = 57;
	pub const F1: SDL_Scancode = 58;
	pub const F2: SDL_Scancode = 59;
	pub const F3: SDL_Scancode = 60;
	pub const F4: SDL_Scancode = 61;
	pub const F5: SDL_Scancode = 62;
	pub const F6: SDL_Scancode = 63;
	pub const F7: SDL_Scancode = 64;
	pub const F8: SDL_Scancode = 65;
	pub const F9: SDL_Scancode = 66;
	pub const F10: SDL_Scancode = 67;
	pub const F11: SDL_Scancode = 68;
	pub const F12: SDL_Scancode = 69;
	pub const PRINTSCREEN: SDL_Scancode = 70;
	pub const SCROLLLOCK: SDL_Scancode = 71;
	pub const PAUSE: SDL_Scancode = 72;
	pub const INSERT: SDL_Scancode = 73;
	pub const HOME: SDL_Scancode = 74;
	pub const PAGEUP: SDL_Scancode = 75;
	pub const DELETE: SDL_Scancode = 76;
	pub const END: SDL_Scancode = 77;
	pub const PAGEDOWN: SDL_Scancode = 78;
	pub const RIGHT: SDL_Scancode = 79;
	pub const LEFT: SDL_Scancode = 80;
	pub const DOWN: SDL_Scancode = 81;
	pub const UP: SDL_Scancode = 82;
	pub const NUMLOCKCLEAR: SDL_Scancode = 83;
	pub const KP_DIVIDE: SDL_Scancode = 84;
	pub const KP_MULTIPLY: SDL_Scancode = 85;
	pub const KP_MINUS: SDL_Scancode = 86;
	pub const KP_PLUS: SDL_Scancode = 87;
	pub const KP_ENTER: SDL_Scancode = 88;
	pub const KP_1: SDL_Scancode = 89;
	pub const KP_2: SDL_Scancode = 90;
	pub const KP_3: SDL_Scancode = 91;
	pub const KP_4: SDL_Scancode = 92;
	pub const KP_5: SDL_Scancode = 93;
	pub const KP_6: SDL_Scancode = 94;
	pub const KP_7: SDL_Scancode = 95;
	pub const KP_8: SDL_Scancode = 96;
	pub const KP_9: SDL_Scancode = 97;
	pub const KP_0: SDL_Scancode = 98;
	pub const KP_PERIOD: SDL_Scancode = 99;
	pub const NONUSBACKSLASH: SDL_Scancode = 100;
	pub const APPLICATION: SDL_Scancode = 101;
	pub const POWER: SDL_Scancode = 102;
	pub const KP_EQUALS: SDL_Scancode = 103;
	pub const F13: SDL_Scancode = 104;
	pub const F14: SDL_Scancode = 105;
	pub const F15: SDL_Scancode = 106;
	pub const F16: SDL_Scancode = 107;
	pub const F17: SDL_Scancode = 108;
	pub const F18: SDL_Scancode = 109;
	pub const F19: SDL_Scancode = 110;
	pub const F20: SDL_Scancode = 111;
	pub const F21: SDL_Scancode = 112;
	pub const F22: SDL_Scancode = 113;
	pub const F23: SDL_Scancode = 114;
	pub const F24: SDL_Scancode = 115;
	pub const EXECUTE: SDL_Scancode = 116;
	pub const HELP: SDL_Scancode = 117;
	pub const MENU: SDL_Scancode = 118;
	pub const SELECT: SDL_Scancode = 119;
	pub const STOP: SDL_Scancode = 120;
	pub const AGAIN: SDL_Scancode = 121;
	pub const UNDO: SDL_Scancode = 122;
	pub const CUT: SDL_Scancode = 123;
	pub const COPY: SDL_Scancode = 124;
	pub const PASTE: SDL_Scancode = 125;
	pub const FIND: SDL_Scancode = 126;
	pub const MUTE: SDL_Scancode = 127;
	pub const VOLUMEUP: SDL_Scancode = 128;
	pub const VOLUMEDOWN: SDL_Scancode = 129;
	pub const KP_COMMA: SDL_Scancode = 133;
	pub const KP_EQUALSAS400: SDL_Scancode = 134;
	pub const INTERNATIONAL1: SDL_Scancode = 135;
	pub const INTERNATIONAL2: SDL_Scancode = 136;
	pub const INTERNATIONAL3: SDL_Scancode = 137;
	pub const INTERNATIONAL4: SDL_Scancode = 138;
	pub const INTERNATIONAL5: SDL_Scancode = 139;
	pub const INTERNATIONAL6: SDL_Scancode = 140;
	pub const INTERNATIONAL7: SDL_Scancode = 141;
	pub const INTERNATIONAL8: SDL_Scancode = 142;
	pub const INTERNATIONAL9: SDL_Scancode = 143;
	pub const LANG1: SDL_Scancode = 144;
	pub const LANG2: SDL_Scancode = 145;
	pub const LANG3: SDL_Scancode = 146;
	pub const LANG4: SDL_Scancode = 147;
	pub const LANG5: SDL_Scancode = 148;
	pub const LANG6: SDL_Scancode = 149;
	pub const LANG7: SDL_Scancode = 150;
	pub const LANG8: SDL_Scancode = 151;
	pub const LANG9: SDL_Scancode = 152;
	pub const ALTERASE: SDL_Scancode = 153;
	pub const SYSREQ: SDL_Scancode = 154;
	pub const CANCEL: SDL_Scancode = 155;
	pub const CLEAR: SDL_Scancode = 156;
	pub const PRIOR: SDL_Scancode = 157;
	pub const RETURN2: SDL_Scancode = 158;
	pub const SEPARATOR: SDL_Scancode = 159;
	pub const OUT: SDL_Scancode = 160;
	pub const OPER: SDL_Scancode = 161;
	pub const CLEARAGAIN: SDL_Scancode = 162;
	pub const CRSEL: SDL_Scancode = 163;
	pub const EXSEL: SDL_Scancode = 164;
	pub const KP_00: SDL_Scancode = 176;
	pub const KP_000: SDL_Scancode = 177;
	pub const THOUSANDSSEPARATOR: SDL_Scancode = 178;
	pub const DECIMALSEPARATOR: SDL_Scancode = 179;
	pub const CURRENCYUNIT: SDL_Scancode = 180;
	pub const CURRENCYSUBUNIT: SDL_Scancode = 181;
	pub const KP_LEFTPAREN: SDL_Scancode = 182;
	pub const KP_RIGHTPAREN: SDL_Scancode = 183;
	pub const KP_LEFTBRACE: SDL_Scancode = 184;
	pub const KP_RIGHTBRACE: SDL_Scancode = 185;
	pub const KP_TAB: SDL_Scancode = 186;
	pub const KP_BACKSPACE: SDL_Scancode = 187;
	pub const KP_A: SDL_Scancode = 188;
	pub const KP_B: SDL_Scancode = 189;
	pub const KP_C: SDL_Scancode = 190;
	pub const KP_D: SDL_Scancode = 191;
	pub const KP_E: SDL_Scancode = 192;
	pub const KP_F: SDL_Scancode = 193;
	pub const KP_XOR: SDL_Scancode = 194;
	pub const KP_POWER: SDL_Scancode = 195;
	pub const KP_PERCENT: SDL_Scancode = 196;
	pub const KP_LESS: SDL_Scancode = 197;
	pub const KP_GREATER: SDL_Scancode = 198;
	pub const KP_AMPERSAND: SDL_Scancode = 199;
	pub const KP_DBLAMPERSAND: SDL_Scancode = 200;
	pub const KP_VERTICALBAR: SDL_Scancode = 201;
	pub const KP_DBLVERTICALBAR: SDL_Scancode = 202;
	pub const KP_COLON: SDL_Scancode = 203;
	pub const KP_HASH: SDL_Scancode = 204;
	pub const KP_SPACE: SDL_Scancode = 205;
	pub const KP_AT: SDL_Scancode = 206;
	pub const KP_EXCLAM: SDL_Scancode = 207;
	pub const KP_MEMSTORE: SDL_Scancode = 208;
	pub const KP_MEMRECALL: SDL_Scancode = 209;
	pub const KP_MEMCLEAR: SDL_Scancode = 210;
	pub const KP_MEMADD: SDL_Scancode = 211;
	pub const KP_MEMSUBTRACT: SDL_Scancode = 212;
	pub const KP_MEMMULTIPLY: SDL_Scancode = 213;
	pub const KP_MEMDIVIDE: SDL_Scancode = 214;
	pub const KP_PLUSMINUS: SDL_Scancode = 215;
	pub const KP_CLEAR: SDL_Scancode = 216;
	pub const KP_CLEARENTRY: SDL_Scancode = 217;
	pub const KP_BINARY: SDL_Scancode = 218;
	pub const KP_OCTAL: SDL_Scancode = 219;
	pub const KP_DECIMAL: SDL_Scancode = 220;
	pub const KP_HEXADECIMAL: SDL_Scancode = 221;
	pub const LCTRL: SDL_Scancode = 224;
	pub const LSHIFT: SDL_Scancode = 225;
	pub const LALT: SDL_Scancode = 226;
	pub const LGUI: SDL_Scancode = 227;
	pub const RCTRL: SDL_Scancode = 228;
	pub const RSHIFT: SDL_Scancode = 229;
	pub const RALT: SDL_Scancode = 230;
	pub const RGUI: SDL_Scancode = 231;
	pub const MODE: SDL_Scancode = 257;
	pub const AUDIONEXT: SDL_Scancode = 258;
	pub const AUDIOPREV: SDL_Scancode = 259;
	pub const AUDIOSTOP: SDL_Scancode = 260;
	pub const AUDIOPLAY: SDL_Scancode = 261;
	pub const AUDIOMUTE: SDL_Scancode = 262;
	pub const MEDIASELECT: SDL_Scancode = 263;
	pub const WWW: SDL_Scancode = 264;
	pub const MAIL: SDL_Scancode = 265;
	pub const CALCULATOR: SDL_Scancode = 266;
	pub const COMPUTER: SDL_Scancode = 267;
	pub const AC_SEARCH: SDL_Scancode = 268;
	pub const AC_HOME: SDL_Scancode = 269;
	pub const AC_BACK: SDL_Scancode = 270;
	pub const AC_FORWARD: SDL_Scancode = 271;
	pub const AC_STOP: SDL_Scancode = 272;
	pub const AC_REFRESH: SDL_Scancode = 273;
	pub const AC_BOOKMARKS: SDL_Scancode = 274;
	pub const BRIGHTNESSDOWN: SDL_Scancode = 275;
	pub const BRIGHTNESSUP: SDL_Scancode = 276;
	pub const DISPLAYSWITCH: SDL_Scancode = 277;
	pub const KBDILLUMTOGGLE: SDL_Scancode = 278;
	pub const KBDILLUMDOWN: SDL_Scancode = 279;
	pub const KBDILLUMUP: SDL_Scancode = 280;
	pub const EJECT: SDL_Scancode = 281;
	pub const SLEEP: SDL_Scancode = 282;
	pub const APP1: SDL_Scancode = 283;
	pub const APP2: SDL_Scancode = 284;
	pub const AUDIOREWIND: SDL_Scancode = 285;
	pub const AUDIOFASTFORWARD: SDL_Scancode = 286;
	pub const SOFTLEFT: SDL_Scancode = 287;
	pub const SOFTRIGHT: SDL_Scancode = 288;
	pub const CALL: SDL_Scancode = 289;
	pub const ENDCALL: SDL_Scancode = 290;
	pub const NUM_SCANCODES: SDL_Scancode = 512;
}

pub mod keycode {
	use super::scancode;
	use super::{SDL_Keycode, SDL_Scancode};
	const fn SDL_SCANCODE_TO_KEYCODE(scancode: SDL_Scancode) -> SDL_Keycode {
		scancode as SDL_Keycode | 1 << 30
	}

	pub const UNKNOWN: SDL_Keycode = 0;
	pub const RETURN: SDL_Keycode = '\r' as _;
	pub const ESCAPE: SDL_Keycode = '\x1b' as _;
	pub const BACKSPACE: SDL_Keycode = '\x08' as _;
	pub const TAB: SDL_Keycode = '\t' as _;
	pub const SPACE: SDL_Keycode = ' ' as _;
	pub const EXCLAIM: SDL_Keycode = '!' as _;
	pub const QUOTEDBL: SDL_Keycode = '"' as _;
	pub const HASH: SDL_Keycode = '#' as _;
	pub const PERCENT: SDL_Keycode = '%' as _;
	pub const DOLLAR: SDL_Keycode = '$' as _;
	pub const AMPERSAND: SDL_Keycode = '&' as _;
	pub const QUOTE: SDL_Keycode = '\'' as _;
	pub const LEFTPAREN: SDL_Keycode = '(' as _;
	pub const RIGHTPAREN: SDL_Keycode = ')' as _;
	pub const ASTERISK: SDL_Keycode = '*' as _;
	pub const PLUS: SDL_Keycode = '+' as _;
	pub const COMMA: SDL_Keycode = ',' as _;
	pub const MINUS: SDL_Keycode = '-' as _;
	pub const PERIOD: SDL_Keycode = '.' as _;
	pub const SLASH: SDL_Keycode = '/' as _;
	pub const N0: SDL_Keycode = '0' as _;
	pub const N1: SDL_Keycode = '1' as _;
	pub const N2: SDL_Keycode = '2' as _;
	pub const N3: SDL_Keycode = '3' as _;
	pub const N4: SDL_Keycode = '4' as _;
	pub const N5: SDL_Keycode = '5' as _;
	pub const N6: SDL_Keycode = '6' as _;
	pub const N7: SDL_Keycode = '7' as _;
	pub const N8: SDL_Keycode = '8' as _;
	pub const N9: SDL_Keycode = '9' as _;
	pub const COLON: SDL_Keycode = ':' as _;
	pub const SEMICOLON: SDL_Keycode = ';' as _;
	pub const LESS: SDL_Keycode = '<' as _;
	pub const EQUALS: SDL_Keycode = '=' as _;
	pub const GREATER: SDL_Keycode = '>' as _;
	pub const QUESTION: SDL_Keycode = '?' as _;
	pub const AT: SDL_Keycode = '@' as _;
	pub const LEFTBRACKET: SDL_Keycode = '[' as _;
	pub const BACKSLASH: SDL_Keycode = '\\' as _;
	pub const RIGHTBRACKET: SDL_Keycode = ']' as _;
	pub const CARET: SDL_Keycode = '^' as _;
	pub const UNDERSCORE: SDL_Keycode = '_' as _;
	pub const BACKQUOTE: SDL_Keycode = '`' as _;
	pub const A: SDL_Keycode = 'a' as _;
	pub const B: SDL_Keycode = 'b' as _;
	pub const C: SDL_Keycode = 'c' as _;
	pub const D: SDL_Keycode = 'd' as _;
	pub const E: SDL_Keycode = 'e' as _;
	pub const F: SDL_Keycode = 'f' as _;
	pub const G: SDL_Keycode = 'g' as _;
	pub const H: SDL_Keycode = 'h' as _;
	pub const I: SDL_Keycode = 'i' as _;
	pub const J: SDL_Keycode = 'j' as _;
	pub const K: SDL_Keycode = 'k' as _;
	pub const L: SDL_Keycode = 'l' as _;
	pub const M: SDL_Keycode = 'm' as _;
	pub const N: SDL_Keycode = 'n' as _;
	pub const O: SDL_Keycode = 'o' as _;
	pub const P: SDL_Keycode = 'p' as _;
	pub const Q: SDL_Keycode = 'q' as _;
	pub const R: SDL_Keycode = 'r' as _;
	pub const S: SDL_Keycode = 's' as _;
	pub const T: SDL_Keycode = 't' as _;
	pub const U: SDL_Keycode = 'u' as _;
	pub const V: SDL_Keycode = 'v' as _;
	pub const W: SDL_Keycode = 'w' as _;
	pub const X: SDL_Keycode = 'x' as _;
	pub const Y: SDL_Keycode = 'y' as _;
	pub const Z: SDL_Keycode = 'z' as _;
	pub const CAPSLOCK: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::CAPSLOCK);
	pub const F1: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F1);
	pub const F2: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F2);
	pub const F3: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F3);
	pub const F4: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F4);
	pub const F5: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F5);
	pub const F6: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F6);
	pub const F7: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F7);
	pub const F8: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F8);
	pub const F9: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F9);
	pub const F10: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F10);
	pub const F11: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F11);
	pub const F12: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F12);
	pub const PRINTSCREEN: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::PRINTSCREEN);
	pub const SCROLLLOCK: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::SCROLLLOCK);
	pub const PAUSE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::PAUSE);
	pub const INSERT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::INSERT);
	pub const HOME: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::HOME);
	pub const PAGEUP: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::PAGEUP);
	pub const DELETE: SDL_Keycode = '\x7f' as _;
	pub const END: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::END);
	pub const PAGEDOWN: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::PAGEDOWN);
	pub const RIGHT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::RIGHT);
	pub const LEFT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::LEFT);
	pub const DOWN: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::DOWN);
	pub const UP: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::UP);
	pub const NUMLOCKCLEAR: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::NUMLOCKCLEAR);
	pub const KP_DIVIDE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_DIVIDE);
	pub const KP_MULTIPLY: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_MULTIPLY);
	pub const KP_MINUS: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_MINUS);
	pub const KP_PLUS: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_PLUS);
	pub const KP_ENTER: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_ENTER);
	pub const KP_1: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_1);
	pub const KP_2: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_2);
	pub const KP_3: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_3);
	pub const KP_4: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_4);
	pub const KP_5: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_5);
	pub const KP_6: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_6);
	pub const KP_7: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_7);
	pub const KP_8: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_8);
	pub const KP_9: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_9);
	pub const KP_0: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_0);
	pub const KP_PERIOD: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_PERIOD);
	pub const APPLICATION: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::APPLICATION);
	pub const POWER: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::POWER);
	pub const KP_EQUALS: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_EQUALS);
	pub const F13: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F13);
	pub const F14: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F14);
	pub const F15: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F15);
	pub const F16: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F16);
	pub const F17: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F17);
	pub const F18: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F18);
	pub const F19: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F19);
	pub const F20: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F20);
	pub const F21: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F21);
	pub const F22: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F22);
	pub const F23: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F23);
	pub const F24: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::F24);
	pub const EXECUTE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::EXECUTE);
	pub const HELP: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::HELP);
	pub const MENU: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::MENU);
	pub const SELECT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::SELECT);
	pub const STOP: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::STOP);
	pub const AGAIN: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AGAIN);
	pub const UNDO: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::UNDO);
	pub const CUT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::CUT);
	pub const COPY: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::COPY);
	pub const PASTE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::PASTE);
	pub const FIND: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::FIND);
	pub const MUTE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::MUTE);
	pub const VOLUMEUP: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::VOLUMEUP);
	pub const VOLUMEDOWN: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::VOLUMEDOWN);
	pub const KP_COMMA: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_COMMA);
	pub const KP_EQUALSAS400: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_EQUALSAS400);
	pub const ALTERASE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::ALTERASE);
	pub const SYSREQ: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::SYSREQ);
	pub const CANCEL: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::CANCEL);
	pub const CLEAR: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::CLEAR);
	pub const PRIOR: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::PRIOR);
	pub const RETURN2: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::RETURN2);
	pub const SEPARATOR: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::SEPARATOR);
	pub const OUT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::OUT);
	pub const OPER: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::OPER);
	pub const CLEARAGAIN: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::CLEARAGAIN);
	pub const CRSEL: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::CRSEL);
	pub const EXSEL: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::EXSEL);
	pub const KP_00: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_00);
	pub const KP_000: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_000);
	pub const THOUSANDSSEPARATOR: SDL_Keycode =
		SDL_SCANCODE_TO_KEYCODE(scancode::THOUSANDSSEPARATOR);
	pub const DECIMALSEPARATOR: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::DECIMALSEPARATOR);
	pub const CURRENCYUNIT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::CURRENCYUNIT);
	pub const CURRENCYSUBUNIT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::CURRENCYSUBUNIT);
	pub const KP_LEFTPAREN: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_LEFTPAREN);
	pub const KP_RIGHTPAREN: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_RIGHTPAREN);
	pub const KP_LEFTBRACE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_LEFTBRACE);
	pub const KP_RIGHTBRACE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_RIGHTBRACE);
	pub const KP_TAB: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_TAB);
	pub const KP_BACKSPACE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_BACKSPACE);
	pub const KP_A: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_A);
	pub const KP_B: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_B);
	pub const KP_C: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_C);
	pub const KP_D: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_D);
	pub const KP_E: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_E);
	pub const KP_F: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_F);
	pub const KP_XOR: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_XOR);
	pub const KP_POWER: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_POWER);
	pub const KP_PERCENT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_PERCENT);
	pub const KP_LESS: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_LESS);
	pub const KP_GREATER: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_GREATER);
	pub const KP_AMPERSAND: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_AMPERSAND);
	pub const KP_DBLAMPERSAND: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_DBLAMPERSAND);
	pub const KP_VERTICALBAR: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_VERTICALBAR);
	pub const KP_DBLVERTICALBAR: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_DBLVERTICALBAR);
	pub const KP_COLON: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_COLON);
	pub const KP_HASH: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_HASH);
	pub const KP_SPACE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_SPACE);
	pub const KP_AT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_AT);
	pub const KP_EXCLAM: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_EXCLAM);
	pub const KP_MEMSTORE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_MEMSTORE);
	pub const KP_MEMRECALL: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_MEMRECALL);
	pub const KP_MEMCLEAR: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_MEMCLEAR);
	pub const KP_MEMADD: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_MEMADD);
	pub const KP_MEMSUBTRACT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_MEMSUBTRACT);
	pub const KP_MEMMULTIPLY: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_MEMMULTIPLY);
	pub const KP_MEMDIVIDE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_MEMDIVIDE);
	pub const KP_PLUSMINUS: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_PLUSMINUS);
	pub const KP_CLEAR: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_CLEAR);
	pub const KP_CLEARENTRY: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_CLEARENTRY);
	pub const KP_BINARY: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_BINARY);
	pub const KP_OCTAL: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_OCTAL);
	pub const KP_DECIMAL: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_DECIMAL);
	pub const KP_HEXADECIMAL: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KP_HEXADECIMAL);
	pub const LCTRL: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::LCTRL);
	pub const LSHIFT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::LSHIFT);
	pub const LALT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::LALT);
	pub const LGUI: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::LGUI);
	pub const RCTRL: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::RCTRL);
	pub const RSHIFT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::RSHIFT);
	pub const RALT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::RALT);
	pub const RGUI: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::RGUI);
	pub const MODE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::MODE);
	pub const AUDIONEXT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AUDIONEXT);
	pub const AUDIOPREV: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AUDIOPREV);
	pub const AUDIOSTOP: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AUDIOSTOP);
	pub const AUDIOPLAY: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AUDIOPLAY);
	pub const AUDIOMUTE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AUDIOMUTE);
	pub const MEDIASELECT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::MEDIASELECT);
	pub const WWW: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::WWW);
	pub const MAIL: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::MAIL);
	pub const CALCULATOR: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::CALCULATOR);
	pub const COMPUTER: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::COMPUTER);
	pub const AC_SEARCH: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AC_SEARCH);
	pub const AC_HOME: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AC_HOME);
	pub const AC_BACK: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AC_BACK);
	pub const AC_FORWARD: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AC_FORWARD);
	pub const AC_STOP: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AC_STOP);
	pub const AC_REFRESH: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AC_REFRESH);
	pub const AC_BOOKMARKS: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AC_BOOKMARKS);
	pub const BRIGHTNESSDOWN: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::BRIGHTNESSDOWN);
	pub const BRIGHTNESSUP: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::BRIGHTNESSUP);
	pub const DISPLAYSWITCH: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::DISPLAYSWITCH);
	pub const KBDILLUMTOGGLE: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KBDILLUMTOGGLE);
	pub const KBDILLUMDOWN: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KBDILLUMDOWN);
	pub const KBDILLUMUP: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::KBDILLUMUP);
	pub const EJECT: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::EJECT);
	pub const SLEEP: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::SLEEP);
	pub const APP1: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::APP1);
	pub const APP2: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::APP2);
	pub const AUDIOREWIND: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AUDIOREWIND);
	pub const AUDIOFASTFORWARD: SDL_Keycode = SDL_SCANCODE_TO_KEYCODE(scancode::AUDIOFASTFORWARD);
}

pub struct Surface {
	ptr: *mut SDL_Surface,
}

impl Surface {
	/// Returns `None` if `ptr` is null.
	/// # Safety
	/// You may only call this function if `ptr` refers to a valid `SDL_Surface`
	/// which can be freed with `SDL_FreeSurface`.
	/// Make sure you only create one `Surface` for any particular surface pointer.
	/// When the `Surface` is dropped, the `SDL_Surface` pointer will be freed.
	pub unsafe fn from_raw(ptr: *mut SDL_Surface) -> Option<Self> {
		if ptr.is_null() {
			None
		} else {
			Some(Self { ptr })
		}
	}

	/// # Safety
	/// It is your responsibility to use the pointer *before* dropping this `Surface`.
	pub unsafe fn get_raw(&self) -> *mut SDL_Surface {
		self.ptr
	}
}

impl Drop for Surface {
	fn drop(&mut self) {
		// SAFETY: this should only be constructed with a valid SDL surface pointer,
		// and the pointer should never be freed by anything else.
		unsafe { SDL_FreeSurface(self.ptr) };
	}
}

unsafe fn get_err() -> String {
	let cstr = CStr::from_ptr(SDL_GetError());
	String::from_utf8_lossy(cstr.to_bytes()).to_string()
}

pub unsafe fn set_main_ready() {
	SDL_SetMainReady();
}

pub unsafe fn create_window(
	title: &str,
	width: i32,
	height: i32,
	flags: u32,
) -> Result<*mut SDL_Window, String> {
	let Ok(tstr) = CString::new(title) else {
		return Err("window title cannot contain null bytes".to_string())
	};
	let window = SDL_CreateWindow(
		tstr.as_ptr(),
		SDL_WINDOWPOS_UNDEFINED,
		SDL_WINDOWPOS_UNDEFINED,
		width,
		height,
		flags,
	);
	if window.is_null() {
		Err(get_err())
	} else {
		Ok(window)
	}
}

pub unsafe fn gl_create_context(window: *mut SDL_Window) -> Result<SDL_GLContext, String> {
	let ctx = SDL_GL_CreateContext(window);
	if ctx.is_null() {
		Err(get_err())
	} else {
		Ok(ctx)
	}
}

// NOTE: the buffer returned by SDL does really have a static lifetime. we're not lying here.
pub unsafe fn get_keyboard_state() -> &'static [u8] {
	let state = SDL_GetKeyboardState(0 as _);
	std::slice::from_raw_parts(state, scancode::NUM_SCANCODES as usize)
}

pub unsafe fn poll_event() -> Option<SDL_Event> {
	let mut event = mem::MaybeUninit::zeroed();

	if SDL_PollEvent(event.as_mut_ptr()) != 0 {
		let event = event.assume_init();
		Some(event)
	} else {
		None
	}
}

pub unsafe fn init() -> Result<(), String> {
	if SDL_Init(SDL_INIT_EVERYTHING) == 0 {
		Ok(())
	} else {
		Err(get_err())
	}
}

pub unsafe fn set_hint(hint: &str, value: &str) {
	// NOTE: hint,value are probably string literals, so cstring is very unlikely to fail
	if let Ok(hstr) = CString::new(hint) {
		if let Ok(vstr) = CString::new(value) {
			SDL_SetHint(hstr.as_ptr(), vstr.as_ptr());
		}
	}
}

pub unsafe fn gl_set_attribute(attr: i32, value: i32) {
	SDL_GL_SetAttribute(attr, value);
}

pub unsafe fn gl_set_context_version(major: i32, minor: i32) {
	SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, major);
	SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, minor);
}

pub unsafe fn gl_get_proc_address(name: &str) -> *const std::os::raw::c_void {
	match CString::new(name) {
		Ok(cstr) => SDL_GL_GetProcAddress(cstr.as_ptr()),
		Err(_) => 0 as _,
	}
}

pub unsafe fn gl_set_swap_interval(interval: i32) {
	SDL_GL_SetSwapInterval(interval);
}

pub unsafe fn show_window(window: *mut SDL_Window) {
	SDL_ShowWindow(window);
}

pub unsafe fn set_relative_mouse_mode(relative: bool) {
	SDL_SetRelativeMouseMode(relative.into());
}

pub unsafe fn gl_swap_window(window: *mut SDL_Window) {
	SDL_GL_SwapWindow(window);
}

pub unsafe fn gl_delete_context(ctx: SDL_GLContext) {
	SDL_GL_DeleteContext(ctx);
}

pub unsafe fn destroy_window(window: *mut SDL_Window) {
	SDL_DestroyWindow(window);
}

pub unsafe fn show_simple_message_box(
	flags: u32,
	title: &str,
	message: &str,
	window: *mut SDL_Window,
) -> Result<(), String> {
	let tstr = CString::new(title).unwrap_or_else(|_| CString::new("bad title").unwrap());
	let mstr = CString::new(message).unwrap_or_else(|_| CString::new("bad message").unwrap());
	SDL_ShowSimpleMessageBox(flags, tstr.as_ptr(), mstr.as_ptr(), window);
	Ok(())
}

pub unsafe fn get_window_size(window: *mut SDL_Window, x: &mut i32, y: &mut i32) {
	SDL_GetWindowSize(window, x as _, y as _);
}

pub unsafe fn open_audio_device(
	device: Option<&str>,
	iscapture: bool,
	desired: &SDL_AudioSpec,
	allowed_changes: c_int,
) -> Result<(SDL_AudioSpec, SDL_AudioDeviceID), String> {
	let mut obtained = mem::MaybeUninit::zeroed();
	let mut devstr = None;
	if let Some(dev) = device {
		devstr = CString::new(dev).ok();
	}

	let devptr: *const c_char = match devstr {
		None => 0 as _,
		Some(s) => s.as_ptr(),
	};
	let id = SDL_OpenAudioDevice(
		devptr,
		iscapture.into(),
		desired as _,
		obtained.as_mut_ptr(),
		allowed_changes,
	);
	if id == 0 {
		return Err(get_err());
	}
	let obtained = obtained.assume_init();
	Ok((obtained, id))
}

pub unsafe fn pause_audio_device(dev: SDL_AudioDeviceID, pause_on: bool) {
	SDL_PauseAudioDevice(dev, pause_on.into());
}

pub unsafe fn close_audio_device(dev: SDL_AudioDeviceID) {
	SDL_CloseAudioDevice(dev);
}

pub unsafe fn get_clipboard_text() -> Result<String, String> {
	let ptr = SDL_GetClipboardText();
	if ptr.is_null() {
		// according to some sources, SDL_GetClipboardText returns NULL on failure.
		// according to the wiki, it returns an empty string.
		return Err(get_err());
	}

	let cstr = CStr::from_ptr(ptr);
	let Ok(r#str) = cstr.to_str() else {
		// this should never happen since SDL should always give back valid UTF-8.
		return Err("clipboard doesn't contain UTF-8".to_string());
	};
	let string = r#str.to_string();
	SDL_free(ptr as *mut c_void);
	Ok(string)
}

pub unsafe fn set_clipboard_text(s: &str) -> Result<(), String> {
	let Ok(cstring) = CString::new(s) else {
		return Err("can't put null bytes in clipboard text.".to_string());
	};

	let result = SDL_SetClipboardText(cstring.as_ptr());
	if result == 0 {
		Ok(())
	} else {
		Err(get_err())
	}
}

pub unsafe fn set_window_fullscreen(win: *mut SDL_Window, flags: u32) -> Result<(), String> {
	let result = SDL_SetWindowFullscreen(win, flags);
	if result == 0 {
		Ok(())
	} else {
		Err(get_err())
	}
}

pub unsafe fn load_bmp(filename: &str) -> Result<Surface, String> {
	let Ok(cstring) = CString::new(filename) else {
		return Err("filename contains null bytes.".to_string());
	};
	// note: this is how SDL_LoadBMP is #defined in SDL_surface.h
	let rwops = SDL_RWFromFile(cstring.as_ptr(), b"rb\0".as_ptr().cast());
	let raw = SDL_LoadBMP_RW(rwops, 1);
	Surface::from_raw(raw).ok_or_else(|| get_err())
}

pub unsafe fn set_window_icon(win: *mut SDL_Window, icon: &Surface) {
	SDL_SetWindowIcon(win, icon.ptr);
}

/// Get all scancodes associated with a keycode.
///
/// `SDL_GetScancodeFromKey` is a bad function! it's possible (though unlikely) for multiple
/// scancodes to map to the same keycode. i've encountered this problem myself
/// with sdl applications (don't ask why).
pub unsafe fn get_scancodes_from_key(keycode: SDL_Keycode) -> impl Iterator<Item = SDL_Scancode> {
	// the current implementation of SDL_GetKeyFromScancode is just an array lookup so this loop
	// should perform well.
	(0..scancode::NUM_SCANCODES).filter(move |&scn| SDL_GetKeyFromScancode(scn) == keycode)
}

pub unsafe fn get_mouse_state() -> (i32, i32, u32) {
	let mut x = 0;
	let mut y = 0;
	let state = SDL_GetMouseState((&mut x) as *mut c_int, (&mut y) as *mut c_int);
	(x as _, y as _, state)
}
