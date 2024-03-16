use bevy::prelude::*;

// states ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum DemoState {
    #[default]
    Home,
    Colliders,
    ConveyorBelt,
    Magnet,
    Destructible,
}

// events ----------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Event)]
pub struct EditDemoState;

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn handle_edit_demo_state(
    current_demo_state: Res<State<DemoState>>,
    mut next_demo_state: ResMut<NextState<DemoState>>,
    mut read_edit_demo_state: EventReader<EditDemoState>,
) {
    for _ in read_edit_demo_state.read() {
        match *current_demo_state.get() {
            DemoState::Home => next_demo_state.set(DemoState::Colliders),
            DemoState::Colliders => next_demo_state.set(DemoState::ConveyorBelt),
            DemoState::ConveyorBelt => next_demo_state.set(DemoState::Magnet),
            DemoState::Magnet => next_demo_state.set(DemoState::Destructible),
            DemoState::Destructible => next_demo_state.set(DemoState::Home),
        }
    }
}
