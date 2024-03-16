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
pub fn spawn_destructible_demo(mut commands: Commands) {
    let numi = 10; // number of VERTICAL hexagons (num rows)
    let numk = 10; // number of HORIZONTAL hexagons (num columns)
    let start_pos = Vec2::new(20.0, 7.0); // position of top left hexagon

    let mut body_entities = Vec::new();

    // k : column
    for k in 0..numk {
        // i : row
        for i in 0..numi {
            let fk = k as f32;
            let fi = i as f32;

            let (debug_color, rigid_body) =
                if (i % 5 == 0 || i == numi - 1) && (k % 5 == 0 || k == numk - 1) {
                    (Color::WHITE, RigidBody::Static)
                } else {
                    (Color::ORANGE, RigidBody::Dynamic)
                };

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
                    rigid_body,
                    Collider::regular_polygon(HEX_RADIUS - 0.75, 6),
                    TransformBundle::from_transform(Transform::from_xyz(
                        start_pos.x + row_offset + (fk * HEX_WIDTH),
                        start_pos.y + (fi * HEX_Y_SHIFT),
                        0.0,
                    )),
                    DebugRender {
                        axis_lengths: None, // Some(Vec2::new(2.0, 2.0)),
                        collider_color: Some(debug_color),
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
                            .with_limits(-1.0, 1.0),
                    );
                });
            }

            // horizontal joint
            if k > 0 {
                if i % 2 == 0 && k % 2 == 0 {
                    // even row, even column hexagons
                    let parent_index = body_entities.len() - numi;
                    let parent_entity = body_entities[parent_index];
                    commands.entity(child_entity).with_children(|cmd| {
                        cmd.spawn(
                            PrismaticJoint::new(parent_entity, child_entity)
                                .with_local_anchor_1(Vec2::new(HEX_WIDTH, 0.0))
                                .with_limits(-1.0, 1.0),
                        );
                    });
                } else if i % 2 == 1 && k % 2 == 1 {
                    // odd row, odd column hexagons
                    let parent_index = body_entities.len() - numi;
                    let parent_entity = body_entities[parent_index];
                    commands.entity(child_entity).with_children(|cmd| {
                        cmd.spawn(
                            PrismaticJoint::new(parent_entity, child_entity)
                                .with_local_anchor_1(Vec2::new(HEX_WIDTH, 0.0))
                                .with_limits(-1.0, 1.0),
                        );
                    });
                }
            }

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
