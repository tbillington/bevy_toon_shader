#![allow(clippy::type_complexity)]

use std::f32::consts::PI;

use bevy::{prelude::*, window::close_on_esc};
// use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_toon_shader::{ToonShaderMainCamera, ToonShaderMaterial, ToonShaderPlugin, ToonShaderSun};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Toon Shader".to_owned(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(ToonShaderPlugin)
        // .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                // ui_example_system,
                rotate_shapes,
                close_on_esc,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut toon_materials: ResMut<Assets<ToonShaderMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 8., 12.0)
                .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        },
        ToonShaderMainCamera,
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_000.,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(2.0, 2.0, 2.0),
                rotation: Quat::from_euler(EulerRot::XYZ, -PI / 4., PI / 6., 0.),
                ..default()
            },
            ..default()
        },
        ToonShaderSun,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::GRAY * 0.2,
        brightness: 0.10,
    });

    let toon_material_textured = toon_materials.add(ToonShaderMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let toon_material = toon_materials.add(ToonShaderMaterial::default());

    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let num_shapes = shapes.len();
    const X_EXTENT: f32 = 14.5;

    for (i, mesh) in shapes.into_iter().enumerate() {
        // Texture
        commands.spawn((
            MaterialMeshBundle {
                mesh: mesh.clone(),
                material: toon_material_textured.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    2.0,
                    3.0,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));

        // Without Texture
        commands.spawn((
            MaterialMeshBundle {
                mesh,
                material: toon_material.clone(),
                transform: Transform::from_xyz(
                    -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                    2.0,
                    -3.0,
                )
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
                ..default()
            },
            Shape,
        ));
    }

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::SILVER),
        ..default()
    });
}

// fn ui_example_system(
//     mut contexts: EguiContexts,
//     mut ambient_light: Option<ResMut<AmbientLight>>,
//     mut controls: ParamSet<(
//         // Camera position
//         Query<&Transform, With<ToonShaderMainCamera>>,
//         // Sun position
//         Query<(&mut Transform, &DirectionalLight), With<ToonShaderSun>>,
//     )>,
// ) {
//     egui::Window::new("Controls").show(contexts.ctx_mut(), |ui| {
//         if let Some(ambient_light) = ambient_light.as_mut() {
//             ui.heading("Ambient Light");
//             let mut orig = ambient_light.color.as_rgba_f32();
//             if ui.color_edit_button_rgba_unmultiplied(&mut orig).changed() {
//                 ambient_light.color = Color::rgba_from_array(orig);
//             }
//         }

//         if let Ok((mut t, _)) = controls.p1().get_single_mut() {
//             ui.heading("Sun");
//             ui.horizontal(|ui| {
//                 ui.label("Angle");

//                 let (mut x, mut y, z) = t.rotation.to_euler(EulerRot::XYZ);
//                 x = x.to_degrees();
//                 y = y.to_degrees();
//                 ui.add(egui::widgets::DragValue::new(&mut x).speed(1.));
//                 ui.add(egui::widgets::DragValue::new(&mut y).speed(1.));
//                 t.rotation = Quat::from_euler(EulerRot::XYZ, x.to_radians(), y.to_radians(), z);
//             });
//         }
//     });
// }

#[derive(Component)]
struct Shape;

fn rotate_shapes(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

// Copied from bevy 3d shapes example https://github.com/bevyengine/bevy/blob/1c5c94715cb17cda5ae209eef12a938501de90b5/examples/3d/3d_shapes.rs#L96
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        bevy::render::render_resource::Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        bevy::render::render_resource::TextureDimension::D2,
        &texture_data,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
        bevy::render::render_asset::RenderAssetUsages::default(),
    )
}
