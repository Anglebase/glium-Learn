#version 330

in float x;
in float y;

in vec2 in_tex_coord;
out vec2 tex_coord;

void main() {
    gl_Position = vec4(x, y, 0.0, 1.0);
    tex_coord = in_tex_coord;
}