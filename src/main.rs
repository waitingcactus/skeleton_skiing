use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// world
const GRAVITY: Vec2 = Vec2::new(0.0, -200.);
const SKY_COLOR: Color = Color::rgb(0.561, 0.933, 1.0);

// skeleton
const SKELETON_STARTING_POS: Vec3 = Vec3::new(-50.0, 0., 0.);
const SKELETON_SIZE: Vec2 = Vec2::new(50.0, 100.0);
const SKELETON_WALK_SPEED: f32 = 5.0;

// floor
const FLOOR_POS: Vec3 = Vec3::new(0., -350.0, 0.);
const FLOOR_SIZE: Vec2 = Vec2::new(1000.0, 25.0);

// slope
const SLOPE_POS: Vec3 = Vec3::new(4880.0, -2737.0, 0.);
const SLOPE_SIZE: Vec2 = Vec2::new(10000.0, 25.0);
const SLOPE_COLOR: Color = Color::rgb(0.94, 0.94, 0.94);

fn main() {
    App::new()
        .insert_resource(ClearColor(SKY_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
            move_player, 
            player_jump_ski, 
            jump_reset, 
            camera_follow_player,
            player_camera_control,
            rotate_player,
        ))
        .insert_resource(RapierConfiguration {
            gravity: GRAVITY,
            ..Default::default()
        })
        .run();
}

// COMPONENTS

#[derive(Component)]
struct Player {
    jump_power: f32,
    is_jumping: bool,
    is_skiing: bool,
    direction: f32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            jump_power: 1.0,
            is_jumping: false,
            is_skiing: false,
            direction: 1.0,
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
    // camera
    commands.spawn(Camera2dBundle::default());

    // floor
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: SLOPE_COLOR,
            custom_size: Some(FLOOR_SIZE),
            ..default()
        },
        transform: Transform::from_translation(FLOOR_POS),
        ..default()
        },
        Floor,
    ))
    .insert(Collider::cuboid(FLOOR_SIZE.x*0.5, FLOOR_SIZE.y*0.5));

    // slope
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: SLOPE_COLOR,
            custom_size: Some(SLOPE_SIZE),
            ..default()
        },
        transform: Transform {
            translation: SLOPE_POS,
            rotation: Quat::from_rotation_z(-0.5),
            ..default()
        },
        ..default()
        },
        Floor,
    ))
    .insert((Collider::cuboid(SLOPE_SIZE.x*0.5, SLOPE_SIZE.y*0.5),
    Friction::coefficient(0.01),

    ));

    // skeleton
    let _player = commands.spawn(Player::default())
    .insert((SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(SKELETON_SIZE),
            ..default()
        },
        transform: Transform::from_translation(SKELETON_STARTING_POS),
        ..default()
        },
        RigidBody::Dynamic,
        Skeleton,
        Collider::cuboid(SKELETON_SIZE.x*0.5, SKELETON_SIZE.y*0.5),
        Velocity::zero(),
        ActiveEvents::COLLISION_EVENTS
    )).id();

    // skis
    // let skis = commands.spawn(RigidBody::Dynamic)
    // .insert((SpriteBundle {
    //     sprite: Sprite {
    //         color: Color::rgb(0.25, 0.25, 0.75),
    //         ..default()
    //     },
    //     transform: Transform::from_translation(SKELETON_STARTING_POS),
    //     ..default()
    //     },
    //     Skis,
    //     Collider::cuboid(SKELETON_SIZE.x*2., SKELETON_SIZE.y*0.25),
    // )).id();

    // commands.entity(player).push_children(&[skis]);
    
}

fn move_player(input: Res<Input<KeyCode>>,
    mut player: Query<&mut Player>,
    mut transform: Query<&mut Transform, With<Player>>,
) {
    let mut player = player.single_mut();
    let mut transform = transform.single_mut();

    if input.pressed(KeyCode::A) {
        if !player.is_skiing {
            transform.translation.x -= SKELETON_WALK_SPEED;

        }
        else if player.is_skiing && !player.is_jumping {
            player.direction = -1.0;
        }
        
    }
    if input.pressed(KeyCode::D) {
        if !player.is_skiing {
            transform.translation.x += SKELETON_WALK_SPEED;
        }
        else if player.is_skiing && !player.is_jumping {
            player.direction = 1.0;
        }
        
    }
    
}

fn player_jump_ski(
    input: Res<Input<KeyCode>>,
    mut player: Query<&Player>,
    mut velocity: Query<&mut Velocity, With<Player>>,
) {
    let player = player.single_mut();
    let mut velocity = velocity.single_mut();

    if !player.is_jumping {
        if input.pressed(KeyCode::Space) {
            let new_x = if player.is_skiing { velocity.linvel.x } else { 0.0 };
            velocity.linvel = Vec2::new(new_x, 200.0 * player.jump_power);
        }
        if input.pressed(KeyCode::S) && player.is_skiing {
            velocity.linvel = Vec2::new(velocity.linvel.x + (20.0 * player.direction), velocity.linvel.y);
        }
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
            if let CollisionEvent::Stopped(h1, h2, _) = collision_event {
                if *h1 == entity || *h2 == entity {
                    player.is_jumping = true;
                }
            }
        }
    }
}

fn rotate_player(
    input: Res<Input<KeyCode>>,
    mut velocity: Query<&mut Velocity, With<Player>>,
) {
    let mut player_velocity = velocity.single_mut();

    if player_velocity.angvel != 0.0 {
        player_velocity.angvel *= 0.98;
    }

    if input.pressed(KeyCode::E) {
        if player_velocity.angvel < 5.0 {
            player_velocity.angvel += 0.25;
        }  
    }
    if input.pressed(KeyCode::Q) {
        if player_velocity.angvel > -5.0 {
            player_velocity.angvel -= 0.25;
        }  
    }
}

fn camera_follow_player(
    mut cameras: Query<&mut Transform, With<Camera>>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let player = player.single();
    for mut camera in cameras.iter_mut() {
        camera.translation.x = player.translation.x;
        camera.translation.y = player.translation.y;
    }
}

fn player_camera_control(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for mut projection in query.iter_mut() {
        let mut log_scale = projection.scale.ln();

        if input.pressed(KeyCode::PageUp) {
            log_scale -= 0.1;
        }
        if input.pressed(KeyCode::PageDown) {
            log_scale += 0.1;
        }

        projection.scale = log_scale.exp();
    }
}