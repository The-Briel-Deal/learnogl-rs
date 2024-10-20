#version 460 core

in vec2 texCoord;

out vec4 fragColorOut;

uniform sampler2D awesomeface;
uniform sampler2D container;
uniform float textureBlend;

void main() {
    fragColorOut = mix(texture(awesomeface, texCoord), texture(container, vec2(-texCoord[0], texCoord[1])), textureBlend);
}
