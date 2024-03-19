use crate::game::player::PassThroughOneWayPlatform;

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
pub struct HomeDemo;

#[derive(Component)]
pub enum KinematicPlatformType {
    Moving,
    Spinning,
}

#[derive(Component)]
pub struct KinematicPlatform {
    pub variant: KinematicPlatformType,
    pub translate_vec: Option<Vec2>,
    pub rotate_scalar: Option<f32>,
    pub timer: Timer,
}

#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct OneWayPlatform(HashSet<Entity>);

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_home_demo(mut commands: Commands) {
    commands.spawn((
        Name::new("HomePlatformOneWay"),
        HomeDemo,
        OneWayPlatform::default(),
        RigidBody::Static,
        Collider::rectangle(150.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(0.0, -50.0, 0.0)),
        DebugRender {
            axis_lengths: None,
            collider_color: Some(Color::WHITE),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("HomePlatformMove"),
        HomeDemo,
        KinematicPlatform {
            variant: KinematicPlatformType::Moving,
            translate_vec: Some(Vec2::new(0.0, 20.0)),
            rotate_scalar: None,
            timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
        },
        RigidBody::Kinematic,
        Collider::rectangle(50.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(75.0, -20.0, 0.0)),
        DebugRender {
            axis_lengths: None,
            collider_color: Some(Color::WHITE),
            ..default()
        },
    ));

    commands.spawn((
        Name::new("HomePlatformSpin"),
        HomeDemo,
        KinematicPlatform {
            variant: KinematicPlatformType::Spinning,
            translate_vec: None,
            rotate_scalar: Some(-1.5),
            timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
        },
        RigidBody::Kinematic,
        Collider::rectangle(50.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(-85.0, 0.0, 0.0)),
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
            }
            // If it's no longer penetrating us, forget it.
            one_way_platform.0.remove(&other_entity);
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

pub fn handle_kinematic_platforms(
    time: Res<Time>,
    mut platform_query: Query<(
        &mut KinematicPlatform,
        &mut LinearVelocity,
        &mut AngularVelocity,
    )>,
) {
    for (mut platform, mut pf_vel, mut pf_ang_vel) in platform_query.iter_mut() {
        platform.timer.tick(time.delta());

        match platform.variant {
            KinematicPlatformType::Moving => {
                pf_vel.0 = platform
                    .translate_vec
                    .expect("Moving platforms should have a translate_vec field assigned");
                if platform.timer.just_finished() {
                    platform.translate_vec = Some(platform.translate_vec.unwrap() * -1.0);
                }
            }
            KinematicPlatformType::Spinning => {
                pf_ang_vel.0 = platform
                    .rotate_scalar
                    .expect("Spinning platforms should have a rotate_scalar field asigned");
                if platform.timer.just_finished() {
                    platform.rotate_scalar = Some(platform.rotate_scalar.unwrap() * -1.0);
                }
            }
        }
    }
}
