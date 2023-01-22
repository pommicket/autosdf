IN vec2 v_pos;
OUT vec2 pos;

void main() {
	pos = v_pos;
	gl_Position = vec4(v_pos, 0.0, 1.0);
}
