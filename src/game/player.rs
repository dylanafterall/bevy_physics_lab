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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component, Reflect)]
pub enum PassThroughOneWayPlatform {
    #[default]
    /// Passes through a `OneWayPlatform` if the contact normal is in line with the platform's local-space up vector
    ByNormal,
    /// Always passes through a `OneWayPlatform`, temporarily set this to allow an actor to jump down through a platform
    Always,
    /// Never passes through a `OneWayPlatform`
    Never,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        Player,
        Holder { is_holding: false },
        InputManagerBundle::with_map(PlayerAction::default_input_map()),
        PassThroughOneWayPlatform::ByNormal,
        RigidBody::Dynamic,
        Collider::capsule(5.0, 5.0),
        GravityScale(0.0),
        LinearDamping(0.75),
        AngularDamping(0.35),
        DebugRender::default().with_collider_color(Color::RED),
    ));
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    for player_entity in player_query.iter() {
        commands.entity(player_entity).despawn_recursive();
    }
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
        player_impulse.apply_impulse(axis_pair * 4000.0);
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

    if action_state.just_released(&PlayerAction::NextDemo) {
        write_edit_demo.send(EditDemoState);
    };
}

pub fn pass_through_one_way_platform(
    mut commands: Commands,
    action_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut player_query: Query<(Entity, &mut PassThroughOneWayPlatform), With<Player>>,
) {
    let (entity, mut pass_through_one_way_platform) = player_query.single_mut();

    let action_state = action_query.single();

    if action_state.pressed(&PlayerAction::Grab) {
        *pass_through_one_way_platform = PassThroughOneWayPlatform::Always;

        // Wake up body when it's allowed to drop down.
        // Otherwise it won't fall because gravity isn't simulated.
        commands.entity(entity).remove::<Sleeping>();
    } else {
        *pass_through_one_way_platform = PassThroughOneWayPlatform::ByNormal;
    };
}

// VARIOUS MEANS OF ACCESSING COLLISIONS ---------------------------------------
// -----------------------------------------------------------------------------
// get player collisions by querying for its CollidingEntities COMPONENT
pub fn _print_player_collisions(
    collision_query: Query<(Entity, &CollidingEntities), With<Player>>,
) {
    for (player_entity, colliding_entities) in &collision_query {
        println!(
            "{:?} is colliding with the following entities: {:?}",
            player_entity, colliding_entities
        );
    }
}

// can access ALL collisions via 3 EVENTS: Collision, CollisionStarted, CollisionEnded
pub fn _print_all_collisions(mut collision_event_reader: EventReader<Collision>) {
    for Collision(contacts) in collision_event_reader.read() {
        println!(
            "Entities {:?} and {:?} are colliding",
            contacts.entity1, contacts.entity2,
        );
    }
}

// get player collisions by searching the Collisions RESOURCE
pub fn _collisions_test(player_query: Query<Entity, With<Player>>, collisions: Res<Collisions>) {
    let player = player_query.single();
    for player_collision in collisions.collisions_with_entity(player) {
        println!("{:?}", player_collision);
    }
}

// PostProcessCollisions: schedule where you can add systems to filter or modify collisions using the Collisions resource
// ------------------------------------------------------
// .add_systems(PostProcessCollisions, filter_collisions)
// ------------------------------------------------------
// fn filter_collisions(mut collisions: ResMut<Collisions>, query: Query<(), With<Invulnerable>>) {
//     // Remove collisions where one of the colliders has an `Invulnerable` component.
//     // In a real project, this could be done more efficiently with collision layers.
//     collisions.retain(|contacts| {
//         !query.contains(contacts.entity1) && !query.contains(contacts.entity2)
//     });
// }
