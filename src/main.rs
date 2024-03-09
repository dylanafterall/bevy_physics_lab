mod game;

use game::game_plugin;

use bevy::{log::LogPlugin, prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

use std::error::Error;

// MAIN ------------------------------------------------------------------------
// -----------------------------------------------------------------------------
fn main() -> Result<(), Box<dyn Error>> {
    App::new()
        // bevy plugins
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::Windowed,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    #[cfg(debug_assertions)]
                    level: bevy::log::Level::DEBUG,
                    #[cfg(not(debug_assertions))]
                    level: bevy::log::Level::INFO,
                    ..default()
                }),
        )
        // non-bevy external plugins
        .add_plugins((
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
            WorldInspectorPlugin::new(),
        ))
        // local plugins
        .add_plugins((game_plugin::GamePlugin,))
        .run();

    Ok(())
}
