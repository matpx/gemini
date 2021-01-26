#version 450

layout(location = 0) in vec3 a_Pos;
layout(location = 1) in vec2 a_UV;
layout(location = 2) in vec3 a_Normal;

layout(location = 0) out vec2 v_UV;
layout(location = 1) out vec3 v_Normal;

layout(set = 0, binding = 0) uniform Globals {
    mat4 u_ViewProj;
};

layout(set = 1, binding = 0) uniform Entity {
    mat4 u_World;
};

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    v_UV = a_UV;
    v_Normal = a_Normal;
    gl_Position = u_ViewProj * u_World * vec4(a_Pos, 1.0);
}