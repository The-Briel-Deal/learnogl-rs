#version 330 core
out vec4 FragColor;
  
uniform vec3 objectColor;
uniform vec3 lightColor;
uniform float ambientLightConstant;

void main()
{
    vec3 ambientLighting = ambientLightConstant * lightColor;

    vec3 resultLighting = ambientLighting * objectColor;

    FragColor = vec4(resultLighting, 1.0);
}
