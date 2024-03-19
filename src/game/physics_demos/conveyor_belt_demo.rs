use bevy::{prelude::*, utils::Duration};
use bevy_xpbd_2d::prelude::*;

// components ------------------------------------------------------------------
// -----------------------------------------------------------------------------
#[derive(Component)]
pub struct ConveyorBeltDemo;

#[derive(Component)]
pub struct ConveyorBelt {
    pub belt_vector: Vec2,
}

#[derive(Component)]
pub struct ConveyorBeltDemoTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct ConveyorBeltBlockTimer {
    pub timer: Timer,
}

// systems ---------------------------------------------------------------------
// -----------------------------------------------------------------------------
pub fn spawn_conveyor_belt_demo(mut commands: Commands) {
    commands.spawn((
        Name::new("ConveyorBottomWall"),
        ConveyorBeltDemo,
        RigidBody::Static,
        Collider::rectangle(180.0, 10.0),
        TransformBundle::from_transform(Transform::from_xyz(0.0, -50.0, 0.0)),
        DebugRender {
            axis_lengths: None,
            collider_color: Some(Color::WHITE),
            ..default()
        },
    ));

    commands
        .spawn((
            Name::new("ConveyorBeltBottom"),
            ConveyorBeltDemo,
            ConveyorBelt {
                belt_vector: Vec2::new(20.0, 0.0),
            },
            RigidBody::Static,
            Collider::round_rectangle(100.0, 2.0, 1.0),
            TransformBundle::from_transform(Transform::from_xyz(-50.0, -20.0, 0.0)),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("ConveyorBeltCasingBottom"),
                RigidBody::Static,
                Collider::rectangle(105.0, 5.0),
                TransformBundle::from_transform(Transform::from_xyz(0.0, -1.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
        });

    commands
        .spawn((
            Name::new("ConveyorBeltMiddle"),
            ConveyorBeltDemo,
            ConveyorBelt {
                belt_vector: Vec2::new(-20.0, 0.0),
            },
            RigidBody::Static,
            Collider::round_rectangle(100.0, 2.0, 1.0),
            TransformBundle::from_transform(Transform::from_xyz(40.0, 15.0, 0.0)),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("ConveyorBeltCasingMiddle"),
                RigidBody::Static,
                Collider::rectangle(105.0, 5.0),
                TransformBundle::from_transform(Transform::from_xyz(0.0, -1.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
        });

    commands
        .spawn((
            Name::new("ConveyorBeltTop"),
            ConveyorBeltDemo,
            ConveyorBelt {
                belt_vector: Vec2::new(20.0, 0.0),
            },
            RigidBody::Static,
            Collider::round_rectangle(100.0, 2.0, 1.0),
            TransformBundle::from_transform(Transform::from_xyz(-40.0, 50.0, 0.0)),
        ))
        .with_children(|children| {
            children.spawn((
                Name::new("ConveyorBeltCasingTop"),
                RigidBody::Static,
                Collider::rectangle(105.0, 5.0),
                TransformBundle::from_transform(Transform::from_xyz(0.0, -1.0, 0.0)),
                DebugRender::default().with_collider_color(Color::WHITE),
            ));
        });

    commands.spawn((
        Name::new("ConveyorRoundRectangle"),
        ConveyorBeltDemo,
        ConveyorBeltBlockTimer {
            timer: Timer::new(Duration::from_secs(20), TimerMode::Repeating),
        },
        RigidBody::Dynamic,
        Restitution::new(0.1),
        Collider::round_rectangle(8.0, 4.0, 1.5),
        TransformBundle::from_transform(Transform::from_xyz(-80.0, 70.0, 0.0)),
    ));

    commands.spawn((
        Name::new("ConveyorBeltDemoTimer"),
        ConveyorBeltDemo,
        ConveyorBeltDemoTimer {
            timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
        },
    ));
}

pub fn spawn_and_despawn_blocks(
    mut commands: Commands,
    time: Res<Time>,
    mut demo_timer_query: Query<&mut ConveyorBeltDemoTimer>,
    mut block_query: Query<(Entity, &mut ConveyorBeltBlockTimer)>,
) {
    let mut demo_timer = demo_timer_query.single_mut();
    demo_timer.timer.tick(time.delta());

    if demo_timer.timer.just_finished() {
        commands.spawn((
            Name::new("ConveyorRoundRectangle"),
            ConveyorBeltDemo,
            ConveyorBeltBlockTimer {
                timer: Timer::new(Duration::from_secs(20), TimerMode::Repeating),
            },
            RigidBody::Dynamic,
            Restitution::new(0.1),
            Collider::round_rectangle(8.0, 4.0, 1.5),
            TransformBundle::from_transform(Transform::from_xyz(-80.0, 70.0, 0.0)),
        ));
    }

    for (block_entity, mut block_timer) in block_query.iter_mut() {
        block_timer.timer.tick(time.delta());

        if block_timer.timer.just_finished() {
            commands.entity(block_entity).despawn();
        }
    }
}

pub fn despawn_conveyor_belt_demo(
    mut commands: Commands,
    demo_query: Query<Entity, With<ConveyorBeltDemo>>,
) {
    for demo_entity in demo_query.iter() {
        commands.entity(demo_entity).despawn_recursive();
    }
}

pub fn handle_belt_collisions(
    belt_query: Query<(&ConveyorBelt, &CollidingEntities)>,
    mut impulse_query: Query<&mut ExternalImpulse>,
) {
    for (belt, colliding_entities) in belt_query.iter() {
        for colliding_ent in colliding_entities.0.iter() {
            if let Ok(mut collider_impulse) = impulse_query.get_mut(*colliding_ent) {
                collider_impulse.set_impulse(belt.belt_vector);
            }
        }
    }
}
