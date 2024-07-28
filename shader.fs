#version 330
out vec4 finalColor;
uniform vec2 resolution;

float circle(in vec2 st){
    return length(st);
}

void main(){
    vec2 st = (gl_FragCoord.xy-resolution.xy*.5)/resolution.y;

	vec3 color = vec3(1.0-circle(st));

	finalColor = vec4( st.x, st.y, 0.0, 1.0);
}

