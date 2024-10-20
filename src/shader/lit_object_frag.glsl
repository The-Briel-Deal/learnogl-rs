#version 330 core
out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform float ambientLightConstant;
uniform float specularStrength;

in vec3 FragPos;
in vec3 Normal;
in vec3 LightPos;

void main()
{
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(LightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuseLighting = diff * lightColor;

    vec3 viewDir = normalize(-FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);

    int shininess = 128;
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specularLighting = specularStrength * spec * lightColor;

    vec3 ambientLighting = ambientLightConstant * lightColor;

    vec3 resultLighting = (ambientLighting + diffuseLighting + specularLighting) * objectColor;

    FragColor = vec4(resultLighting, 1.0);
}
