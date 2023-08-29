use bevy::prelude::*;
use bevy_rapier3d::render::RapierDebugRenderPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierDebugRenderPlugin::default());
    }
}
