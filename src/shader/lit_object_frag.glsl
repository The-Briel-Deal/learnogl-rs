#version 460 core

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

uniform Material material;
uniform vec3 lightColor;

in vec3 FragPos;
in vec3 Normal;
in vec3 LightPos;

out vec4 FragColor;

vec3 calculateAmbientLighting(vec3 ambientLightConstant, vec3 lightColor);
vec3 calculateDiffuseLighting(vec3 normal, vec3 lightDir, vec3 lightColor, vec3 diffuseConstant);
vec3 calculateSpecularLighting(vec3 normal, vec3 lightDir, vec3 lightColor, vec3 FragPos, vec3 specularStrength, float shininess);

void main()
{
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(LightPos - FragPos);

    vec3 diffuseLighting = calculateDiffuseLighting(norm, lightDir, lightColor, material.diffuse);
    vec3 specularLighting = calculateSpecularLighting(norm, lightDir, lightColor, FragPos, material.specular, material.shininess);
    vec3 ambientLighting = calculateAmbientLighting(material.ambient, lightColor);

    vec3 resultLighting = ambientLighting + diffuseLighting + specularLighting;

    FragColor = vec4(resultLighting, 1.0);
}

vec3 calculateAmbientLighting(vec3 ambientLightConstant, vec3 lightColor) {
    vec3 ambientLighting = ambientLightConstant * lightColor;
    return ambientLighting;
}

vec3 calculateDiffuseLighting(vec3 normal, vec3 lightDir, vec3 lightColor, vec3 diffuseConstant) {
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuseLighting = diff * (lightColor, diffuseConstant);
    return diffuseLighting;
}

vec3 calculateSpecularLighting(vec3 normal, vec3 lightDir, vec3 lightColor, vec3 FragPos, vec3 specularStrength, float shininess) {
    vec3 viewDir = normalize(-FragPos);
    vec3 reflectDir = reflect(-lightDir, normal);
    float angleBetween = dot(viewDir, reflectDir);
    float spec = pow(max(angleBetween, 0.0), shininess);
    vec3 specularLighting = specularStrength * spec * lightColor;
    return specularLighting;
}
