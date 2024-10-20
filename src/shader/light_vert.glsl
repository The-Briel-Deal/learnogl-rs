#version 330 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aNormal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

uniform vec3 lightPos;
uniform float specularStrength;
uniform vec3 lightColor;

out vec3 FragPos;
out vec3 Normal;
out vec3 LightPos;

out vec3 SpecularLighting;

void main()
{
    gl_Position = projection * view * model * vec4(aPos, 1.0);
    FragPos = vec3(view * model * vec4(aPos, 1.0));
    Normal = mat3(transpose(inverse(view * model))) * aNormal;
    LightPos = vec3(view * vec4(lightPos, 1.0));

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(LightPos - FragPos);

    vec3 viewDir = normalize(-FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    int shininess = 128;
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    SpecularLighting = specularStrength * spec * lightColor;
}
