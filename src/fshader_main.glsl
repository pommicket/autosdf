// this draws the scene

IN vec2 pos;
uniform mat3 u_rotation;
uniform vec3 u_translation;
uniform float u_time;
uniform float u_fov;
uniform float u_focal_length;
uniform float u_level_set;
uniform int u_hsv;

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

%SDF%
%COLOR%

// see https://en.wikipedia.org/wiki/HSL_and_HSV#HSV_to_RGB_alternative
float hsvf(float n, vec3 hsv) {
	float k = mod(n + hsv.x * 6.0, 6.0);
	return hsv.z - hsv.z * hsv.y * clamp(min(k, 4.0 - k), 0.0, 1.0);
}

vec3 hsv_to_rgb(vec3 hsv) {
	hsv.yz = clamp(hsv.yz, 0.0, 1.0);
	return vec3(hsvf(5.0, hsv), hsvf(3.0, hsv), hsvf(1.0, hsv));
}

vec3 get_color(vec3 p) {
	if (u_hsv != 0) {
		vec3 hsv = get_color_(p);
		// make sure object isn't too dark so we can actually see it
		hsv.z = mix(hsv.z, 1.0, 0.5);
		return hsv_to_rgb(hsv);
	} else {
		// we're not clamping this because it makes a cool glowing effect if we don't
		vec3 color = get_color_(p);
		return mix(color, vec3(1.0), 0.2);
	}
}

#define ITERATIONS 30
#define AA_X 1
#define AA_Y 1


float sdf_adjusted(vec3 p) {
	return sdf(p) - u_level_set;
}
#define sdf sdf_adjusted

vec3 normal(vec3 p)
{
// thanks to https://iquilezles.org/articles/normalsSDF/
    float h = 0.0001;
    vec2 k = vec2(1.,-1.);
    vec3 sdf_normal = k.xyy*sdf(p + k.xyy*h) + 
                      k.yyx*sdf(p + k.yyx*h) + 
                      k.yxy*sdf(p + k.yxy*h) + 
                      k.xxx*sdf(p + k.xxx*h);
    return normalize(sdf_normal);
}

void main() {
	float min_dist = 10.;
	vec2 inv_screen_size = 1.0 / vec2(1280.0, 720.0); // @TODO
	vec2 aa_delta = inv_screen_size / vec2(AA_X, AA_Y);
	vec3 final_color = vec3(0);
	for (int m = 0; m < AA_X; m++) {
	for (int n = 0; n < AA_Y; n++) {
	vec2 aa_offset = vec2(float(m), float(n)) * aa_delta;
	vec3 pos3d = vec3((pos + aa_offset) * sin(u_fov * 0.5), -1.0) * u_focal_length;
	vec3 p = u_rotation * pos3d;
	vec3 delta = normalize(p);
	p += u_translation;
	if (sdf(p) < 0.0) {
		// looking inside object
		o_color = vec4(get_color(p), 1.0);
		return;
	}
	int i;
	for (i = 0; i < ITERATIONS; i++) {
		float dist = sdf(p);
		min_dist = min(min_dist, dist);
		if (dist > 100.0) break;
		p += dist * delta;
	}

	float threshold = 0.02;
	if (min_dist < threshold) {
		vec3 N = normal(p);
		// light direction = towards user
		// this makes it seem like the user is pointing a flashlight at the object.
		vec3 light_direction = u_rotation * vec3(0.0, 0.0, 1.0);
		float L_diffuse = max(0., dot(N, light_direction));
		// Phong lighting
		vec3 R = reflect(light_direction, N);
		vec3 view_direction = u_rotation * vec3(0.0, 0.0, -1.0);
		// wikipedia calls this exponent the "shininess" (α)
		float shininess = 16.0;
		float L_specular = pow(max(0.0, dot(R, view_direction)), shininess);
		float brightness = (1.0/threshold) * (threshold-min_dist);
		brightness = pow(brightness, 16.0);
		float L_ambient = 0.3;
		vec3 color = get_color(p);
		float specularity = 0.15; // strength of specular lighting
		final_color += brightness * mix(mix(L_diffuse, 1.0, L_ambient) * color, vec3(L_specular), specularity);
		break;
	}
	
	}
	}
	final_color *= 1.0 / (AA_X * AA_Y);
	o_color = vec4(final_color, 1.0);
}
