#version 330 core

attribute vec3 position;
attribute vec2 textureCoord;

out vec3 fragColorIn;
out vec2 texCoord;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(position, 1.0);
    texCoord = textureCoord;
}
