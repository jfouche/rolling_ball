use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

pub struct PlankPlugin;

impl Plugin for PlankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_plank)
            .add_systems(Update, move_plank);
    }
}

#[derive(Component)]
pub struct Plank;

fn spawn_plank(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const SIZE: (f32, f32, f32) = (8.0, 1.0, 30.0);

    commands
        .spawn((
            Name::new("Plank"),
            Plank,
            PbrBundle {
                mesh: meshes.add(shape::Box::new(SIZE.0, SIZE.1, SIZE.2).into()),
                material: materials.add(Color::MAROON.into()),
                ..default()
            },
        ))
        .insert((
            RigidBody::Fixed,
            Collider::cuboid(SIZE.0 / 2.0, SIZE.1 / 2.0, SIZE.2 / 2.0),
        ));
}

fn move_plank(
    mut planks: Query<&mut Transform, With<Plank>>,
    buttons: Res<Input<MouseButton>>,
    mut motion: EventReader<MouseMotion>,
) {
    if buttons.pressed(MouseButton::Right) {
        if let Ok(mut transform) = planks.get_single_mut() {
            for ev in motion.iter() {
                // angle = delta px deg converted to rad
                let dz = ev.delta.x.to_radians();
                let dx = -ev.delta.y.to_radians();
                transform.rotate_local_x(dx);
                transform.rotate_local_z(dz);
            }
        }
    }
}
