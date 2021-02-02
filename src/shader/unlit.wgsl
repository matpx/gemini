[[location(0)]]
var<in> in_position: vec3<f32>;

[[location(1)]]
var<in> in_uv_vs: vec2<f32>;

[[location(2)]]
var<in> in_normal_vs: vec3<f32>;

[[location(0)]]
var<out> out_uv: vec2<f32>;

[[location(1)]]
var<out> out_normal: vec3<f32>;

[[builtin(position)]]
var<out> out_position: vec4<f32>;


[[block]]
struct Globals {
    view_proj: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> u_globals: Globals;


[[block]]
struct Locals {
    model: mat4x4<f32>;
};

[[group(1), binding(0)]]
var r_locals: Locals;


[[stage(vertex)]]
fn vs_main() {
    out_uv = in_uv_vs;
    out_normal = in_normal_vs;
    out_position = u_globals.view_proj * r_locals.model * vec4<f32>(in_position, 1.0);
}


[[location(0)]]
var<in> in_uv_fs: vec2<f32>;

[[location(1)]]
var<in> in_normal_fs: vec3<f32>;

[[location(0)]]
var<out> out_color: vec4<f32>;


[[block]]
struct Primitive {
    color: vec4<f32>;
};

[[group(2), binding(0)]]
var r_primitive: Primitive;


[[stage(fragment)]]
fn fs_main() {
    out_color = vec4<f32>(1.0,1.0,1.0,1.0);
}