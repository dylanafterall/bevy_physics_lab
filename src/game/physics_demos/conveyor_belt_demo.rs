use crate::game::player::*;

use bevy::{
    prelude::*,
    utils::{Duration, HashSet},
};
use bevy_xpbd_2d::{
    math::{Scalar, Vector},
    prelude::*,
};

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct ConveyorBeltDemo;

#[derive(Component)]
pub struct ConveyorBelt {
    pub belt_vector: Vec2,
}

#[derive(Component)]
pub struct ConveyorBeltDemoTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct ConveyorBeltBlockTimer {
    pub timer: Timer,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct OneWayPlatform(HashSet<Entity>);

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_conveyor_belt_demo(mut commands: Commands) {
    commands.spawn((
        Name::new("ConveyorBottomWall"),
        ConveyorBeltDemo,
        RigidBody::Static,
        Collider::rectangle(180.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(0.0, -50.0, 0.0)),
        DebugRender {
            axis_lengths: None,
            collider_color: Some(Color::WHITE),
            ..default()
        },
    ));

    commands
        .spawn((
            Name::new("ConveyorBeltBottom"),
            ConveyorBeltDemo,
            ConveyorBelt {
                belt_vector: Vec2::new(20.0, 0.0),
            },
            OneWayPlatform::default(),
            RigidBody::Static,
            Collider::round_rectangle(100.0, 2.0, 1.0),
            TransformBundle::from_transform(Transform::from_xyz(-50.0, -20.0, 0.0)),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("ConveyorBeltCasingBottom"),
                OneWayPlatform::default(),
                RigidBody::Static,
                Collider::rectangle(105.0, 5.0),
                TransformBundle::from_transform(Transform::from_xyz(0.0, -1.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
        });

    commands
        .spawn((
            Name::new("ConveyorBeltMiddle"),
            ConveyorBeltDemo,
            ConveyorBelt {
                belt_vector: Vec2::new(-20.0, 0.0),
            },
            OneWayPlatform::default(),
            RigidBody::Static,
            Collider::round_rectangle(100.0, 2.0, 1.0),
            TransformBundle::from_transform(Transform::from_xyz(40.0, 15.0, 0.0)),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("ConveyorBeltCasingMiddle"),
                OneWayPlatform::default(),
                RigidBody::Static,
                Collider::rectangle(105.0, 5.0),
                TransformBundle::from_transform(Transform::from_xyz(0.0, -1.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
        });

    commands
        .spawn((
            Name::new("ConveyorBeltTop"),
            ConveyorBeltDemo,
            ConveyorBelt {
                belt_vector: Vec2::new(20.0, 0.0),
            },
            OneWayPlatform::default(),
            RigidBody::Static,
            Collider::round_rectangle(100.0, 2.0, 1.0),
            TransformBundle::from_transform(Transform::from_xyz(-40.0, 50.0, 0.0)),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("ConveyorBeltCasingTop"),
                OneWayPlatform::default(),
                RigidBody::Static,
                Collider::rectangle(105.0, 5.0),
                TransformBundle::from_transform(Transform::from_xyz(0.0, -1.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
        });

    commands.spawn((
        Name::new("ConveyorRoundRectangle"),
        ConveyorBeltDemo,
        ConveyorBeltBlockTimer {
            timer: Timer::new(Duration::from_secs(20), TimerMode::Repeating),
        },
        RigidBody::Dynamic,
        Restitution::new(0.1),
        Collider::round_rectangle(8.0, 4.0, 1.5),
        TransformBundle::from_transform(Transform::from_xyz(-80.0, 70.0, 0.0)),
    ));

    commands.spawn((
        Name::new("ConveyorBeltDemoTimer"),
        ConveyorBeltDemo,
        ConveyorBeltDemoTimer {
            timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
        },
    ));
}

pub fn spawn_and_despawn_blocks(
    mut commands: Commands,
    time: Res<Time>,
    mut demo_timer_query: Query<&mut ConveyorBeltDemoTimer>,
    mut block_query: Query<(Entity, &mut ConveyorBeltBlockTimer)>,
) {
    let mut demo_timer = demo_timer_query.single_mut();
    demo_timer.timer.tick(time.delta());

    if demo_timer.timer.just_finished() {
        commands.spawn((
            Name::new("ConveyorRoundRectangle"),
            ConveyorBeltDemo,
            ConveyorBeltBlockTimer {
                timer: Timer::new(Duration::from_secs(20), TimerMode::Repeating),
            },
            RigidBody::Dynamic,
            Restitution::new(0.1),
            Collider::round_rectangle(8.0, 4.0, 1.5),
            TransformBundle::from_transform(Transform::from_xyz(-80.0, 70.0, 0.0)),
        ));
    }

    for (block_entity, mut block_timer) in block_query.iter_mut() {
        block_timer.timer.tick(time.delta());

        if block_timer.timer.just_finished() {
            commands.entity(block_entity).despawn();
        }
    }
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
    mut impulse_query: Query<&mut ExternalImpulse>,
) {
    for (belt, colliding_entities) in belt_query.iter() {
        for colliding_ent in colliding_entities.0.iter() {
            if let Ok(mut collider_impulse) = impulse_query.get_mut(*colliding_ent) {
                collider_impulse.set_impulse(belt.belt_vector);
            }
        }
    }
}

pub fn one_way_platform(
    mut one_way_platforms_query: Query<&mut OneWayPlatform>,
    other_colliders_query: Query<
        Option<&PassThroughOneWayPlatform>,
        (With<Collider>, Without<OneWayPlatform>), // NOTE: This precludes OneWayPlatform passing through a OneWayPlatform
    >,
    mut collisions: ResMut<Collisions>,
) {
    // This assumes that Collisions contains empty entries for entities
    // that were once colliding but no longer are.
    collisions.retain(|contacts| {
        // This is used in a couple of if statements below; writing here for brevity below.
        fn any_penetrating(contacts: &Contacts) -> bool {
            contacts.manifolds.iter().any(|manifold| {
                manifold
                    .contacts
                    .iter()
                    .any(|contact| contact.penetration > 0.0)
            })
        }

        // Differentiate between which normal of the manifold we should use
        enum RelevantNormal {
            Normal1,
            Normal2,
        }

        // First, figure out which entity is the one-way platform, and which is the other.
        // Choose the appropriate normal for pass-through depending on which is which.
        let (mut one_way_platform, other_entity, relevant_normal) =
            if let Ok(one_way_platform) = one_way_platforms_query.get_mut(contacts.entity1) {
                (one_way_platform, contacts.entity2, RelevantNormal::Normal1)
            } else if let Ok(one_way_platform) = one_way_platforms_query.get_mut(contacts.entity2) {
                (one_way_platform, contacts.entity1, RelevantNormal::Normal2)
            } else {
                // Neither is a one-way-platform, so accept the collision:
                // we're done here.
                return true;
            };

        if one_way_platform.0.contains(&other_entity) {
            // If we were already allowing a collision for a particular entity,
            // and if it is penetrating us still, continue to allow it to do so.
            if any_penetrating(contacts) {
                return false;
            } else {
                // If it's no longer penetrating us, forget it.
                one_way_platform.0.remove(&other_entity);
            }
        }

        match other_colliders_query.get(other_entity) {
            // Pass-through is set to never, so accept the collision.
            Ok(Some(PassThroughOneWayPlatform::Never)) => true,
            // Pass-through is set to always, so always ignore this collision
            // and register it as an entity that's currently penetrating.
            Ok(Some(PassThroughOneWayPlatform::Always)) => {
                one_way_platform.0.insert(other_entity);
                false
            }
            // Default behaviour is "by normal".
            Err(_) | Ok(None) | Ok(Some(PassThroughOneWayPlatform::ByNormal)) => {
                // If all contact normals are in line with the local up vector of this platform,
                // then this collision should occur: the entity is on top of the platform.
                if contacts.manifolds.iter().all(|manifold| {
                    let normal = match relevant_normal {
                        RelevantNormal::Normal1 => manifold.normal1,
                        RelevantNormal::Normal2 => manifold.normal2,
                    };

                    normal.length() > Scalar::EPSILON && normal.dot(Vector::Y) >= 0.5
                }) {
                    true
                } else if any_penetrating(contacts) {
                    // If it's already penetrating, ignore the collision and register
                    // the other entity as one that's currently penetrating.
                    one_way_platform.0.insert(other_entity);
                    false
                } else {
                    // In all other cases, allow this collision.
                    true
                }
            }
        }
    });
}
