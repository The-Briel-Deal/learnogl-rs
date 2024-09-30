#version 330 core

in vec3 fragColorIn;

out vec4 fragColorOut;

void main() {
    fragColorOut = vec4(fragColorIn, 1.0);
}
