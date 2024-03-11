use crate::game::player;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct ConveyorBeltDemo;

#[derive(Component)]
pub struct ConveyorBelt {
    pub belt_direction: Vec2,
}

#[derive(Component)]
pub struct OneWayPlatform {
    pub allowed_direction: Vec2,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_conveyor_belt_demo(mut commands: Commands) {
    commands
        .spawn((
            Name::new("ConveyorBottomWall"),
            ConveyorBeltDemo,
            RigidBody::Static,
            Collider::rectangle(180.0, 10.0),
            TransformBundle::from_transform(Transform::from_xyz(0.0, -50.0, 0.0)),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("ConveyorLeftWall"),
                Collider::rectangle(10.0, 100.0),
                TransformBundle::from_transform(Transform::from_xyz(-95.0, 45.0, 0.0)),
            ));
            children.spawn((
                Name::new("ConveyorRightWall"),
                Collider::rectangle(10.0, 100.0),
                TransformBundle::from_transform(Transform::from_xyz(95.0, 45.0, 0.0)),
            ));
        });

    commands
        .spawn((
            Name::new("ConveyorBelt"),
            ConveyorBeltDemo,
            ConveyorBelt {
                belt_direction: Vec2::X,
            },
            RigidBody::Static,
            Collider::rectangle(80.0, 5.0),
            TransformBundle::from_transform(Transform::from_xyz(-40.0, -25.0, 0.0)),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("ConveyorBeltCasing"),
                RigidBody::Static,
                Collider::rectangle(84.0, 5.0),
                TransformBundle::from_transform(Transform::from_xyz(0.0, -1.0, 0.0)),
            ));
        });

    commands.spawn((
        Name::new("ConveyorOneWayPlatform"),
        ConveyorBeltDemo,
        OneWayPlatform {
            allowed_direction: Vec2::Y,
        },
        RigidBody::Static,
        Collider::rectangle(60.0, 5.0),
        TransformBundle::from_transform(Transform::from_xyz(40.0, 25.0, 0.0)),
    ));

    commands.spawn((
        Name::new("ConveyorRoundRectangle"),
        ConveyorBeltDemo,
        RigidBody::Dynamic,
        Collider::round_rectangle(8.0, 4.0, 1.5),
        TransformBundle::from_transform(Transform::from_xyz(30.0, 45.0, 0.0)),
    ));
}

pub fn despawn_conveyor_belt_demo(
    mut commands: Commands,
    demo_query: Query<Entity, With<ConveyorBeltDemo>>,
) {
    for demo_entity in demo_query.iter() {
        commands.entity(demo_entity).despawn_recursive();
    }
}

pub fn handle_belt_collisions(
    belt_query: Query<(&ConveyorBelt, &CollidingEntities)>,
    mut force_query: Query<&mut ExternalImpulse>,
) {
    for (belt_dir, colliding_entities) in belt_query.iter() {
        for colliding_ent in colliding_entities.0.iter() {
            if let Ok(mut collider_force) = force_query.get_mut(*colliding_ent) {
                collider_force.apply_impulse(belt_dir.belt_direction);
            }
        }
    }
}

pub fn handle_one_way_collisions(
    mut collisions: ResMut<Collisions>,
    platform_query: Query<(&OneWayPlatform, &CollidingEntities)>,
    player_query: Query<(Entity, &LinearVelocity), With<player::Player>>,
) {
}
