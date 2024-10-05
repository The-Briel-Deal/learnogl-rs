#version 330 core

in vec3 fragColorIn;
in vec2 texCoord;

out vec4 fragColorOut;

uniform sampler2D texture1;
uniform sampler2D texture2;
uniform float textureBlend;

void main() {
    fragColorOut = mix(texture(texture1, texCoord), texture(texture2, vec2(-texCoord[0], texCoord[1])), textureBlend);
}
