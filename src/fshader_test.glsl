// used to determine the default level set

%COMMON%
%SDF%

IN vec2 pos;

vec4 rand(vec2 coord) {
	// this seems to give pretty good quality noise for |coord| < 2
	vec4 a = sin((coord.xyyx + vec4(3.0)) * (coord.xyxy + vec4(4.0)) * 100.0 + 0.5832) * 1618.0;
	vec4 b = sin((coord.xyyx + vec4(3.0)) * (coord.xyxy + vec4(4.0)) * 222.0 + 5.1339) * 2412.0;
	return vec4(fract(a.xy + a.zw), fract(b.xy + b.zw));
}

void main() {
	gl_FragColor = vec4(rand(pos));//sdf(8.0 * rand(pos).xyz - 4.0));
}
