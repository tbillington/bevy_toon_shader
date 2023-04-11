struct ToonShaderMaterial {
    color: vec4<f32>,
    sun_pos: vec3<f32>,
};

@group(1) @binding(0)
var<uniform> material: ToonShaderMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment (in: FragmentInput) -> @location(0) vec4<f32> {
    let base_color = material.color * textureSample(base_color_texture, base_color_sampler, in.uv);
    let normal = normalize(in.world_normal);
    let n_dot_l = dot(material.sun_pos, normal);
    var intensity: f32 = 0.0;
    if n_dot_l > 0.0 {
        intensity = 1.0;
    }
    let ambient_color = vec4<f32>(0.4, 0.4, 0.4, 1.0);
    return base_color * (intensity + ambient_color);
}
