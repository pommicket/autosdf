
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
