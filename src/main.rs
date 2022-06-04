//! A simplified implementation of the classic game "Breakout"

use bevy::{
    core::FixedTimestep,
    math::{const_vec3},
    prelude::*,
};
use movement::{Paddle, Controls, new_ball};
use scoreboard::Scoreboard;
use collision::{CollisionEvent, CollisionSound, Collider, WallLocation, Side, WallBundle};

// Defines the amount of time that should elapse between each physics step.
const TIME_STEP: f32 = 1.0 / 60.0;

// These constants are defined in `Transform` units.
// Using the default 2D camera they correspond 1:1 with screen pixels.
// The `const_vec3!` macros are needed as functions that operate on floats cannot be constant in Rust.
const PADDLE_SIZE: Vec3 = const_vec3!([20.0, 120.0, 0.0]);
const GAP_BETWEEN_PADDLE_AND_WALL: f32 = 60.0;
const PADDLE_SPEED: f32 = 500.0;
// How close can the paddle get to the wall
const PADDLE_PADDING: f32 = 10.0;

// We set the z-value of the ball to 1 so it renders on top in the case of overlapping sprites.
const BALL_STARTING_POSITION: Vec3 = const_vec3!([0.0, -50.0, 1.0]);
const BALL_SIZE: Vec3 = const_vec3!([30.0, 30.0, 0.0]);
const BALL_SPEED: f32 = 400.0;

const WALL_THICKNESS: f32 = 10.0;
// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

pub mod collision;
pub mod scoreboard;
pub mod movement;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { left: 0, right: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(collision::check_for_collisions)
                .with_system(movement::move_paddle.before(collision::check_for_collisions))
                .with_system(movement::apply_velocity.before(collision::check_for_collisions))
                .with_system(collision::play_collision_sound.after(collision::check_for_collisions)),
        )
        .add_system(scoreboard::update_scoreboard)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

// Add the game's entities to our world
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Sound
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    // Left Paddle
    let left_paddle = LEFT_WALL + GAP_BETWEEN_PADDLE_AND_WALL;

    commands
        .spawn()
        .insert(Paddle)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(left_paddle, 0.0, 0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Collider)
        .insert(Controls {
            up: KeyCode::W,
            down: KeyCode::S,
        });

    // Ball
    new_ball(&mut commands);

    // Right Paddle
    let right_paddle = RIGHT_WALL - GAP_BETWEEN_PADDLE_AND_WALL;

    commands
        .spawn()
        .insert(Paddle)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(right_paddle, 0.0, 0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Collider)
        .insert(Controls {
            up: KeyCode::Up,
            down: KeyCode::Down,
        });

    // Scoreboard
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Left: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: TEXT_COLOR,
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: SCORE_COLOR,
                    },
                },
            ],
            ..default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        },
        ..default()
    }).insert(Side(WallLocation::Left));
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Right: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: TEXT_COLOR,
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: SCORE_COLOR,
                    },
                },
            ],
            ..default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: SCOREBOARD_TEXT_PADDING,
                right: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
            ..default()
        },
        ..default()
    }).insert(Side(WallLocation::Right));

    // Walls
    commands.spawn_bundle(WallBundle::new(WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(WallLocation::Bottom));
    commands.spawn_bundle(WallBundle::new(WallLocation::Top));
}
