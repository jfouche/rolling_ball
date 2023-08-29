use bevy::prelude::*;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};

mod ball;
mod debug;
mod plank;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Rolling ball".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // DEBUG
        .add_plugins(debug::DebugPlugin)
        // GAME PLUGINS
        .add_plugins((ball::BallPlugin, plank::PlankPlugin))
        // MAIN SYSTEMS
        .add_systems(Startup, (spawn_camera, spawn_light))
        // RUN
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let translation = Vec3::new(0.0, 23., 52.0);
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}
