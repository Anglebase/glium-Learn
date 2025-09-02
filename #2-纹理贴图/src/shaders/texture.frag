#version 330

uniform sampler2D tex;
out vec4 color;

in vec2 tex_coord;

void main() {
    color = texture(tex, tex_coord);
}