uniform sampler2D u_main_texture;
uniform sampler2D u_menu_texture;
uniform float u_paused;
uniform float u_aspect_ratio;
uniform float u_menu_scale;
uniform vec2 u_highlight_button;
IN vec2 uv;
uniform vec4 u_flash;
uniform int u_flash_icon;

#define ICON_COPY 1
#define ICON_PLAY 2
#define ICON_PAUSE 3
#define ICON_REWIND 4
#define ICON_SCREENSHOT 5

bool play_icon(vec2 pos) {
	vec2 a = abs(pos);
	if (a.x >= 0.5 || a.y >= 0.5)
		return false;
	return a.y < 0.25 + 0.5 * pos.x;
}

bool get_icon(vec2 pos) {
	bool icon = false;
	switch (u_flash_icon) {
	case 0: break;
	case ICON_COPY:
	case ICON_SCREENSHOT:
		icon = abs(pos.x) > u_aspect_ratio - 0.1 || abs(pos.y) > 0.9;
		break;
	case ICON_PLAY:
		icon = play_icon(pos);
		break;
	case ICON_REWIND:
		icon = play_icon(vec2(-pos.x, pos.y));
		break;
	case ICON_PAUSE:
		vec2 p = abs(pos);
		icon = p.x >= 0.1 && p.x <= 0.4 && p.y <= 0.5;
		break;
	}
	return icon;
}

void main() {
	vec3 color = texture(u_main_texture, uv).xyz;
	
	if (get_icon((uv * 2.0 - 1.0) * vec2(u_aspect_ratio, 1.0)))
		color = mix(color, u_flash.xyz, u_flash.w);
	
	// amount to darken screen by when paused
	float pause_darkening = 0.5;
	color *= 1.0 - pause_darkening * u_paused;
	vec2 menu_uv = (uv * 2.0 - 1.0) * vec2(1.0, -1.0);
	menu_uv *= 1.0 / u_menu_scale;
	menu_uv = menu_uv * vec2(u_aspect_ratio, 1.0);
	menu_uv = 0.5 * menu_uv + 0.5;
	float menu_pixel = u_paused * texture(u_menu_texture, menu_uv).x;
	vec3 menu_color = vec3(1.0);
	if (uv.y >= u_highlight_button.x && uv.y <= u_highlight_button.y) {
		menu_color = vec3(1.0, 0.5, 0.0); // highlight button
	}
	color = mix(color, menu_color, menu_pixel);
	gl_FragColor = vec4(clamp(color, 0.0, 1.0), 1.0);
}
