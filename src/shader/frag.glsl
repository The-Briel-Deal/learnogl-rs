#version 330 core

in vec3 fragColorIn;
in vec2 texCoord;

out vec4 fragColorOut;

uniform sampler2D ourTexture;

void main() {
    fragColorOut = texture(ourTexture, texCoord);
}
