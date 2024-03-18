use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct JointDemo;

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_joint_demo(mut commands: Commands) {
    // -------------------------------------------------------------------------
    // -------------------------------------------------------------------------
    let prismatic_anchor = commands
        .spawn((
            Name::new("JointPrismaticAnchor"),
            JointDemo,
            RigidBody::Static,
            Collider::rectangle(5.0, 5.0),
            TransformBundle::from_transform(Transform::from_xyz(-110.0, 40.0, 0.0)),
            DebugRender {
                axis_lengths: None,
                ..default()
            },
        ))
        .id();
    let prismatic_object = commands
        .spawn((
            Name::new("JointPrismaticObject"),
            JointDemo,
            RigidBody::Dynamic,
            Collider::circle(5.0),
            TransformBundle::from_transform(Transform::from_xyz(-110.0, 20.0, 0.0)),
            DebugRender {
                axis_lengths: Some(Vec2::new(3.0, 3.0)),
                collider_color: Some(Color::CYAN),
                ..default()
            },
        ))
        .id();
    commands.entity(prismatic_object).with_children(|cmd| {
        cmd.spawn(
            PrismaticJoint::new(prismatic_anchor, prismatic_object)
                .with_local_anchor_1(Vec2::new(0.0, -20.0))
                .with_free_axis(Vec2::X)
                .with_limits(-10.0, 10.0)
                .with_compliance(0.0000001),
        );
    });

    // -------------------------------------------------------------------------
    // -------------------------------------------------------------------------
    let prismatic_anchor_2 = commands
        .spawn((
            Name::new("JointPrismaticAnchor2"),
            JointDemo,
            RigidBody::Static,
            Collider::rectangle(5.0, 5.0),
            TransformBundle::from_transform(Transform::from_xyz(-70.0, 40.0, 0.0)),
            DebugRender {
                axis_lengths: None,
                ..default()
            },
        ))
        .id();
    let prismatic_object_2 = commands
        .spawn((
            Name::new("JointPrismaticObject2"),
            JointDemo,
            RigidBody::Dynamic,
            Collider::circle(5.0),
            TransformBundle::from_transform(Transform::from_xyz(-70.0, 20.0, 0.0)),
            DebugRender {
                axis_lengths: Some(Vec2::new(3.0, 3.0)),
                collider_color: Some(Color::CYAN),
                ..default()
            },
        ))
        .id();
    commands.entity(prismatic_object_2).with_children(|cmd| {
        cmd.spawn(
            PrismaticJoint::new(prismatic_anchor_2, prismatic_object_2)
                .with_local_anchor_1(Vec2::new(0.0, -10.0))
                .with_local_anchor_2(Vec2::new(0.0, 10.0))
                .with_free_axis(Vec2::X)
                .with_limits(-10.0, 10.0)
                .with_compliance(0.0000001),
        );
    });

    // -------------------------------------------------------------------------
    // -------------------------------------------------------------------------
    let prismatic_anchor_3 = commands
        .spawn((
            Name::new("JointPrismaticAnchor3"),
            JointDemo,
            RigidBody::Static,
            Collider::rectangle(5.0, 5.0),
            TransformBundle::from_transform(Transform::from_xyz(-30.0, 40.0, 0.0)),
            DebugRender {
                axis_lengths: None,
                ..default()
            },
        ))
        .id();
    let prismatic_object_3 = commands
        .spawn((
            Name::new("JointPrismaticObject3"),
            JointDemo,
            RigidBody::Dynamic,
            Collider::circle(5.0),
            TransformBundle::from_transform(Transform::from_xyz(-30.0, 20.0, 0.0)),
            DebugRender {
                axis_lengths: Some(Vec2::new(3.0, 3.0)),
                collider_color: Some(Color::CYAN),
                ..default()
            },
        ))
        .id();
    commands.entity(prismatic_object_3).with_children(|cmd| {
        cmd.spawn(
            PrismaticJoint::new(prismatic_anchor_3, prismatic_object_3)
                .with_local_anchor_1(Vec2::new(0.0, -10.0))
                .with_local_anchor_2(Vec2::new(0.0, 10.0))
                .with_free_axis(Vec2::new(0.5, 0.5))
                .with_limits(-10.0, 10.0)
                .with_compliance(0.0000001),
        );
    });

    // -------------------------------------------------------------------------
    // -------------------------------------------------------------------------
    let revolute_anchor = commands
        .spawn((
            Name::new("JointRevoluteAnchor"),
            JointDemo,
            RigidBody::Static,
            Collider::rectangle(5.0, 5.0),
            TransformBundle::from_transform(Transform::from_xyz(-110.0, -20.0, 0.0)),
            DebugRender {
                axis_lengths: None,
                ..default()
            },
        ))
        .id();
    let revolute_object = commands
        .spawn((
            Name::new("JointRevoluteObject"),
            JointDemo,
            RigidBody::Dynamic,
            Collider::circle(5.0),
            TransformBundle::from_transform(Transform::from_xyz(-110.0, -40.0, 0.0)),
            DebugRender {
                axis_lengths: Some(Vec2::new(3.0, 3.0)),
                collider_color: Some(Color::YELLOW),
                ..default()
            },
        ))
        .id();
    commands.entity(revolute_object).with_children(|cmd| {
        cmd.spawn(
            RevoluteJoint::new(revolute_anchor, revolute_object)
                .with_local_anchor_2(Vec2::new(0.0, 20.0))
                .with_angular_velocity_damping(0.5)
                .with_compliance(0.0000001),
        );
    });

    // -------------------------------------------------------------------------
    // -------------------------------------------------------------------------
    let revolute_anchor_2 = commands
        .spawn((
            Name::new("JointRevoluteAnchor2"),
            JointDemo,
            RigidBody::Static,
            Collider::rectangle(5.0, 5.0),
            TransformBundle::from_transform(Transform::from_xyz(-70.0, -20.0, 0.0)),
            DebugRender {
                axis_lengths: None,
                ..default()
            },
        ))
        .id();
    let revolute_object_2 = commands
        .spawn((
            Name::new("JointRevoluteObject2"),
            JointDemo,
            RigidBody::Dynamic,
            Collider::circle(5.0),
            TransformBundle::from_transform(Transform::from_xyz(-70.0, -40.0, 0.0)),
            DebugRender {
                axis_lengths: Some(Vec2::new(3.0, 3.0)),
                collider_color: Some(Color::YELLOW),
                ..default()
            },
        ))
        .id();
    commands.entity(revolute_object_2).with_children(|cmd| {
        cmd.spawn(
            RevoluteJoint::new(revolute_anchor_2, revolute_object_2)
                .with_local_anchor_1(Vec2::new(0.0, -10.0))
                .with_local_anchor_2(Vec2::new(0.0, 10.0))
                .with_angular_velocity_damping(0.5)
                .with_compliance(0.0000001),
        );
    });

    // -------------------------------------------------------------------------
    // -------------------------------------------------------------------------
    let revolute_anchor_3 = commands
        .spawn((
            Name::new("JointRevoluteAnchor3"),
            JointDemo,
            RigidBody::Static,
            Collider::rectangle(5.0, 5.0),
            TransformBundle::from_transform(Transform::from_xyz(-30.0, -20.0, 0.0)),
            DebugRender {
                axis_lengths: None,
                ..default()
            },
        ))
        .id();
    let revolute_object_3 = commands
        .spawn((
            Name::new("JointRevoluteObject3"),
            JointDemo,
            RigidBody::Dynamic,
            Collider::circle(5.0),
            TransformBundle::from_transform(Transform::from_xyz(-30.0, -40.0, 0.0)),
            DebugRender {
                axis_lengths: Some(Vec2::new(3.0, 3.0)),
                collider_color: Some(Color::YELLOW),
                ..default()
            },
        ))
        .id();
    commands.entity(revolute_object_3).with_children(|cmd| {
        cmd.spawn(
            RevoluteJoint::new(revolute_anchor_3, revolute_object_3)
                .with_local_anchor_2(Vec2::new(0.0, 20.0))
                .with_angle_limits(-0.5, 0.5)
                .with_angular_velocity_damping(0.5)
                .with_compliance(0.0000001),
        );
    });

    // -------------------------------------------------------------------------
    // -------------------------------------------------------------------------
    let rope_anchor = commands
        .spawn((
            Name::new("JointRopeAnchor"),
            JointDemo,
            RigidBody::Static,
            Collider::rectangle(5.0, 5.0),
            TransformBundle::from_transform(Transform::from_xyz(30.0, 40.0, 0.0)),
            DebugRender {
                axis_lengths: None,
                ..default()
            },
        ))
        .id();
    let rope_object = commands
        .spawn((
            Name::new("JointRopeObject"),
            JointDemo,
            RigidBody::Dynamic,
            Collider::circle(5.0),
            TransformBundle::from_transform(Transform::from_xyz(30.0, 20.0, 0.0)),
            DebugRender {
                axis_lengths: Some(Vec2::new(3.0, 3.0)),
                collider_color: Some(Color::FUCHSIA),
                ..default()
            },
        ))
        .id();
    commands.entity(rope_object).with_children(|cmd| {
        cmd.spawn(
            DistanceJoint::new(rope_anchor, rope_object)
                .with_rest_length(20.0)
                .with_limits(0.0, 20.0)
                .with_compliance(0.0000001),
        );
    });

    // -------------------------------------------------------------------------
    // -------------------------------------------------------------------------
    let distance_anchor = commands
        .spawn((
            Name::new("JointDistanceAnchor"),
            JointDemo,
            RigidBody::Static,
            Collider::rectangle(5.0, 5.0),
            TransformBundle::from_transform(Transform::from_xyz(70.0, 40.0, 0.0)),
            DebugRender {
                axis_lengths: None,
                ..default()
            },
        ))
        .id();
    let distance_object = commands
        .spawn((
            Name::new("JointDistanceObject"),
            JointDemo,
            RigidBody::Dynamic,
            Collider::circle(5.0),
            TransformBundle::from_transform(Transform::from_xyz(70.0, 20.0, 0.0)),
            DebugRender {
                axis_lengths: Some(Vec2::new(3.0, 3.0)),
                collider_color: Some(Color::FUCHSIA),
                ..default()
            },
        ))
        .id();
    commands.entity(distance_object).with_children(|cmd| {
        cmd.spawn(
            DistanceJoint::new(distance_anchor, distance_object)
                .with_rest_length(20.0)
                .with_limits(18.0, 22.0)
                .with_compliance(0.001),
        );
    });
}

pub fn despawn_joint_demo(mut commands: Commands, demo_query: Query<Entity, With<JointDemo>>) {
    for demo_entity in demo_query.iter() {
        commands.entity(demo_entity).despawn_recursive();
    }
}
