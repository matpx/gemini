#version 450

layout(location = 0) in vec2 a_UV;
layout(location = 1) in vec3 a_Normal;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(a_Normal.x, a_UV.y, 1.0, 1.0);
}