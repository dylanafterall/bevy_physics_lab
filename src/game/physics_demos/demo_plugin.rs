use super::*;
use crate::game::demo_state::DemoState;

use bevy::prelude::*;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(DemoState::Home), home_demo::spawn_home_demo)
            .add_systems(OnExit(DemoState::Home), home_demo::despawn_home_demo);
    }
}
