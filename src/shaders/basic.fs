#version 330
out vec4 out_color;

in vec3 f_color;
in vec2 f_TexCoords;

uniform sampler2D texture0;

void main() {
    out_color = (texture(texture0, f_TexCoords) * vec4(f_color, 1.0));
}