use super::*;
use crate::game::{demo_state::DemoState, player};

use bevy::prelude::*;
use bevy_xpbd_2d::PostProcessCollisions;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter DemoState:: ---------------------------------------------
            .add_systems(OnEnter(DemoState::Home), home_demo::spawn_home_demo)
            .add_systems(
                OnEnter(DemoState::Colliders),
                colliders_demo::spawn_colliders_demo,
            )
            .add_systems(
                OnEnter(DemoState::ConveyorBelt),
                conveyor_belt_demo::spawn_conveyor_belt_demo,
            )
            .add_systems(
                OnEnter(DemoState::Magnet),
                magnet_demo::spawn_magnet_demo.after(player::spawn_player),
            )
            .add_systems(OnEnter(DemoState::Joint), joint_demo::spawn_joint_demo)
            .add_systems(
                OnEnter(DemoState::Destructible),
                (
                    destructible_demo::spawn_destructible_platform,
                    destructible_demo::spawn_destructible_prismatic_grid,
                    destructible_demo::spawn_destructible_revolute_grid,
                    destructible_demo::spawn_destructible_distance_grid,
                    destructible_demo::spawn_destructible_static_grid,
                ),
            )
            // OnExit DemoState:: ----------------------------------------------
            .add_systems(OnExit(DemoState::Home), home_demo::despawn_home_demo)
            .add_systems(
                OnExit(DemoState::Colliders),
                colliders_demo::despawn_colliders_demo,
            )
            .add_systems(
                OnExit(DemoState::ConveyorBelt),
                conveyor_belt_demo::despawn_conveyor_belt_demo,
            )
            .add_systems(OnExit(DemoState::Magnet), magnet_demo::despawn_magnet_demo)
            .add_systems(OnExit(DemoState::Joint), joint_demo::despawn_joint_demo)
            .add_systems(
                OnExit(DemoState::Destructible),
                destructible_demo::despawn_destructible_demo,
            )
            // Update ----------------------------------------------------------
            .add_systems(
                Update,
                (
                    home_demo::handle_kinematic_platforms.run_if(in_state(DemoState::Home)),
                    colliders_demo::rotate_gravity.run_if(in_state(DemoState::Colliders)),
                    conveyor_belt_demo::handle_belt_collisions
                        .run_if(in_state(DemoState::ConveyorBelt)),
                    conveyor_belt_demo::spawn_and_despawn_blocks
                        .run_if(in_state(DemoState::ConveyorBelt)),
                    magnet_demo::apply_magnet_forces.run_if(in_state(DemoState::Magnet)),
                    magnet_demo::toggle_oscillating_magnets.run_if(in_state(DemoState::Magnet)),
                    destructible_demo::break_destructible_joints
                        .run_if(in_state(DemoState::Destructible)),
                ),
            )
            // PostProcessCollisions -------------------------------------------
            .add_systems(
                PostProcessCollisions,
                (home_demo::one_way_platform.run_if(in_state(DemoState::Home)),),
            );
    }
}
