use bevy::math::vec3;
use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate, apply_engines, apply_velocity, apply_drag))
        .run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

#[derive(Component, Default)]
struct Velocity(Vec3);

#[derive(Component)]
struct Mass {
    pub kg: f32,
}

#[derive(Component, Default)]
struct Engines(Vec<Engine>);
struct Engine {
    force: f32,
}

const X_EXTENT: f32 = 12.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {

    let my_gltf = ass.load("my.glb#Scene0");
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    let shapes = [
        // meshes.add(Cuboid::default()),
        meshes.add(Capsule3d::default()),
        // meshes.add(Torus::default()),
        // meshes.add(Cylinder::default()),
        // meshes.add(Sphere::default().mesh().ico(2).unwrap()),
        // meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                material: debug_material.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            Shape,
            Velocity {
                0: vec3(0.0, 0.0, 0.0),
            },
            Engines {
                0: vec![Engine { force: 0.5 }],
            },
            Mass { kg: 1.0 },
        ));
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::SILVER),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 6., 12.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
        ..default()
    });
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        // transform.rotate_x(time.delta_seconds() / 2.);
        // transform.rotate_y(time.delta_seconds() / 2.);
        // transform.rotate_z(time.delta_seconds() / 2.);
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += time.delta_seconds() * velocity.0;
    }
}

fn apply_engines(mut query: Query<(&mut Velocity, &Transform, &Engines, &Mass)>, time: Res<Time>) {
    for (mut velocity, transform, engines, mass) in &mut query {
        for engine in &engines.0 {
            velocity.0 += transform
                .rotation
                .mul_vec3(Vec3::new(engine.force / mass.kg, 0.0, 0.0))
                * time.delta_seconds();
        }
    }
}

const DRAG_FACTOR: f32 = 0.5;
fn apply_drag(mut query: Query<&mut Velocity>, time: Res<Time>) {
    for mut velocity in &mut query {
        velocity.0 *= (1.0 - (DRAG_FACTOR * time.delta_seconds()));
    }
}

/// Creates a colorful test pattern
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
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}
