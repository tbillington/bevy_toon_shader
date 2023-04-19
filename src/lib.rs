use bevy::{
    asset::load_internal_asset,
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, AsBindGroupShaderType, RenderPipelineDescriptor, ShaderRef, ShaderType,
            SpecializedMeshPipelineError,
        },
    },
};

pub const TOON_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 11079857277321826659);

pub struct ToonShaderPlugin;

impl Plugin for ToonShaderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            TOON_SHADER_HANDLE,
            "toon_shader.wgsl",
            Shader::from_wgsl
        );

        app.add_plugin(MaterialPlugin::<ToonShaderMaterial>::default());
        app.add_system(update_toon_shader);
    }
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "7b033895-875f-4cb5-97ae-8601fcc37053"]
// #[bind_group_data(ToonShaderMaterialKey)]
#[uniform(0, ToonShaderMaterialUniform)]
pub struct ToonShaderMaterial {
    pub color: Color,
    pub sun_pos: Vec3,
    pub sun_dir: Vec3,
    pub sun_color: Color,
    pub camera_pos: Vec3,
    pub ambient_color: Color,
    #[texture(1)]
    #[sampler(2)]
    pub base_color_texture: Option<Handle<Image>>,
}

impl Material for ToonShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        TOON_SHADER_HANDLE.typed().into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        _descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        Ok(())
    }
}

impl AsBindGroupShaderType<ToonShaderMaterialUniform> for ToonShaderMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &bevy::render::render_asset::RenderAssets<Image>,
    ) -> ToonShaderMaterialUniform {
        ToonShaderMaterialUniform {
            color: self.color.into(),
            sun_pos: self.sun_pos,
            sun_dir: self.sun_dir,
            sun_color: self.sun_color.into(),
            camera_pos: self.camera_pos,
            ambient_color: self.ambient_color.into(),
        }
    }
}

#[derive(Clone, Default, ShaderType)]
pub struct ToonShaderMaterialUniform {
    pub color: Vec4,
    pub sun_pos: Vec3,
    pub sun_dir: Vec3,
    pub sun_color: Vec4,
    pub camera_pos: Vec3,
    pub ambient_color: Vec4,
}

// #[derive(Eq, PartialEq, Hash, Clone)]
// pub struct ToonShaderMaterialKey {}

// impl From<&ToonShaderMaterial> for ToonShaderMaterialKey {
//     fn from(_material: &ToonShaderMaterial) -> Self {
//         Self {}
//     }
// }

#[derive(Component)]
pub struct ToonShaderMainCamera;

#[derive(Component)]
pub struct ToonShaderSun;

pub fn update_toon_shader(
    main_cam: Query<&Transform, With<ToonShaderMainCamera>>,
    sun: Query<(&Transform, &DirectionalLight), With<ToonShaderSun>>,
    ambient_light: Option<Res<AmbientLight>>,
    mut toon_materials: ResMut<Assets<ToonShaderMaterial>>,
) {
    for (_, mut toon_mat) in toon_materials.iter_mut() {
        if let Ok(cam_t) = main_cam.get_single() {
            toon_mat.camera_pos = cam_t.translation;
        }
        if let Ok((sun_t, dir_light)) = sun.get_single() {
            toon_mat.sun_pos = sun_t.translation;
            toon_mat.sun_dir = sun_t.back();
            toon_mat.sun_color = dir_light.color;
        }
        if let Some(light) = &ambient_light {
            toon_mat.ambient_color = light.color;
        }
    }
}
