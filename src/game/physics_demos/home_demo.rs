use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct HomeDemo;

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_home_demo(mut commands: Commands) {
    commands.spawn((
        Name::new("HomePlatform"),
        HomeDemo,
        RigidBody::Static,
        Collider::rectangle(150.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(0.0, -50.0, 0.0)),
        DebugRender {
            axis_lengths: None,
            collider_color: Some(Color::WHITE),
            ..default()
        },
    ));
}

pub fn despawn_home_demo(mut commands: Commands, demo_query: Query<Entity, With<HomeDemo>>) {
    for demo_entity in demo_query.iter() {
        commands.entity(demo_entity).despawn_recursive();
    }
}
