
float smooth_min(float a, float b, float k) {
	k = clamp(k, 0.0, 1.0);
	float h = max(k-abs(a-b), 0.0)/k;
	return min(a, b) - h*h*h*k*(1.0/6.0);
}

// thanks to https://iquilezles.org/articles/distfunctions/

float sdf_box_frame(vec3 p, vec3 b, float e) {
	p = abs(p)-b;
	vec3 q = abs(p+e)-e;
	return min(min(
		length(max(vec3(p.x,q.y,q.z),0.0))+min(max(p.x,max(q.y,q.z)),0.0),
		length(max(vec3(q.x,p.y,q.z),0.0))+min(max(q.x,max(p.y,q.z)),0.0)),
		length(max(vec3(q.x,q.y,p.z),0.0))+min(max(q.x,max(q.y,p.z)),0.0));
}

float sdf_torus(vec3 p, vec2 t) {
	vec2 q = vec2(length(p.xy)-t.x,p.z);
	return length(q)-t.y;
}

float sdf_tri_prism(vec3 p, vec2 h) {
	vec3 q = abs(p);
	return max(q.z-h.y,max(q.x*0.866025+p.y*0.5,-p.y)-h.x*0.5);
}

float sdf_vertical_line_segment(vec3 p, float h) {
	p.y -= clamp(p.y, 0.0, h);
	return length(p);
}

float sdf_cylinder(vec3 p, float h, float r) {
	vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(r,h);
	return min(max(d.x,d.y),0.0) + length(max(d,0.0));
}
