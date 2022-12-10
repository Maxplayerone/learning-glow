#version 330
out vec4 out_color;

in vec3 f_color;
in vec2 f_TexCoords;

void main() {
    out_color = vec4(f_TexCoords, 1.0, 1.0);
}