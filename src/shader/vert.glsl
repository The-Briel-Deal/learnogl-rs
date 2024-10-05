#version 330 core

attribute vec2 position;
attribute vec3 color;
attribute vec2 textureCoord;

out vec3 fragColorIn;
out vec2 texCoord;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(position, 0.0, 1.0);
    fragColorIn = vec3(abs(position), 0.0);
    texCoord = textureCoord;
}
