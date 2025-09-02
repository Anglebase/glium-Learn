#version 330

in float x;
in float y;
in float z;

in vec2 tex_coord;
out vec2 frag_tex_coord;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(x, y, z, 1.0);
    frag_tex_coord = tex_coord;
}