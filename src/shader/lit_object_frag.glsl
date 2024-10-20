#version 460 core

uniform int shininess;
uniform float ambientLightConstant;
uniform float specularStrength;
uniform vec3 objectColor;
uniform vec3 lightColor;

in vec3 FragPos;
in vec3 Normal;
in vec3 LightPos;

out vec4 FragColor;

vec3 calculateDiffuseLighting(vec3 normal, vec3 lightDir, vec3 lightColor);
vec3 calculateSpecularLighting(vec3 normal, vec3 lightDir, vec3 lightColor, vec3 FragPos, float specularStrength, int shininess);
vec3 calculateAmbientLighting(float ambientLightConstant, vec3 lightColor);

void main()
{
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(LightPos - FragPos);

    vec3 diffuseLighting = calculateDiffuseLighting(norm, lightDir, lightColor);
    vec3 specularLighting = calculateSpecularLighting(norm, lightDir, lightColor, FragPos, specularStrength, shininess);
    vec3 ambientLighting = calculateAmbientLighting(ambientLightConstant, lightColor);

    vec3 resultLighting = (ambientLighting + diffuseLighting + specularLighting) * objectColor;

    FragColor = vec4(resultLighting, 1.0);
}

vec3 calculateDiffuseLighting(vec3 normal, vec3 lightDir, vec3 lightColor) {
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 diffuseLighting = diff * lightColor;
    return diffuseLighting;
}

vec3 calculateSpecularLighting(vec3 normal, vec3 lightDir, vec3 lightColor, vec3 FragPos, float specularStrength, int shininess) {
    vec3 viewDir = normalize(-FragPos);
    vec3 reflectDir = reflect(-lightDir, normal);
    float angleBetween = dot(viewDir, reflectDir);
    float spec = pow(max(angleBetween, 0.0), shininess);
    vec3 specularLighting = specularStrength * spec * lightColor;
    return specularLighting;
}

vec3 calculateAmbientLighting(float ambientLightConstant, vec3 lightColor) {
    vec3 ambientLighting = ambientLightConstant * lightColor;
    return ambientLighting;
}
