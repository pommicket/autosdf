uniform sampler2D u_main_texture;
uniform sampler2D u_menu_texture;
uniform float u_paused;
uniform float u_aspect_ratio;
uniform float u_menu_scale;
uniform vec2 u_highlight_button;
IN vec2 uv;

void main() {
	// amount to darken screen by when paused
	float pause_darkening = 0.75;
	vec4 color = texture(u_main_texture, uv) * (1.0 - pause_darkening * u_paused);
	vec2 menu_uv = (uv * 2.0 - 1.0) * vec2(1.0, -1.0);
	menu_uv *= 1.0 / u_menu_scale;
	menu_uv = menu_uv * vec2(u_aspect_ratio, 1.0);
	menu_uv = 0.5 * menu_uv + 0.5;
	float menu_pixel = u_paused * texture(u_menu_texture, menu_uv).x;
	vec4 menu_color = vec4(1.0);
	if (uv.y >= u_highlight_button.x && uv.y <= u_highlight_button.y) {
		menu_color = vec4(1.0, 0.5, 0.0, 1.0); // highlight button
	}
	color = mix(color, menu_color, menu_pixel);
	gl_FragColor = clamp(color, 0.0, 1.0);
}
