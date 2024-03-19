use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct JointDemo;

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_joint_demo(mut commands: Commands) {
    // anchors -----------------------------------------------------------------
    let mut anchor_spawn = |x: f32, y: f32| {
        commands
            .spawn((
                JointDemo,
                RigidBody::Static,
                Collider::rectangle(5.0, 5.0),
                TransformBundle::from_transform(Transform::from_xyz(x, y, 0.0)),
                DebugRender {
                    axis_lengths: None,
                    ..default()
                },
            ))
            .id()
    };

    let prismatic_anchor = anchor_spawn(-110.0, 40.0);
    let prismatic_anchor_2 = anchor_spawn(-70.0, 40.0);
    let prismatic_anchor_3 = anchor_spawn(-30.0, 40.0);
    let revolute_anchor = anchor_spawn(-110.0, -20.0);
    let revolute_anchor_2 = anchor_spawn(-70.0, -20.0);
    let revolute_anchor_3 = anchor_spawn(-30.0, -20.0);
    let distance_anchor = anchor_spawn(30.0, 40.0);
    let distance_anchor_2 = anchor_spawn(70.0, 40.0);

    // objects ----------------------------------------------------------------
    let mut object_spawn = |x: f32, y: f32, color: Color| {
        commands
            .spawn((
                JointDemo,
                RigidBody::Dynamic,
                Collider::circle(5.0),
                TransformBundle::from_transform(Transform::from_xyz(x, y, 0.0)),
                DebugRender {
                    axis_lengths: Some(Vec2::new(3.0, 3.0)),
                    collider_color: Some(color),
                    ..default()
                },
            ))
            .id()
    };

    let prismatic_object = object_spawn(-110.0, 20.0, Color::CYAN);
    let prismatic_object_2 = object_spawn(-70.0, 20.0, Color::CYAN);
    let prismatic_object_3 = object_spawn(-30.0, 20.0, Color::CYAN);
    let revolute_object = object_spawn(-110.0, -40.0, Color::YELLOW);
    let revolute_object_2 = object_spawn(-70.0, -40.0, Color::YELLOW);
    let revolute_object_3 = object_spawn(-30.0, -40.0, Color::YELLOW);
    let distance_object = object_spawn(30.0, 20.0, Color::FUCHSIA);
    let distance_object_2 = object_spawn(70.0, 20.0, Color::FUCHSIA);

    // joints ------------------------------------------------------------------
    commands.entity(prismatic_object).with_children(|cmd| {
        cmd.spawn(
            PrismaticJoint::new(prismatic_anchor, prismatic_object)
                .with_local_anchor_1(Vec2::new(0.0, -20.0))
                .with_free_axis(Vec2::X)
                .with_limits(-10.0, 10.0)
                .with_compliance(0.0000001),
        );
    });

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

    commands.entity(revolute_object).with_children(|cmd| {
        cmd.spawn(
            RevoluteJoint::new(revolute_anchor, revolute_object)
                .with_local_anchor_2(Vec2::new(0.0, 20.0))
                .with_angular_velocity_damping(0.5)
                .with_compliance(0.0000001),
        );
    });

    commands.entity(revolute_object_2).with_children(|cmd| {
        cmd.spawn(
            RevoluteJoint::new(revolute_anchor_2, revolute_object_2)
                .with_local_anchor_1(Vec2::new(0.0, -10.0))
                .with_local_anchor_2(Vec2::new(0.0, 10.0))
                .with_angular_velocity_damping(0.5)
                .with_compliance(0.0000001),
        );
    });

    commands.entity(revolute_object_3).with_children(|cmd| {
        cmd.spawn(
            RevoluteJoint::new(revolute_anchor_3, revolute_object_3)
                .with_local_anchor_2(Vec2::new(0.0, 20.0))
                .with_angle_limits(-0.5, 0.5)
                .with_angular_velocity_damping(0.5)
                .with_compliance(0.0000001),
        );
    });

    commands.entity(distance_object).with_children(|cmd| {
        cmd.spawn(
            DistanceJoint::new(distance_anchor, distance_object)
                .with_rest_length(20.0)
                .with_limits(0.0, 20.0)
                .with_compliance(0.0000001),
        );
    });

    commands.entity(distance_object_2).with_children(|cmd| {
        cmd.spawn(
            DistanceJoint::new(distance_anchor_2, distance_object_2)
                .with_rest_length(20.0)
                .with_limits(18.0, 22.0)
                .with_compliance(0.001),
        );
    });

    // revolute rope -----------------------------------------------------------
    let numi = 20; // number of bodies connected by joints
    let spacing = 5.0;
    let mut start_pos = Vec2::new(50.0, -20.0); // position of top body in rope

    let mut body_entities = Vec::new();

    for i in 0..numi {
        let fi = i as f32;

        let (rigid_body, color) = if i == 0 {
            (RigidBody::Static, Color::WHITE)
        } else {
            (RigidBody::Dynamic, Color::YELLOW)
        };

        let child_entity = commands
            .spawn((
                JointDemo,
                rigid_body,
                Collider::circle(1.5),
                TransformBundle::from_transform(Transform::from_xyz(
                    start_pos.x,
                    start_pos.y - (fi * spacing),
                    0.0,
                )),
                DebugRender {
                    axis_lengths: Some(Vec2::new(1.0, 1.0)),
                    collider_color: Some(color),
                    ..default()
                },
            ))
            .id();

        // joint
        if i > 0 {
            let parent_entity = *body_entities.last().unwrap();
            commands.entity(child_entity).with_children(|cmd| {
                cmd.spawn(
                    RevoluteJoint::new(parent_entity, child_entity)
                        .with_local_anchor_2(Vec2::new(0.0, spacing))
                        .with_angle_limits(-0.25 - fi * 0.1, 0.25 + fi * 0.1)
                        .with_compliance(0.000001),
                );
            });
        }

        body_entities.push(child_entity);
    }

    // distance rope -----------------------------------------------------------
    start_pos = Vec2::new(90.0, -20.0); // position of top body in rope

    body_entities.clear();

    for i in 0..numi {
        let fi = i as f32;

        let (rigid_body, color) = if i == 0 {
            (RigidBody::Static, Color::WHITE)
        } else {
            (RigidBody::Dynamic, Color::FUCHSIA)
        };

        let child_entity = commands
            .spawn((
                JointDemo,
                rigid_body,
                Collider::circle(1.5),
                TransformBundle::from_transform(Transform::from_xyz(
                    start_pos.x,
                    start_pos.y - (fi * spacing),
                    0.0,
                )),
                DebugRender {
                    axis_lengths: Some(Vec2::new(1.0, 1.0)),
                    collider_color: Some(color),
                    ..default()
                },
            ))
            .id();

        // joint
        if i > 0 {
            let parent_entity = *body_entities.last().unwrap();
            commands.entity(child_entity).with_children(|cmd| {
                cmd.spawn(
                    DistanceJoint::new(parent_entity, child_entity)
                        .with_rest_length(spacing)
                        .with_limits(0.5, spacing)
                        .with_compliance(0.000001),
                );
            });
        }

        body_entities.push(child_entity);
    }
}

pub fn despawn_joint_demo(mut commands: Commands, demo_query: Query<Entity, With<JointDemo>>) {
    for demo_entity in demo_query.iter() {
        commands.entity(demo_entity).despawn_recursive();
    }
}
