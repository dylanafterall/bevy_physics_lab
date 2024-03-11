use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct MagnetDemo;

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_magnet_demo(mut commands: Commands) {}

pub fn despawn_magnet_demo(mut commands: Commands, demo_query: Query<Entity, With<MagnetDemo>>) {
    for demo_entity in demo_query.iter() {
        commands.entity(demo_entity).despawn_recursive();
    }
}
