use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use bevy_rapier2d::prelude::*;
use rand::Rng;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Rusty Bird".into(),
                    resolution: (300.0, 400.0).into(),
                    present_mode: PresentMode::AutoVsync,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0)))
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, (jump, move_walls, display_events, roof, calculate_score))
        .run();
}
#[derive(Component)]
struct ScoreText;
#[derive(Component)]
struct DeathMessage;
#[derive(Component)]
struct Player {
    score: i32,
    hops: i32,
    dead: bool,
    started: bool,
}
#[derive(Component)]
struct BottomWall;
#[derive(Component)]
struct TopWall;
fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn setup_physics(mut commands: Commands, asset_server: Res<AssetServer>) {

    let bird_sprite = asset_server.load("images/bird.png");

    let pipe_sprite = asset_server.load("images/pipe.png");

    let ground_sprite = asset_server.load("images/floor.png");

    let background_sprite = asset_server.load("images/background.png");

    commands.spawn(SpriteBundle {
        texture: background_sprite,
        ..Default::default()
    }).insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 100.0))
        .insert(SpriteBundle {sprite: Sprite {custom_size: Some(Vec2::new(400.0, 200.0)),  ..default()}, texture: ground_sprite.clone(), ..default()})
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -240.0, 2.0)))
        .insert(ActiveEvents::COLLISION_EVENTS);
    /* Create the bottom wall */
    commands
        .spawn(Collider::cuboid(20.0, 100.0))
        .insert(SpriteBundle {sprite: Sprite {custom_size: Some(Vec2::new(40.0, 200.0)), ..default()}, texture: pipe_sprite.clone(), ..default()})
        .insert(TransformBundle::from(Transform::from_xyz(200.0, -50.0, 1.0)))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(BottomWall);
    /* Create the top wall */
    commands
        .spawn(Collider::cuboid(20.0, 100.0))
        .insert(SpriteBundle {sprite: Sprite {custom_size: Some(Vec2::new(40.0, 200.0)), flip_y: true, ..default()}, texture: pipe_sprite.clone(), ..default()})
        .insert(TransformBundle::from(Transform::from_xyz(200.0, 250.0, 1.0)))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(TopWall);
    /* Create the bottom second wall */
    commands
        .spawn(Collider::cuboid(20.0, 100.0))
        .insert(SpriteBundle {sprite: Sprite {custom_size: Some(Vec2::new(40.0, 200.0)), ..default()}, texture: pipe_sprite.clone(), ..default()})
        .insert(TransformBundle::from(Transform::from_xyz(400.0, -50.0, 1.0)))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(BottomWall);
    /* Create the top second wall */
    commands
        .spawn(Collider::cuboid(20.0, 100.0))
        .insert(SpriteBundle {sprite: Sprite {custom_size: Some(Vec2::new(40.0, 200.0)), flip_y: true, ..default()}, texture: pipe_sprite.clone(), ..default()})
        .insert(TransformBundle::from(Transform::from_xyz(400.0, 250.0, 1.0)))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(TopWall);
    /* Create the bouncing ball */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(SpriteBundle {texture: bird_sprite.clone(), ..default()})
        .insert(Collider::ball(12.0))
        .insert(Restitution::coefficient(1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 100.0, 3.0)))
        .insert(Velocity {linvel: Vec2::new(0.0, 0.0), angvel: 0.0})
        .insert(GravityScale(20.0))
        .insert(Player {score: 0, hops: 0, dead: false, started: false});
    /* Create the score tab */
    commands.spawn((
        TextBundle::from_section(
            "Score",
            TextStyle {
                font: asset_server.load("fonts/SF-Pro-Display-Thin.otf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        }),
        ScoreText,
    ));
    commands.spawn((
        TextBundle::from_section(
            "Click space to jump!",
            TextStyle {
                font: asset_server.load("fonts/SF-Pro-Display-Thin.otf"),
                font_size: 25.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        DeathMessage,
    ));
}
fn jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity>,
    mut player: Query<&mut Player>,
) {
    let mut player = player.single_mut();
    let mut velocity = query.single_mut();
    if keyboard_input.just_pressed(KeyCode::Space) {
        player.started = true;

        if player.started && player.dead == false {
            velocity.linvel.y = 300.0;
            player.hops += 1;
        }
    }
    if player.started == false {
        velocity.linvel.y = 0.0;
    }
}
fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<&mut Player>,
    mut velocity_query: Query<&mut Velocity>,
) {
    let mut player = player_query.single_mut();
    let mut rng = rand::thread_rng();
    let mut vel = velocity_query.single_mut();
    for _collision in collision_events.iter() {
        println!("You hit an object!");
        vel.angvel = rng.gen_range(-5.0, 0.0);
        player.dead = true;
    }
}
fn move_walls(
    mut bottom_wall_query: Query<&mut Transform, (With<BottomWall>, Without<TopWall>)>,
    mut top_wall_query: Query<&mut Transform, (With<TopWall>, Without<BottomWall>)>,
    mut window: Query<&Window>,
    player: Query<&Player>,
) {
    let mut rng = rand::thread_rng();
    let pipe_height = rng.gen_range(-50.0, 50.0);
    let window = window.single_mut();
    if player.single().started == false {
        return;
    }
    for mut bottom_wall_transform in bottom_wall_query.iter_mut() {
        bottom_wall_transform.translation.x -= 1.0;
        if bottom_wall_transform.translation.x < -window.width() / 2.0 - 20.0 {
            bottom_wall_transform.translation.x = 400.0 - window.width() / 2.0;
            bottom_wall_transform.translation = Vec3::new(400.0 - window.width() / 2.0, pipe_height - 100.0, 1.0);
        }
    }
    for mut top_wall_transform in top_wall_query.iter_mut() {
        top_wall_transform.translation.x -= 1.0;
        if top_wall_transform.translation.x < -window.width() / 2.0 - 20.0 {
            top_wall_transform.translation = Vec3::new(400.0 - window.width() / 2.0, pipe_height + 175.0, 1.0);
        }
    }
}
fn roof(
    mut player_query: Query<(&Transform, &mut Player)>,
) {
    for (transform, mut player) in player_query.iter_mut() {
        if transform.translation.y >= 255.0 {
            println!("Player went above 255!");
            player.dead = true;
        }
    }
}
fn calculate_score (
    mut player_transform_query: Query<&Transform, (With<Player>, Without<TopWall>)>,
    wall_query: Query<&Transform, (With<TopWall>, Without<Player>)>,
    mut player: Query<&mut Player>,
    mut text_query: Query<&mut Text, With<ScoreText>>,
    mut death_message_query: Query<&mut Text, (With<DeathMessage>, Without<ScoreText>)>,
) {
    let mut player = player.single_mut();
    let mut text = text_query.single_mut();

    if player.dead {
        death_message_query.single_mut().sections[0].value = format!("You died! Restart the game to play again. Score: {}", player.score);
    }

    text.sections[0].value = format!("{}", player.score);
    for player_transform in player_transform_query.iter_mut() {
        for wall_transform in wall_query.iter() {
            if player_transform.translation.x == wall_transform.translation.x {
                player.score += 1;
                println!("Score is now: {}", player.score);
            }
        }
    }
}