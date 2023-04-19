# Bevy Toon Shader

**Features**
- "Sun" color & direction
- Specular highlight
- Light banding
- Shadow casting
- Albedo map (base color texture)

**Not supported (yet)**
- Dynamic color banding amount
- Shadow receiving
- Rim lighting

<img width="1234" alt="bevy_toon_shader" src="https://user-images.githubusercontent.com/2771466/233092241-71a0f13a-fc0a-4022-913c-ddc3658d7f48.png">

## Installation

```sh
cargo add bevy_toon_shader
```

```rust
use bevy::prelude::*;
use bevy_toon_shader::{ToonShaderMainCamera, ToonShaderMaterial, ToonShaderPlugin, ToonShaderSun};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ToonShaderPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut toon_materials: ResMut<Assets<ToonShaderMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3dBundle { /* ... */ },
        ToonShaderMainCamera,
    ));

    // Sun
    commands.spawn((
        DirectionalLightBundle { /* ... */ },
        ToonShaderSun,
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::GRAY * 0.2,
        ..default()
    });

    let material = toon_materials.add(ToonShaderMaterial {
        base_color_texture: None,
        color: Color::WHITE,
        sun_dir: Vec3::ZERO,
        sun_color: Color::WHITE,
        camera_pos: Vec3::new(0.0, 6., 12.0),
        ambient_color: Color::WHITE,
    });

    // 3D object
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::try_from(shape::Torus::default()).unwrap()),
        transform: Transform::from_xyz(0., 2., 0.),
        material: toon_material,
        ..default()
    });
}
```

## Running the example

```sh
git clone https://github.com/tbillington/bevy_toon_shader.git
cd bevy_toon_shader
cargo run --example scene
```

## Bevy Support Table

| bevy | bevy_toon_shader |
| -- | -- |
| 0.10 | 0.1 |

## Credit

Code initially adapted from this [excellent Unity tutorial](https://roystan.net/articles/toon-shader/) by Roystan.

