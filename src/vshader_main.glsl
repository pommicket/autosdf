IN vec2 v_pos;
OUT vec2 pos;
uniform float u_aspect_ratio;

void main() {
	pos = v_pos * vec2(u_aspect_ratio, 1.0);
	gl_Position = vec4(v_pos, 0.0, 1.0);
}
