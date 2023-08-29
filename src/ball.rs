use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_rapier3d::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
    }
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    const RADIUS: f32 = 1.0;

    commands
        .spawn((
            Name::new("Ball"),
            // Player,
            PbrBundle {
                mesh: meshes.add(
                    shape::UVSphere {
                        radius: RADIUS,
                        ..default()
                    }
                    .into(),
                ),
                material: debug_material,
                transform: Transform::from_xyz(0.0, 5.0, 0.0),
                ..default()
            },
        ))
        .insert((
            RigidBody::Dynamic,
            Collider::ball(RADIUS),
            ColliderMassProperties::Mass(10.),
            // Restitution::new(1.0),
            // Velocity::default(),
            // ExternalForce::default(),
            // Damping {
            //     linear_damping: 0.5,
            //     ..Default::default()
            // },
        ));
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
    )
}
