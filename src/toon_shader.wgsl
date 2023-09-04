#define_import_path bevy_pbr::fragment

#import bevy_pbr::pbr_functions as pbr_functions
#import bevy_pbr::pbr_bindings as pbr_bindings
#import bevy_pbr::pbr_types as pbr_types
#import bevy_pbr::prepass_utils

#import bevy_pbr::mesh_vertex_output       MeshVertexOutput
#import bevy_pbr::mesh_bindings            mesh
#import bevy_pbr::mesh_view_bindings       view, fog, screen_space_ambient_occlusion_texture
#import bevy_pbr::mesh_view_types          FOG_MODE_OFF
#import bevy_core_pipeline::tonemapping    screen_space_dither, powsafe, tone_mapping
#import bevy_pbr::parallax_mapping         parallaxed_uv

#import bevy_pbr::prepass_utils

#import bevy_pbr::gtao_utils gtao_multibounce

struct ToonShaderMaterial {
    color: vec4<f32>,
    sun_dir: vec3<f32>,
    sun_color: vec4<f32>,
    camera_pos: vec3<f32>,
    ambient_color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: ToonShaderMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

// #import bevy_pbr::mesh_vertex_output MeshVertexOutput
// #import bevy_pbr::shadows fetch_directional_shadow
// #import bevy_pbr::mesh_view_bindings view

@fragment
fn fragment(
    in: MeshVertexOutput,
    @builtin(front_facing) is_front: bool,
) -> @location(0) vec4<f32> {

    let base_color = textureSample(base_color_texture, base_color_sampler, in.uv);

    var pbr = pbr_functions::pbr_input_new();
    pbr.frag_coord = in.position;
    pbr.world_position = in.world_position;
    pbr.world_normal = in.world_normal;
    pbr.N = in.world_normal;

    // pbr.material.base_color = vec4(0.5, 0.5, 0.5, 1.0);
    pbr.material.base_color = base_color;
    pbr.material.perceptual_roughness = 0.05;
    var output_color = pbr_functions::pbr(pbr);

    return output_color;
}

// @fragment
// fn fragment (in: MeshVertexOutput) -> @location(0) vec4<f32> {
//     let base_color = material.color * textureSample(base_color_texture, base_color_sampler, in.uv);
//     let normal = normalize(in.world_normal);
//     let n_dot_l = dot(material.sun_dir, normal);
//     var light_intensity = 0.0;

//     if n_dot_l > 0.0 {
//         let bands = 3.0;
//         var x = n_dot_l * bands;

//         x = round(x);

//         light_intensity = x / bands;
//     } else {
//         light_intensity = 0.0;
//     }

//     var light = light_intensity * material.sun_color;

//     let view_dir: vec3<f32> = normalize(material.camera_pos - in.world_position.xyz);

//     let half_vector = normalize(material.sun_dir + view_dir);
//     let n_dot_h = dot(normal, half_vector);
//     let glossiness = 32.0;
//     let specular_intensity = pow(n_dot_h, glossiness * glossiness);

//     let specular_intensity_smooth = smoothstep(0.005, 0.01, specular_intensity);
//     let specular = specular_intensity_smooth * vec4<f32>(0.9, 0.9 ,0.9 ,1.0);

//     let camera_view = view;
//     let view_z = dot(vec4<f32>(
//         camera_view.inverse_view[0].z,
//         camera_view.inverse_view[1].z,
//         camera_view.inverse_view[2].z,
//         camera_view.inverse_view[3].z
//     ), in.world_position);
//     let shadow = fetch_directional_shadow(0u, in.world_position, in.world_normal, view_z);

//     light = light * shadow;
//     light = light + material.ambient_color;
//     light = light + specular;

//     return base_color * light;
// }
