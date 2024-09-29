#version 330 core
precision mediump float;

out vec4 vertexColorOut;

in vec3 vertexColorIn;

void main() {
    vertexColorOut = vec4(vertexColorIn, 0.5);
}
