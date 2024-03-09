use super::player;

use bevy::{
    core_pipeline::{
        bloom::{BloomPrefilterSettings, BloomSettings},
        tonemapping::Tonemapping,
    },
    prelude::*,
    render::{
        camera::{CameraOutputMode, ScalingMode},
        render_resource::{BlendState, LoadOp},
    },
};

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct DemoCamera;

// events ----------------------------------------------------------------------
// -----------------------------------------------------------------------------

#[derive(Event)]
pub struct CameraTranslation {
    pub position: Vec2,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        DemoCamera,
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                msaa_writeback: false,
                output_mode: CameraOutputMode::Write {
                    blend_state: Some(BlendState::ALPHA_BLENDING),
                    color_attachment_load_op: LoadOp::Load,
                },
                ..default()
            },
            projection: OrthographicProjection {
                near: -1.0,
                scaling_mode: ScalingMode::Fixed {
                    width: 256.0,
                    height: 144.0,
                },
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings {
            intensity: 0.1,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 1.0,
                threshold_softness: 0.0,
            },
            ..default()
        },
    ));
}

pub fn camera_follow_player(
    player_query: Query<&Transform, With<player::Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<player::Player>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        camera_transform.translation.z, // don't change the current depth (z) value
    );
}
