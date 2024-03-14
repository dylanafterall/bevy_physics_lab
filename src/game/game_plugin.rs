use super::{camera, demo_state::*, physics_demos::demo_plugin, player};

use bevy::{prelude::*, transform::TransformSystem};
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

// plugins ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // plugins ---------------------------------------------------------
            .add_plugins((
                PhysicsPlugins::default(),
                PhysicsDebugPlugin::default(),
                InputManagerPlugin::<player::PlayerAction>::default(),
                demo_plugin::DemoPlugin,
            ))
            // resources -------------------------------------------------------
            .insert_resource(GravityFactor { factor: 100.0 })
            // states ----------------------------------------------------------
            .init_state::<DemoState>()
            // events ----------------------------------------------------------
            .add_event::<EditDemoState>()
            .add_event::<camera::CameraTranslation>()
            // Startup ---------------------------------------------------------
            .add_systems(Startup, camera::spawn_camera)
            // PostStartup -----------------------------------------------------
            .add_systems(PostStartup, setup_gravity)
            // Entering DemoState:: --------------------------------------------
            .add_systems(OnEnter(DemoState::Home), player::spawn_player)
            .add_systems(OnEnter(DemoState::Colliders), player::spawn_player)
            .add_systems(OnEnter(DemoState::ConveyorBelt), player::spawn_player)
            .add_systems(OnEnter(DemoState::Magnet), player::spawn_player)
            // Exiting DemoState:: ---------------------------------------------
            .add_systems(OnExit(DemoState::Home), player::despawn_player)
            .add_systems(
                OnExit(DemoState::Colliders),
                (player::despawn_player, setup_gravity),
            )
            .add_systems(OnExit(DemoState::ConveyorBelt), player::despawn_player)
            .add_systems(OnExit(DemoState::Magnet), player::despawn_player)
            // Update ----------------------------------------------------------
            .add_systems(
                Update,
                (
                    handle_edit_demo_state,
                    player::handle_player_input,
                    player::pass_through_one_way_platform,
                ),
            )
            // PostUpdate ------------------------------------------------------
            .add_systems(
                PostUpdate,
                camera::camera_follow_player
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            );
    }
}

// resources -------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Resource)]
pub struct GravityFactor {
    pub factor: f32,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
fn setup_gravity(mut gravity: ResMut<Gravity>, g_factor: Res<GravityFactor>) {
    gravity.0 = Vec2::NEG_Y * g_factor.factor;
}
