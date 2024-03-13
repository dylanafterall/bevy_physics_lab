use super::*;
use crate::game::demo_state::DemoState;

use bevy::prelude::*;
use bevy_xpbd_2d::PostProcessCollisions;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(DemoState::Home), home_demo::spawn_home_demo)
            .add_systems(OnExit(DemoState::Home), home_demo::despawn_home_demo)
            .add_systems(
                OnEnter(DemoState::Colliders),
                colliders_demo::spawn_colliders_demo,
            )
            .add_systems(
                OnExit(DemoState::Colliders),
                colliders_demo::despawn_colliders_demo,
            )
            .add_systems(
                OnEnter(DemoState::ConveyorBelt),
                conveyor_belt_demo::spawn_conveyor_belt_demo,
            )
            .add_systems(
                OnExit(DemoState::ConveyorBelt),
                conveyor_belt_demo::despawn_conveyor_belt_demo,
            )
            .add_systems(OnEnter(DemoState::Magnet), magnet_demo::spawn_magnet_demo)
            .add_systems(OnExit(DemoState::Magnet), magnet_demo::despawn_magnet_demo)
            .add_systems(
                Update,
                (
                    colliders_demo::rotate_gravity.run_if(in_state(DemoState::Colliders)),
                    conveyor_belt_demo::handle_belt_collisions
                        .run_if(in_state(DemoState::ConveyorBelt)),
                    conveyor_belt_demo::spawn_and_despawn_blocks
                        .run_if(in_state(DemoState::ConveyorBelt)),
                    magnet_demo::toggle_magnet.run_if(in_state(DemoState::Magnet)),
                ),
            )
            .add_systems(
                PostProcessCollisions,
                (conveyor_belt_demo::one_way_platform.run_if(in_state(DemoState::ConveyorBelt)),),
            );
    }
}
