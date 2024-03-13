use crate::game::game_plugin::GravityFactor;

use bevy::{prelude::*, utils::Duration};
use bevy_xpbd_2d::prelude::*;

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct CollidersDemo;

#[derive(Component)]
pub struct CollidersTimer {
    pub timer: Timer,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_colliders_demo(mut commands: Commands) {
    commands
        .spawn((
            Name::new("CollidersBottomWall"),
            CollidersDemo,
            RigidBody::Static,
            Collider::rectangle(180.0, 10.0),
            TransformBundle::from_transform(Transform::from_xyz(0.0, -50.0, 0.0)),
            DebugRender {
                axis_lengths: None,
                collider_color: Some(Color::WHITE),
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("CollidersLeftWall"),
                Collider::rectangle(10.0, 110.0),
                TransformBundle::from_transform(Transform::from_xyz(-95.0, 50.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
            children.spawn((
                Name::new("CollidersRightWall"),
                Collider::rectangle(10.0, 110.0),
                TransformBundle::from_transform(Transform::from_xyz(95.0, 50.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
            children.spawn((
                Name::new("CollidersTopWall"),
                Collider::rectangle(180.0, 10.0),
                TransformBundle::from_transform(Transform::from_xyz(0.0, 100.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
        });

    commands.spawn((
        Name::new("CollidersCircle"),
        CollidersDemo,
        RigidBody::Dynamic,
        Collider::circle(5.0),
        Restitution::new(0.9).with_combine_rule(CoefficientCombine::Max),
        TransformBundle::from_transform(Transform::from_xyz(-80.0, 0.0, 0.0)),
    ));

    commands.spawn((
        Name::new("CollidersEllipse"),
        CollidersDemo,
        RigidBody::Dynamic,
        Collider::ellipse(5.0, 7.0),
        TransformBundle::from_transform(Transform::from_xyz(-60.0, 0.0, 0.0)),
    ));

    commands.spawn((
        Name::new("CollidersRoundRectangle"),
        CollidersDemo,
        RigidBody::Dynamic,
        Collider::round_rectangle(9.0, 5.0, 1.5),
        GravityScale(-1.0),
        TransformBundle::from_transform(Transform::from_xyz(-40.0, 0.0, 0.0)),
    ));

    commands.spawn((
        Name::new("CollidersTriangle"),
        CollidersDemo,
        RigidBody::Dynamic,
        Collider::triangle(
            Vec2::new(-5.0, -5.0),
            Vec2::new(5.0, -5.0),
            Vec2::new(0.0, 5.0),
        ),
        TransformBundle::from_transform(Transform::from_xyz(40.0, 0.0, 0.0)),
    ));

    commands.spawn((
        Name::new("CollidersRegPolygon"),
        CollidersDemo,
        RigidBody::Dynamic,
        Collider::regular_polygon(6.0, 9),
        LockedAxes::ROTATION_LOCKED,
        TransformBundle::from_transform(Transform::from_xyz(60.0, 0.0, 0.0)),
    ));

    commands.spawn((
        Name::new("CollidersTimer"),
        CollidersDemo,
        CollidersTimer {
            timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
        },
    ));
}

pub fn despawn_colliders_demo(
    mut commands: Commands,
    demo_query: Query<Entity, With<CollidersDemo>>,
) {
    for demo_entity in demo_query.iter() {
        commands.entity(demo_entity).despawn_recursive();
    }
}

pub fn rotate_gravity(
    time: Res<Time>,
    mut timer_query: Query<&mut CollidersTimer>,
    mut gravity: ResMut<Gravity>,
    g_factor: Res<GravityFactor>,
) {
    let mut colliders_timer = timer_query.single_mut();
    colliders_timer.timer.tick(time.delta());

    if colliders_timer.timer.just_finished() {
        gravity.0 = match gravity.0.normalize() {
            Vec2::NEG_Y => Vec2::X * g_factor.factor,
            Vec2::X => Vec2::Y * g_factor.factor,
            Vec2::Y => Vec2::NEG_X * g_factor.factor,
            _ => Vec2::NEG_Y * g_factor.factor,
        }
    }
}
