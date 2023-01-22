uniform sampler2D u_texture;
IN vec2 uv;

void main() {
	gl_FragColor = texture(u_texture, uv);
}
