use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct CollidersDemo;

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
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("CollidersLeftWall"),
                Collider::rectangle(10.0, 100.0),
                TransformBundle::from_transform(Transform::from_xyz(-95.0, 45.0, 0.0)),
            ));
            children.spawn((
                Name::new("CollidersRightWall"),
                Collider::rectangle(10.0, 100.0),
                TransformBundle::from_transform(Transform::from_xyz(95.0, 45.0, 0.0)),
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
        Collider::ellipse(3.0, 5.0),
        TransformBundle::from_transform(Transform::from_xyz(-60.0, 0.0, 0.0)),
    ));

    commands.spawn((
        Name::new("CollidersRoundRectangle"),
        CollidersDemo,
        RigidBody::Dynamic,
        Collider::round_rectangle(8.0, 4.0, 1.5),
        TransformBundle::from_transform(Transform::from_xyz(-40.0, 0.0, 0.0)),
    ));

    commands.spawn((
        Name::new("CollidersTriangle"),
        CollidersDemo,
        RigidBody::Dynamic,
        Collider::triangle(
            Vec2::new(-4.0, -5.0),
            Vec2::new(4.0, -5.0),
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
}

pub fn despawn_colliders_demo(
    mut commands: Commands,
    demo_query: Query<Entity, With<CollidersDemo>>,
) {
    for demo_entity in demo_query.iter() {
        commands.entity(demo_entity).despawn_recursive();
    }
}
