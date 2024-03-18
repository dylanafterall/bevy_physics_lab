// code inspired by bevy_rapier_2d's "joints2.rs" example
// https://github.com/dimforge/bevy_rapier/blob/master/bevy_rapier2d/examples/joints2.rs
// -----------------------------------------------------------------------------
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

// hexagon tesselation  --------------------------------------------------------
// -----------------------------------------------------------------------------
pub const HEX_RADIUS: f32 = 5.0;
// using pointy-top hexagon convention (vice flat-top)
pub const HEX_WIDTH: f32 = 1.7320508 * HEX_RADIUS;
pub const HEX_X_SHIFT: f32 = 0.5 * HEX_WIDTH;

pub const HEX_HEIGHT: f32 = 2.0 * HEX_RADIUS;
pub const HEX_Y_SHIFT: f32 = 0.75 * HEX_HEIGHT;

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct DestructibleDemo;

#[derive(Component)]
pub struct Destructible {
    pub impulse_threshold: f32,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_destructible_platform(mut commands: Commands) {
    commands
        .spawn((
            Name::new("DestructiblePlatform"),
            DestructibleDemo,
            RigidBody::Static,
            Collider::rectangle(600.0, 10.0),
            TransformBundle::from_transform(Transform::from_xyz(220.0, -50.0, 0.0)),
            DebugRender {
                axis_lengths: None,
                collider_color: Some(Color::WHITE),
                ..default()
            },
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("DestuctibleRightWall"),
                Collider::rectangle(10.0, 110.0),
                TransformBundle::from_transform(Transform::from_xyz(305.0, 50.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
        });
}

pub fn spawn_destructible_prismatic_grid(mut commands: Commands) {
    let numi = 10; // number of VERTICAL hexagons (num rows)
    let numk = 10; // number of HORIZONTAL hexagons (num columns)
    let start_pos = Vec2::new(20.0, -35.0); // position of bottom left hexagon

    let mut body_entities = Vec::new();

    // k : column
    for k in 0..numk {
        // i : row
        for i in 0..numi {
            let fk = k as f32;
            let fi = i as f32;

            // let (debug_color, rigid_body) =
            //     if (i % 5 == 0 || i == numi - 1) && (k % 5 == 0 || k == numk - 1) {
            //         (Color::WHITE, RigidBody::Static)
            //     } else {
            //         (Color::ORANGE, RigidBody::Dynamic)
            //     };

            let row_offset = if i % 2 == 0 {
                0.0 // if an even num row, don't offset horizontally
            } else {
                HEX_X_SHIFT // if an odd num row, offset horizontally by half of hex width
            };

            // hexagon
            let child_entity = commands
                .spawn((
                    DestructibleDemo,
                    Destructible {
                        impulse_threshold: 10.0,
                    },
                    RigidBody::Dynamic,
                    GravityScale(0.01),
                    Collider::regular_polygon(HEX_RADIUS - 0.75, 6),
                    TransformBundle::from_transform(Transform::from_xyz(
                        start_pos.x + row_offset + (fk * HEX_WIDTH),
                        start_pos.y + (fi * HEX_Y_SHIFT),
                        0.0,
                    )),
                    DebugRender {
                        axis_lengths: None,
                        collider_color: Some(Color::CYAN),
                        ..default()
                    },
                ))
                .id();

            // vertical joint
            if i > 0 {
                // as we build vertical joint chain, alternate joint vector x-component
                let x_shift = if i % 2 == 0 {
                    -HEX_X_SHIFT // if current row even, make joint to hexagon above to the LEFT
                } else {
                    HEX_X_SHIFT // if current row odd, make joint to hexagon above to the RIGHT
                };
                let parent_entity = *body_entities.last().unwrap();
                commands.entity(child_entity).with_children(|cmd| {
                    cmd.spawn(
                        PrismaticJoint::new(parent_entity, child_entity)
                            .with_local_anchor_1(Vec2::new(x_shift, HEX_Y_SHIFT))
                            .with_free_axis(Vec2::Y)
                            .with_limits(-1.5, 1.5)
                            .with_linear_velocity_damping(0.5)
                            .with_angular_velocity_damping(0.5)
                            .with_compliance(0.0000001),
                    );
                });
            }

            // horizontal joint
            if k > 0 {
                // even column, even row hexagons
                if k % 2 == 0 && i % 2 == 0 {
                    let parent_index = body_entities.len() - numi;
                    let parent_entity = body_entities[parent_index];
                    commands.entity(child_entity).with_children(|cmd| {
                        cmd.spawn(
                            PrismaticJoint::new(parent_entity, child_entity)
                                .with_local_anchor_1(Vec2::new(HEX_WIDTH, 0.0))
                                .with_free_axis(Vec2::X)
                                .with_limits(-1.5, 1.5)
                                .with_linear_velocity_damping(0.5)
                                .with_angular_velocity_damping(0.5)
                                .with_compliance(0.0000001),
                        );
                    });
                }
                // odd column, odd row hexagons
                if k % 2 == 1 && i % 2 == 1 {
                    let parent_index = body_entities.len() - numi;
                    let parent_entity = body_entities[parent_index];
                    commands.entity(child_entity).with_children(|cmd| {
                        cmd.spawn(
                            PrismaticJoint::new(parent_entity, child_entity)
                                .with_local_anchor_1(Vec2::new(HEX_WIDTH, 0.0))
                                .with_free_axis(Vec2::X)
                                .with_limits(-1.5, 1.5)
                                .with_linear_velocity_damping(0.5)
                                .with_angular_velocity_damping(0.5)
                                .with_compliance(0.0000001),
                        );
                    });
                }
            }

            body_entities.push(child_entity);
        }
    }
}

pub fn spawn_destructible_revolute_grid(mut commands: Commands) {
    let numi = 10; // number of VERTICAL hexagons (num rows)
    let numk = 10; // number of HORIZONTAL hexagons (num columns)
    let start_pos = Vec2::new(170.0, -35.0); // position of bottom left hexagon

    let mut body_entities = Vec::new();

    // k : column
    for k in 0..numk {
        // i : row
        for i in 0..numi {
            let fk = k as f32;
            let fi = i as f32;

            let row_offset = if i % 2 == 0 {
                0.0 // if an even num row, don't offset horizontally
            } else {
                HEX_X_SHIFT // if an odd num row, offset horizontally by half of hex width
            };

            // hexagon
            let child_entity = commands
                .spawn((
                    DestructibleDemo,
                    Destructible {
                        impulse_threshold: 10.0,
                    },
                    RigidBody::Dynamic,
                    GravityScale(0.01),
                    Collider::regular_polygon(HEX_RADIUS - 0.75, 6),
                    TransformBundle::from_transform(Transform::from_xyz(
                        start_pos.x + row_offset + (fk * HEX_WIDTH),
                        start_pos.y + (fi * HEX_Y_SHIFT),
                        0.0,
                    )),
                    DebugRender {
                        axis_lengths: None,
                        collider_color: Some(Color::YELLOW),
                        ..default()
                    },
                ))
                .id();

            // vertical joint
            if i > 0 {
                // as we build vertical joint chain, alternate joint vector x-component
                let x_shift = if i % 2 == 0 {
                    -HEX_X_SHIFT // if current row even, make joint to hexagon above to the LEFT
                } else {
                    HEX_X_SHIFT // if current row odd, make joint to hexagon above to the RIGHT
                };
                let parent_entity = *body_entities.last().unwrap();
                commands.entity(child_entity).with_children(|cmd| {
                    cmd.spawn(
                        RevoluteJoint::new(parent_entity, child_entity)
                            .with_local_anchor_1(Vec2::new(x_shift, HEX_Y_SHIFT))
                            .with_angle_limits(-1.0, 1.0)
                            .with_linear_velocity_damping(0.5)
                            .with_angular_velocity_damping(0.5)
                            .with_compliance(0.0000001),
                    );
                });
            }

            // horizontal joint
            if k > 0 {
                // even column, even row hexagons
                if k % 2 == 0 && i % 2 == 0 {
                    let parent_index = body_entities.len() - numi;
                    let parent_entity = body_entities[parent_index];
                    commands.entity(child_entity).with_children(|cmd| {
                        cmd.spawn(
                            RevoluteJoint::new(parent_entity, child_entity)
                                .with_local_anchor_1(Vec2::new(HEX_WIDTH, 0.0))
                                .with_angle_limits(-0.25, 0.25)
                                .with_linear_velocity_damping(0.5)
                                .with_angular_velocity_damping(0.5)
                                .with_compliance(0.0000001),
                        );
                    });
                }
                // odd column, odd row hexagons
                if k % 2 == 1 && i % 2 == 1 {
                    let parent_index = body_entities.len() - numi;
                    let parent_entity = body_entities[parent_index];
                    commands.entity(child_entity).with_children(|cmd| {
                        cmd.spawn(
                            RevoluteJoint::new(parent_entity, child_entity)
                                .with_local_anchor_1(Vec2::new(HEX_WIDTH, 0.0))
                                .with_angle_limits(-0.25, 0.25)
                                .with_linear_velocity_damping(0.5)
                                .with_angular_velocity_damping(0.5)
                                .with_compliance(0.0000001),
                        );
                    });
                }
            }

            body_entities.push(child_entity);
        }
    }
}

pub fn spawn_destructible_distance_grid(mut commands: Commands) {
    let numi = 10; // number of VERTICAL hexagons (num rows)
    let numk = 10; // number of HORIZONTAL hexagons (num columns)
    let start_pos = Vec2::new(320.0, -35.0); // position of bottom left hexagon

    let mut body_entities = Vec::new();

    // k : column
    for k in 0..numk {
        // i : row
        for i in 0..numi {
            let fk = k as f32;
            let fi = i as f32;

            let row_offset = if i % 2 == 0 {
                0.0 // if an even num row, don't offset horizontally
            } else {
                HEX_X_SHIFT // if an odd num row, offset horizontally by half of hex width
            };

            // hexagon
            let child_entity = commands
                .spawn((
                    DestructibleDemo,
                    Destructible {
                        impulse_threshold: 10.0,
                    },
                    RigidBody::Dynamic,
                    GravityScale(0.01),
                    Collider::regular_polygon(HEX_RADIUS - 0.75, 6),
                    TransformBundle::from_transform(Transform::from_xyz(
                        start_pos.x + row_offset + (fk * HEX_WIDTH),
                        start_pos.y + (fi * HEX_Y_SHIFT),
                        0.0,
                    )),
                    DebugRender {
                        axis_lengths: None,
                        collider_color: Some(Color::FUCHSIA),
                        ..default()
                    },
                ))
                .id();

            // vertical joint
            if i > 0 {
                let parent_entity = *body_entities.last().unwrap();
                let rest_length = Vec2::new(HEX_X_SHIFT, HEX_Y_SHIFT).length();
                commands.entity(child_entity).with_children(|cmd| {
                    cmd.spawn(
                        DistanceJoint::new(parent_entity, child_entity)
                            .with_rest_length(rest_length)
                            .with_limits(rest_length - 0.1, rest_length + 2.0)
                            .with_compliance(0.0000001),
                    );
                });
            }

            // horizontal joint
            if k > 0 {
                let parent_index = body_entities.len() - numi;
                let parent_entity = body_entities[parent_index];
                commands.entity(child_entity).with_children(|cmd| {
                    cmd.spawn(
                        DistanceJoint::new(parent_entity, child_entity)
                            .with_rest_length(HEX_WIDTH)
                            .with_limits(HEX_WIDTH - 0.1, HEX_WIDTH + 2.0)
                            .with_compliance(0.0000001),
                    );
                });
            }

            body_entities.push(child_entity);
        }
    }
}

pub fn spawn_destructible_static_grid(mut commands: Commands) {
    let numi = 10; // number of VERTICAL hexagons (num rows)
    let numk = 10; // number of HORIZONTAL hexagons (num columns)
    let start_pos = Vec2::new(-150.0, -35.0); // position of top left hexagon

    let mut body_entities = Vec::new();

    // k : column
    for k in 0..numk {
        // i : row
        for i in 0..numi {
            let fk = k as f32;
            let fi = i as f32;

            let row_offset = if i % 2 == 0 {
                0.0 // if an even num row, don't offset horizontally
            } else {
                HEX_X_SHIFT // if an odd num row, offset horizontally by half of hex width
            };

            // hexagon
            let child_entity = commands
                .spawn((
                    DestructibleDemo,
                    Destructible {
                        impulse_threshold: 10.0,
                    },
                    RigidBody::Static,
                    Collider::regular_polygon(HEX_RADIUS - 0.75, 6),
                    TransformBundle::from_transform(Transform::from_xyz(
                        start_pos.x + row_offset + (fk * HEX_WIDTH),
                        start_pos.y + (fi * HEX_Y_SHIFT),
                        0.0,
                    )),
                    DebugRender {
                        axis_lengths: None, // Some(Vec2::new(2.0, 2.0)),
                        ..default()
                    },
                ))
                .id();

            body_entities.push(child_entity);
        }
    }
}

pub fn despawn_destructible_demo(
    mut commands: Commands,
    demo_query: Query<Entity, With<DestructibleDemo>>,
) {
    for demo_entity in demo_query.iter() {
        commands.entity(demo_entity).despawn_recursive();
    }
}

pub fn break_destructible_joints() {}
