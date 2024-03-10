use super::{camera, demo_state, physics_demos::demo_plugin, player};

use bevy::{prelude::*, transform::TransformSystem};
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<demo_state::DemoState>()
            .add_event::<demo_state::EditDemoState>()
            .add_event::<camera::CameraTranslation>()
            .add_systems(Startup, (camera::spawn_camera, player::spawn_player))
            .add_systems(
                Update,
                (
                    demo_state::handle_edit_demo_state,
                    player::handle_player_input,
                ),
            )
            .add_systems(
                PostUpdate,
                camera::camera_follow_player
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            )
            .add_plugins((
                PhysicsPlugins::default(),
                PhysicsDebugPlugin::default(),
                InputManagerPlugin::<player::PlayerAction>::default(),
                demo_plugin::DemoPlugin,
            ))
            .insert_resource(Gravity(Vec2::NEG_Y * 100.0));
    }
}
