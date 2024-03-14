use crate::game::player::Player;

use bevy::{prelude::*, utils::Duration};
use bevy_xpbd_2d::prelude::*;

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct MagnetDemo;

// color convention for magnet charges:
// Positive = RED, Negative = BLUE, 0.0 = WHITE
pub enum MagnetStatus {
    Positive,
    Negative,
    Off,
}

#[derive(Component)]
pub struct Magnet {
    // charge_abs should always be positive
    pub charge_abs: f32,
    pub status: MagnetStatus,
}

#[derive(Component)]
pub struct OscillatingMagnet {
    pub timer: Timer,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_magnet_demo(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut DebugRender), With<Player>>,
) {
    // disable player collisions for this demo, make it less visible
    let (player_ent, mut player_render) = player_query.single_mut();
    commands.entity(player_ent).insert(CollisionLayers::NONE);
    player_render.axis_lengths = None;
    player_render.collider_color = Some(Color::rgba(0.5, 0.5, 0.5, 0.1));

    // spawn barrier to contain dynamic magnets --------------------------------
    commands
        .spawn((
            Name::new("MagnetBottomWall"),
            MagnetDemo,
            RigidBody::Static,
            Collider::rectangle(200.0, 10.0),
            Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
            TransformBundle::from_transform(Transform::from_xyz(0.0, -60.0, 0.0)),
            DebugRender {
                axis_lengths: None,
                collider_color: Some(Color::WHITE),
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("MagnetLeftWall"),
                Collider::rectangle(10.0, 130.0),
                Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
                TransformBundle::from_transform(Transform::from_xyz(-105.0, 60.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
            children.spawn((
                Name::new("MagnetRightWall"),
                Collider::rectangle(10.0, 130.0),
                Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
                TransformBundle::from_transform(Transform::from_xyz(105.0, 60.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
            children.spawn((
                Name::new("MagnetTopWall"),
                Collider::rectangle(200.0, 10.0),
                Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
                TransformBundle::from_transform(Transform::from_xyz(0.0, 120.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
        });

    // spawn a large, oscillating-charge magnet in the center ------------------
    commands
        .spawn((
            Name::new("MagnetCenter"),
            MagnetDemo,
            Magnet {
                charge_abs: 10_000.0,
                status: MagnetStatus::Off,
            },
            OscillatingMagnet {
                timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
            },
            RigidBody::Dynamic,
            LockedAxes::ALL_LOCKED,
            Collider::circle(15.0),
            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
            TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
            DebugRender::default().with_collider_color(Color::WHITE),
        ))
        .with_children(|children| {
            children.spawn((
                Collider::circle(95.0),
                Sensor,
                DebugRender {
                    // semi-transparent Color::ORANGE
                    collider_color: Some(Color::rgba(1.0, 0.65, 0.0, 0.1)),
                    ..default()
                },
            ));
        });

    // spawn a bunch of small, dynamic magnets ---------------------------------
    let mut magnet_small =
        |x_pos: f32, y_pos: f32, charge: f32, status: MagnetStatus, color: Color| {
            commands
                .spawn((
                    Name::new("MagnetSmall"),
                    MagnetDemo,
                    Magnet {
                        charge_abs: charge,
                        status,
                    },
                    RigidBody::Dynamic,
                    Collider::circle(1.5),
                    GravityScale(0.0),
                    TransformBundle::from_transform(Transform::from_xyz(x_pos, y_pos, 0.0)),
                    DebugRender {
                        collider_color: Some(color),
                        axis_lengths: Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                ))
                .with_children(|children| {
                    children.spawn((
                        Collider::circle(4.5),
                        Sensor,
                        DebugRender {
                            // semi-transparent Color::ORANGE
                            collider_color: Some(Color::rgba(1.0, 0.65, 0.0, 0.1)),
                            ..default()
                        },
                    ));
                });
        };

    for x in (-88..89).step_by(8) {
        magnet_small(x as f32, 30.0, 15.0, MagnetStatus::Negative, Color::BLUE);
    }
    for x in (-88..89).step_by(8) {
        magnet_small(x as f32, -30.0, 15.0, MagnetStatus::Positive, Color::RED);
    }
}

pub fn despawn_magnet_demo(mut commands: Commands, demo_query: Query<Entity, With<MagnetDemo>>) {
    for demo_entity in demo_query.iter() {
        commands.entity(demo_entity).despawn_recursive();
    }
}

pub fn apply_magnet_forces(
    magnet_query: Query<(&Magnet, &Collider, &Transform, &Children)>,
    colliders_query: Query<&CollidingEntities>,
    mut impulse_query: Query<&mut ExternalImpulse, With<Magnet>>,
) {
    // outer magnet loop - for each magnet, check if its (child) sensor detects any colliders
    for (i_magnet, _i_collider, i_transform, i_children) in magnet_query.iter() {
        if let Ok(sensor_colliding_ents) = colliders_query.get(i_children[0]) {
            // inner collider loop - iterate through the detected colliders
            for collider_ent in sensor_colliding_ents.0.iter() {
                // if the current collider is also a magnet, apply an impulse to it
                if let Ok((j_magnet, j_collider, j_transform, _j_children)) =
                    magnet_query.get(*collider_ent)
                {
                    let distance = j_collider.distance_to_point(
                        j_transform.translation.xy(),
                        j_transform.rotation,
                        i_transform.translation.xy(),
                        true,
                    );

                    let i_charge = i_magnet.charge_abs
                        * match i_magnet.status {
                            MagnetStatus::Positive => 1.0,
                            MagnetStatus::Negative => -1.0,
                            MagnetStatus::Off => 0.0,
                        };

                    let j_charge = j_magnet.charge_abs
                        * match j_magnet.status {
                            MagnetStatus::Positive => 1.0,
                            MagnetStatus::Negative => -1.0,
                            MagnetStatus::Off => 0.0,
                        };

                    let magnitude = i_charge * j_charge / f32::powi(distance, 2);
                    let direction =
                        (j_transform.translation.xy() - i_transform.translation.xy()).normalize();

                    if let Ok(mut j_impulse) = impulse_query.get_mut(*collider_ent) {
                        j_impulse
                            .set_impulse(direction * magnitude)
                            .with_persistence(false);
                    }
                }
            }
        }
    }
}

pub fn toggle_oscillating_magnets(
    time: Res<Time>,
    mut magnet_query: Query<(&mut OscillatingMagnet, &mut Magnet, &mut DebugRender)>,
) {
    for (mut oscillator, mut magnet, mut render) in magnet_query.iter_mut() {
        oscillator.timer.tick(time.delta());

        if oscillator.timer.just_finished() {
            // cycle the magnet status and set new corresponding color
            magnet.status = match magnet.status {
                MagnetStatus::Positive => {
                    render.collider_color = Some(Color::BLUE);
                    MagnetStatus::Negative
                }
                MagnetStatus::Negative => {
                    render.collider_color = Some(Color::WHITE);
                    MagnetStatus::Off
                }
                MagnetStatus::Off => {
                    render.collider_color = Some(Color::RED);
                    MagnetStatus::Positive
                }
            }
        }
    }
}
