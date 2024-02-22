struct ToonShaderMaterial {
    color: vec4<f32>,
    sun_dir: vec3<f32>,
    sun_color: vec4<f32>,
    camera_pos: vec3<f32>,
    ambient_color: vec4<f32>,
};

@group(2) @binding(0)
var<uniform> material: ToonShaderMaterial;
@group(2) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(2) @binding(2)
var base_color_sampler: sampler;

#import bevy_pbr::forward_io::VertexOutput

@fragment
fn fragment (in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = material.color * textureSample(base_color_texture, base_color_sampler, in.uv);
    let normal = normalize(in.world_normal);
    let n_dot_l = dot(material.sun_dir, normal);
    var light_intensity = 0.0;

    if n_dot_l > 0.0 {
        let bands = 3.0;
        var x = n_dot_l * bands;

        x = round(x);

        light_intensity = x / bands;
    } else {
        light_intensity = 0.0;
    }

    let light = light_intensity * material.sun_color;

    let view_dir: vec3<f32> = normalize(material.camera_pos - in.world_position.xyz);

    let half_vector = normalize(material.sun_dir + view_dir);
    let n_dot_h = dot(normal, half_vector);
    let glossiness = 32.0;
    let specular_intensity = pow(n_dot_h, glossiness * glossiness);

    let specular_intensity_smooth = smoothstep(0.005, 0.01, specular_intensity);
    let specular = specular_intensity_smooth * vec4<f32>(0.9, 0.9 ,0.9 ,1.0);

    return base_color * (light + material.ambient_color + specular);
}
