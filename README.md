# Bevy gradient background

[![Crates.io](https://img.shields.io/crates/v/bevy_gradient_background.svg)](https://crates.io/crates/bevy_gradient_background) [![Docs.rs](https://docs.rs/bevy_gradient_background/badge.svg)](https://docs.rs/bevy_gradient_background) [![License](https://img.shields.io/crates/l/bevy_gradient_background.svg)](https://crates.io/crates/bevy_gradient_background)

Bevy gradient background adds a simple 2-color, top-to-bottom gradient background to the scene.

For compatibility, see the [version table](#compatible-bevy-versions).

## Example

```rust
use bevy::prelude::*;
use bevy_gradient_background::GradientBackground;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GradientBackground {
            top_color: Color::srgb_u8(0, 0, 255),
            bottom_color: Color::srgb_u8(0, 255, 0),
            ..default()
        })
        .add_systems(Startup, add_primitives)
        .run();
}

fn add_primitives(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(-20.0, 5.0, 0.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            range: 50.0,
            ..default()
        },
        Transform::from_xyz(-20.0, 5.0, 0.0),
    ));

    let mesh_handle = meshes.add(Cylinder::new(2.5, 5.0));
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb_u8(240, 240, 240),
        ..default()
    });
    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
        Transform::from_xyz(0.0, 0.0, -5.0),
    ));

    let mesh_handle = meshes.add(Cuboid::new(5.0, 5.0, 5.0));
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::srgb_u8(240, 240, 240),
        ..default()
    });
    commands.spawn((
        Mesh3d(mesh_handle),
        MeshMaterial3d(material_handle),
        Transform::from_xyz(0.0, 0.0, 5.0),
    ));
}
```

## Compatible Bevy versions

Compatibility of `bevy_gradient_background` versions:

| `bevy` version | `bevy_gradient_background` version |
|:---------------|:-----------------------------------|
| `0.19`         | `0.1`                              |
