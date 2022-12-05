#version 330
layout (location=0) in vec3 position;
layout (location=1) in vec3 a_color;

uniform float offset_value;

out vec3 f_color;

void main(){
    f_color = a_color;
    gl_Position = vec4(position.x + offset_value, position.y, position.z, 1.0);
}