#version 330
out vec4 out_color;

in vec3 f_color;
in vec2 f_TexCoords;

uniform sampler2D texture0;
uniform sampler2D texture1;

void main() {
    out_color = mix(texture(texture0, f_TexCoords), texture(texture1, f_TexCoords), 0.5);
}