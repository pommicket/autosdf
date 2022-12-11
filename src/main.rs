mod sdl;
pub mod win;

fn try_main() -> Result<(), String> {
	let mut window = win::Window::new("AutoSDF", 1280, 720, true)
		.map_err(|e| format!("Error creating window: {e}"))?;
	
	let program = window.create_program(
		"attribute vec2 v_pos;
		void main() {
			gl_Position = vec4(v_pos, 0.0, 1.0);
		}",
		"void main() {
			o_color = vec4(1.0, 0.0, 0.0, 1.0);
		}"
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
	
	'mainloop: loop {
		while let Some(event) = window.next_event() {
			use win::Event::*;
			match event {
				Quit => break 'mainloop,
				_ => {},
			}
			println!("{event:?}");
		}
		
		window.clear_screen(win::ColorF32::BLACK);
		window.use_program(&program);
		
		window.draw_array(&array);
		
		window.swap();
	}
	
	Ok(())
}

fn main() {
	if let Err(e) = try_main() {
		win::display_error_message(&e);
	}
}
