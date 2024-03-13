use bevy::{prelude::*, utils::Duration};
use bevy_xpbd_2d::prelude::*;

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct MagnetDemo;

// color convention: Positive = RED, Negative = BLUE, Off = BLACK
#[derive(PartialEq)]
pub enum MagnetStatus {
    Positive,
    Negative,
    Off,
}

#[derive(Component)]
pub struct MagnetStatic {
    pub strength_factor: f32,
    pub status: MagnetStatus,
}

#[derive(Component)]
pub struct MagnetDynamic {
    pub strength_factor: f32,
}

#[derive(Component)]
pub struct MagnetTimer {
    pub timer: Timer,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_magnet_demo(mut commands: Commands) {
    commands
        .spawn((
            Name::new("MagnetBottomWall"),
            MagnetDemo,
            RigidBody::Static,
            Collider::rectangle(200.0, 10.0),
            TransformBundle::from_transform(Transform::from_xyz(0.0, -65.0, 0.0)),
            DebugRender {
                axis_lengths: None,
                collider_color: Some(Color::WHITE),
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("MagnetLeftWall"),
                Collider::rectangle(10.0, 125.0),
                TransformBundle::from_transform(Transform::from_xyz(-105.0, 57.5, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
            children.spawn((
                Name::new("MagnetRightWall"),
                Collider::rectangle(10.0, 125.0),
                TransformBundle::from_transform(Transform::from_xyz(105.0, 57.5, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
            children.spawn((
                Name::new("MagnetTopWall"),
                Collider::rectangle(200.0, 10.0),
                TransformBundle::from_transform(Transform::from_xyz(0.0, 115.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
        });

    commands
        .spawn((
            Name::new("MagnetMain"),
            MagnetDemo,
            MagnetStatic {
                strength_factor: 200.0,
                status: MagnetStatus::Off,
            },
            RigidBody::Static,
            Collider::circle(15.0),
            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
            TransformBundle::from_transform(Transform::from_xyz(0.0, -25.0, 0.0)),
            DebugRender::default().with_collider_color(Color::WHITE),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("MagnetMainSensor"),
                RigidBody::Static,
                Collider::circle(95.0),
                Sensor,
            ));
        });

    let mut magnet_small = |x_pos: f32, y_pos: f32, strength: f32, color: Color| {
        commands.spawn((
            Name::new("MagnetSmall"),
            MagnetDemo,
            MagnetDynamic {
                strength_factor: strength,
            },
            RigidBody::Dynamic,
            Collider::circle(1.5),
            TransformBundle::from_transform(Transform::from_xyz(x_pos, y_pos, 0.0)),
            DebugRender {
                collider_color: Some(color),
                axis_lengths: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
        ));
    };

    for x in (-90..-4).step_by(4) {
        magnet_small(x as f32, -45.0, -100.0, Color::BLUE);
    }
    for x in (4..90).step_by(4) {
        magnet_small(x as f32, -45.0, 100.0, Color::RED);
    }

    commands.spawn((
        Name::new("MagnetTimer"),
        MagnetDemo,
        MagnetTimer {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
        },
    ));
}

pub fn despawn_magnet_demo(mut commands: Commands, demo_query: Query<Entity, With<MagnetDemo>>) {
    for demo_entity in demo_query.iter() {
        commands.entity(demo_entity).despawn_recursive();
    }
}

pub fn toggle_magnet(
    time: Res<Time>,
    mut timer_query: Query<&mut MagnetTimer>,
    mut magnet_stat_query: Query<(&mut MagnetStatic, &mut DebugRender, &Transform, &Children)>,
    colliders_query: Query<&CollidingEntities>,
    mut magnet_dyn_query: Query<(&MagnetDynamic, &Collider, &Transform, &mut ExternalImpulse)>,
) {
    let mut magnet_timer = timer_query.single_mut();
    magnet_timer.timer.tick(time.delta());

    // loop through the static magnets (which EXERT force on dynamic magnets)
    for (mut magnet_stat, mut magnet_render, stat_transform, children) in
        magnet_stat_query.iter_mut()
    {
        // handle toggling MagnetStatus in outer loop, before mutating dyn magnets
        match magnet_stat.status {
            MagnetStatus::Positive => {
                if magnet_timer.timer.just_finished() {
                    magnet_stat.strength_factor *= -1.0;
                    magnet_stat.status = MagnetStatus::Negative;
                    magnet_render.collider_color = Some(Color::BLUE);
                }
            }
            MagnetStatus::Negative => {
                if magnet_timer.timer.just_finished() {
                    magnet_stat.status = MagnetStatus::Off;
                    magnet_render.collider_color = Some(Color::WHITE);
                }
            }
            MagnetStatus::Off => {
                if magnet_timer.timer.just_finished() {
                    magnet_stat.strength_factor *= -1.0;
                    magnet_stat.status = MagnetStatus::Positive;
                    magnet_render.collider_color = Some(Color::RED);
                }
            }
        }

        if let Ok(sensor_colliding_ents) = colliders_query.get(children[0]) {
            // loop through the objects colliding with the static magnet's sensor
            for collider_ent in sensor_colliding_ents.0.iter() {
                // if the current collider is a dynamic magnet, (conditionally) apply a force to it
                if let Ok((magnet_dyn, collider, dyn_transform, mut collider_impulse)) =
                    magnet_dyn_query.get_mut(*collider_ent)
                {
                    let mut force: f32 = 0.0;
                    let mut direction: Vec2 = Vec2::ZERO;
                    // if static magnet is not off, calculate force vector it exerts on dyn magnet
                    if magnet_stat.status != MagnetStatus::Off {
                        let distance = collider.distance_to_point(
                            dyn_transform.translation.xy(),
                            dyn_transform.rotation,
                            stat_transform.translation.xy(),
                            true,
                        );
                        force = magnet_dyn.strength_factor * magnet_stat.strength_factor
                            / f32::powi(distance, 2);
                        direction = (stat_transform.translation.xy()
                            - dyn_transform.translation.xy())
                        .normalize();
                    }

                    match magnet_stat.status {
                        MagnetStatus::Positive | MagnetStatus::Negative => {
                            collider_impulse
                                .set_impulse(direction * force)
                                .with_persistence(false);
                        }
                        MagnetStatus::Off => {}
                    }
                }
            }
        }
    }
}
