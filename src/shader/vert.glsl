#version 330 core

attribute vec2 position;

out vec3 fragColorIn;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    fragColorIn = vec3(abs(position), 0.0);
}
