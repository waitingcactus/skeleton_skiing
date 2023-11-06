use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

// world
const GRAVITY: Vec2 = Vec2::new(0.0, -200.);

// skeleton
const SKELETON_STARTING_POS: Vec3 = Vec3::new(-50.0, 0., 0.);
const SKELETON_SIZE: Vec2 = Vec2::new(50.0, 100.0);
const SKELETON_WALK_SPEED: f32 = 200.0;

// floor
const FLOOR_POS: Vec3 = Vec3::new(0., -350.0, 0.);
const FLOOR_SIZE: Vec2 = Vec2::new(1000.0, 25.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.95, 0.95, 0.95)))
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_player, player_jump, jump_reset))
        .insert_resource(RapierConfiguration {
            gravity: GRAVITY,
            ..Default::default()
        })
        .run();
}

#[derive(Component)]
struct Player {
    jump_power: f32,
    is_jumping: bool,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            jump_power: 1.0,
            is_jumping: false,
        }
    }
}

#[derive(Component)]
struct Skeleton;

#[derive(Component)]
struct Slope;

#[derive(Component)]
struct Floor;

#[derive(Component)]
struct Ramp;

#[derive(Component)]
struct Skis;

fn setup(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());

    // floor
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(FLOOR_SIZE),
            ..default()
        },
        transform: Transform::from_translation(FLOOR_POS),
        ..default()
        },
        Floor,
    ))
    .insert(Collider::cuboid(FLOOR_SIZE.x*0.5, FLOOR_SIZE.y*0.5));

    // skeleton
    let player = commands.spawn(RigidBody::Dynamic)
    .insert((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(SKELETON_SIZE),
            ..default()
        },
        transform: Transform::from_translation(SKELETON_STARTING_POS),
        ..default()
        },
        Skeleton,
        Player::default(),
        Collider::cuboid(SKELETON_SIZE.x*0.5, SKELETON_SIZE.y*0.5),
        Velocity::zero(),
        ActiveEvents::COLLISION_EVENTS
    )).id();

    // skis
    let skis = commands.spawn(RigidBody::Dynamic)
    .insert((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            ..default()
        },
        transform: Transform::from_translation(SKELETON_STARTING_POS),
        ..default()
        },
        Skis,
        Collider::cuboid(SKELETON_SIZE.x*0.25, SKELETON_SIZE.y*0.25),
    )).id();

    commands.entity(player).push_children(&[skis]);
    
}

fn move_player(input: Res<Input<KeyCode>>,
    time_step: Res<FixedTime>,
    mut transform: Query<&mut Transform, With<Player>>,
    mut velocity: Query<&mut Velocity, With<Player>>,
    mut player: Query<&mut Player>,
) {
    let mut player_transform = transform.single_mut();
    let mut player_velocity = velocity.single_mut();
    let mut player = player.single_mut();
    let mut direction = 0.0;
    let mut ang_direction = 0.0;

    if input.pressed(KeyCode::A) {
        direction -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        direction += 1.0;
    }
    if input.pressed(KeyCode::E) {
        ang_direction += 0.25;
    }
    if input.pressed(KeyCode::Q) {
        ang_direction -= 0.25;
    }

    let new_x = 
    player_transform.translation.x + direction * SKELETON_WALK_SPEED * time_step.period.as_secs_f32();

    player_transform.translation.x = new_x;

    if player_velocity.angvel < 5.0 && player_velocity.angvel > -5.0 {
        player_velocity.angvel += ang_direction;
    }
    
}

fn player_jump(
    input: Res<Input<KeyCode>>,
    mut player: Query<&mut Player>,
    mut velocity: Query<&mut Velocity, With<Player>>,
) {
    let mut player = player.single_mut();
    let mut velocity = velocity.single_mut();

    if input.pressed(KeyCode::Space) && !player.is_jumping {
        velocity.linvel = Vec2::new(0.0, 200.0 * player.jump_power);
        player.is_jumping = true;
    }
}

fn jump_reset(
    mut query: Query<(Entity, &mut Player)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        for (entity, mut player) in query.iter_mut() {
            if let CollisionEvent::Started(h1, h2, _) = collision_event {
                if *h1 == entity || *h2 == entity {
                    player.is_jumping = false;
                }
            }
        }
    }
}

fn rotate_player() {
    todo!();
}