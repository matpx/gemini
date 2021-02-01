#version 450

layout(location = 0) in vec2 a_UV;
layout(location = 1) in vec3 a_Normal;

layout(location = 0) out vec4 outColor;

layout(set = 2, binding = 0) uniform Entity {
    vec4 u_Color;
};

void main() {
    outColor = vec4(u_Color.xyz, 1.0);
}