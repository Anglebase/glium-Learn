#version 330

in float x;
in float y;
in float z;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(x, y, z, 1.0);
}