#version 330 core
out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform float ambientLightConstant;

in vec3 FragPos;
in vec3 Normal;
in vec3 LightPos;

in vec3 SpecularLighting;

void main()
{
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(LightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuseLighting = diff * lightColor;


    vec3 ambientLighting = ambientLightConstant * lightColor;

    vec3 resultLighting = (ambientLighting + diffuseLighting + SpecularLighting) * objectColor;

    FragColor = vec4(resultLighting, 1.0);
}
