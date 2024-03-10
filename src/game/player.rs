use crate::game::demo_state::EditDemoState;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

// -----------------------------------------------------------------------------
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Move,
    SpinCW,
    SpinCCW,
    Grab,
    NextDemo,
}

impl PlayerAction {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(Self::Move, VirtualDPad::wasd());
        input_map.insert(Self::SpinCW, KeyCode::ArrowRight);
        input_map.insert(Self::SpinCCW, KeyCode::ArrowLeft);
        input_map.insert(Self::Grab, KeyCode::KeyE);
        input_map.insert(Self::NextDemo, KeyCode::Space);

        input_map
    }
}

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Holder {
    pub is_holding: bool,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        Player,
        Holder { is_holding: false },
        InputManagerBundle::with_map(PlayerAction::default_input_map()),
        RigidBody::Dynamic,
        Collider::capsule(5.0, 5.0),
        GravityScale(0.0),
        LinearDamping(0.25),
        AngularDamping(0.25),
        DebugRender::default().with_collider_color(Color::RED),
    ));
}

pub fn handle_player_input(
    action_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut player_query: Query<(&mut ExternalImpulse, &mut ExternalAngularImpulse), With<Player>>,
    mut write_edit_demo: EventWriter<EditDemoState>,
) {
    let action_state = action_query.single();

    if action_state.just_pressed(&PlayerAction::Move) {
        let axis_pair = action_state
            .clamped_axis_pair(&PlayerAction::Move)
            .unwrap()
            .xy();

        let (mut player_impulse, _) = player_query.single_mut();
        player_impulse.apply_impulse(axis_pair * 2500.0);
    }

    if action_state.just_pressed(&PlayerAction::SpinCW) {
        let (_, mut player_impulse) = player_query.single_mut();
        player_impulse.apply_impulse(-5000.0);
    }
    if action_state.just_pressed(&PlayerAction::SpinCCW) {
        let (_, mut player_impulse) = player_query.single_mut();
        player_impulse.apply_impulse(5000.0);
    }

    if action_state.just_pressed(&PlayerAction::Grab) {
        println!("Grabbing");
    };

    if action_state.just_pressed(&PlayerAction::NextDemo) {
        write_edit_demo.send(EditDemoState);
    };
}
