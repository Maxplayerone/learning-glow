#version 330
layout (location=0) in vec3 position;
layout (location=1) in vec3 a_color;
layout (location=2) in vec2 a_TexCoords;

out vec3 f_color;
out vec2 f_TexCoords;

uniform mat4 cool_matrix;

void main(){
    f_color = a_color;
    f_TexCoords = a_TexCoords;
    gl_Position = cool_matrix * vec4(position, 1.0);
}