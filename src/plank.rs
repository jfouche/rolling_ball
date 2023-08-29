use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlankPlugin;

impl Plugin for PlankPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_plank);
    }
}

fn spawn_plank(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const SIZE: (f32, f32, f32) = (8.0, 1.0, 30.0);

    commands
        .spawn((
            Name::new("Plank"),
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
