use bevy::prelude::*;

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
    commands
        .spawn((
            Name::new("Plank"),
            PbrBundle {
                mesh: meshes.add(shape::Box::new(30.0, 1.0, 8.0).into()),
                material: materials.add(Color::MAROON.into()),
                ..default()
            },
        ))
        // .insert((RigidBody::Fixed, Collider::cuboid(25., 0., 25.)))
        ;
}
