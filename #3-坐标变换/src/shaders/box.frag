#version 330

out vec4 color;
in vec2 frag_tex_coord;

uniform sampler2D tex;

void main() {
    color = texture(tex, frag_tex_coord);
}