#![allow(clippy::type_complexity)]

use std::f32::consts::PI;

use bevy::{prelude::*, window::close_on_esc};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_toon_shader::{ToonShaderMainCamera, ToonShaderMaterial, ToonShaderPlugin, ToonShaderSun};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ToonShaderPlugin)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_systems((ui_example_system, close_on_esc))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<CustomMaterial>>,
    mut toon_materials: ResMut<Assets<ToonShaderMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                // hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            transform: Transform::from_xyz(0.0, 6., 12.0)
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
                rotation: Quat::from_rotation_x(-PI / 4.),
                ..default()
            },
            ..default()
        },
        ToonShaderSun,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::RED * 0.25,
        brightness: 0.10,
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::try_from(shape::Icosphere::default()).unwrap()),
        transform: Transform::from_xyz(0.0, 2.5, 0.0),
        material: toon_materials.add(ToonShaderMaterial {
            base_color_texture: None,
            color: Color::RED,
            // flags: 256,
            sun_pos: Vec3::new(0.0, 2.5, 0.0) + Vec3::ONE,
            sun_dir: Vec3::ZERO,
            sun_color: Color::WHITE,
            camera_pos: Vec3::new(0.0, 6., 12.0),
            // is_red: true,
            ambient_color: Color::WHITE,
        }),
        // material: materials.add(CustomMaterial {
        //     color: Color::BLUE,
        //     // flags: 256,
        //     sun_pos: vec3(0.0, 2.5, 0.0) + Vec3::ONE,
        //     // is_red: true,
        // }),
        ..default()
    });
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut ambient_light: Option<ResMut<AmbientLight>>,
    mut controls: ParamSet<(
        Query<&Transform, With<ToonShaderMainCamera>>,
        Query<(&mut Transform, &DirectionalLight), With<ToonShaderSun>>,
    )>,
) {
    egui::Window::new("Controls").show(contexts.ctx_mut(), |ui| {
        if let Some(ambient_light) = ambient_light.as_mut() {
            ui.label("Color: ");
            let mut orig = ambient_light.color.as_rgba_f32();
            if ui.color_edit_button_rgba_unmultiplied(&mut orig).changed() {
                ambient_light.color = Color::from(orig);
            }
        }

        if let Ok((mut t, _)) = controls.p1().get_single_mut() {
            ui.horizontal(|ui| {
                ui.label("Dir: ");

                let (mut x, mut y, z) = t.rotation.to_euler(EulerRot::XYZ);
                x = x.to_degrees();
                y = y.to_degrees();
                ui.add(egui::widgets::DragValue::new(&mut x).speed(1.));
                ui.add(egui::widgets::DragValue::new(&mut y).speed(1.));
                t.rotation = Quat::from_euler(EulerRot::XYZ, x.to_radians(), y.to_radians(), z);
            });
        }
    });
}
