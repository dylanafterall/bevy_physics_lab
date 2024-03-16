use crate::game::player::Player;

use bevy::{prelude::*, utils::Duration};
use bevy_xpbd_2d::prelude::*;

// polarity conventions --------------------------------------------------------
// -----------------------------------------------------------------------------
const POS_POL_COLOR: Color = Color::RED;
const POS_POL_COEFF: f32 = 1.0;

const NEG_POL_COLOR: Color = Color::BLUE;
const NEG_POL_COEFF: f32 = -1.0;

const NEUT_POL_COLOR: Color = Color::WHITE;
const NEUT_POL_COEFF: f32 = 0.0;

const MAGNET_SENSOR_COL: Color = Color::rgba(1.0, 0.65, 0.0, 0.05);

pub enum MagnetPolarity {
    Positive,
    Negative,
    Neutral,
}

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct MagnetDemo;

#[derive(Component)]
pub struct Magnet {
    // abs_charge should always be >= 0.0
    pub abs_charge: f32,
    pub polarity: MagnetPolarity,
}

#[derive(Component)]
pub struct OscillatingMagnet {
    pub osc_timer: Timer,
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
                abs_charge: 10_000.0,
                polarity: MagnetPolarity::Neutral,
            },
            OscillatingMagnet {
                osc_timer: Timer::new(Duration::from_secs(10), TimerMode::Repeating),
            },
            RigidBody::Dynamic,
            LockedAxes::ALL_LOCKED,
            Collider::circle(15.0),
            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
            TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
            DebugRender::default().with_collider_color(NEUT_POL_COLOR),
        ))
        .with_children(|children| {
            children.spawn((
                Collider::circle(95.0),
                Sensor,
                DebugRender {
                    collider_color: Some(MAGNET_SENSOR_COL),
                    ..default()
                },
            ));
        });

    // spawn a bunch of small, dynamic magnets ---------------------------------
    let mut magnet_small =
        |x_pos: f32, y_pos: f32, charge: f32, polarity: MagnetPolarity, color: Color| {
            commands
                .spawn((
                    Name::new("MagnetSmall"),
                    MagnetDemo,
                    Magnet {
                        abs_charge: charge,
                        polarity,
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
                            collider_color: Some(MAGNET_SENSOR_COL),
                            ..default()
                        },
                    ));
                });
        };

    for x in (-88..89).step_by(8) {
        magnet_small(
            x as f32,                 // x coord
            30.0,                     // y coord
            15.0,                     // abs_charge
            MagnetPolarity::Negative, // polarity
            NEG_POL_COLOR,            // debug render color
        );
    }
    for x in (-88..89).step_by(8) {
        magnet_small(
            x as f32,
            -30.0,
            15.0,
            MagnetPolarity::Positive,
            POS_POL_COLOR,
        );
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
                    // calculate distance and charges in preparation for force magnitude calc
                    let distance = j_collider.distance_to_point(
                        j_transform.translation.xy(),
                        j_transform.rotation,
                        i_transform.translation.xy(),
                        true,
                    );
                    // use polarity to determine if magnets will attract / repel / nothing
                    let i_charge = i_magnet.abs_charge
                        * match i_magnet.polarity {
                            MagnetPolarity::Positive => POS_POL_COEFF,
                            MagnetPolarity::Negative => NEG_POL_COEFF,
                            MagnetPolarity::Neutral => NEUT_POL_COEFF,
                        };
                    let j_charge = j_magnet.abs_charge
                        * match j_magnet.polarity {
                            MagnetPolarity::Positive => POS_POL_COEFF,
                            MagnetPolarity::Negative => NEG_POL_COEFF,
                            MagnetPolarity::Neutral => NEUT_POL_COEFF,
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
        oscillator.osc_timer.tick(time.delta());

        if oscillator.osc_timer.just_finished() {
            // cycle the magnet polarity and set new corresponding color
            magnet.polarity = match magnet.polarity {
                MagnetPolarity::Positive => {
                    render.collider_color = Some(NEG_POL_COLOR);
                    MagnetPolarity::Negative
                }
                MagnetPolarity::Negative => {
                    render.collider_color = Some(NEUT_POL_COLOR);
                    MagnetPolarity::Neutral
                }
                MagnetPolarity::Neutral => {
                    render.collider_color = Some(POS_POL_COLOR);
                    MagnetPolarity::Positive
                }
            }
        }
    }
}
