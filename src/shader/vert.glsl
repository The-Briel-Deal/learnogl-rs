#version 330 core
precision mediump float;

uniform float uniColor;
uniform vec2 center;

attribute vec2 position;
attribute vec3 color;

out vec3 vertexColorIn;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    vertexColorIn = vec3(abs(position), 0.0);
}
