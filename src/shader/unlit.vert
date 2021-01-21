#version 450

layout(location = 0) in vec3 a_Pos;

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
    gl_Position = u_ViewProj * u_World * vec4(a_Pos, 1.0);
}