# Bevy Toon Shader

[![Crates.io](https://img.shields.io/crates/v/bevy_toon_shader)](https://crates.io/crates/bevy_toon_shader)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/tbillington/bevy_toon_shader#license)

A [toon shader](https://en.wikipedia.org/wiki/Cel_shading) for the [Bevy](https://github.com/bevyengine/bevy/) game engine.

<img width="1234" alt="bevy_toon_shader" src="https://user-images.githubusercontent.com/2771466/233092241-71a0f13a-fc0a-4022-913c-ddc3658d7f48.png">

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

## License

Except where noted, all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.

### Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
