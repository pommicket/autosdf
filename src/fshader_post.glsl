uniform sampler2D u_main_texture;
uniform sampler2D u_menu_texture;
uniform float u_paused;
IN vec2 uv;

void main() {
	vec4 color = texture(u_main_texture, uv) * (1.0 - 0.5 * u_paused);
	color += texture(u_menu_texture, uv).xxxx * u_paused;
	gl_FragColor = clamp(color, 0.0, 1.0);
}
