use bevy::{
    prelude::*,
    math::{vec2},
};
use rand::distributions::{Distribution, Uniform};

use crate::{BOTTOM_WALL, WALL_THICKNESS, TOP_WALL, PADDLE_SIZE, PADDLE_PADDING, PADDLE_SPEED, TIME_STEP, BALL_SIZE, BALL_STARTING_POSITION, BALL_COLOR, BALL_SPEED};

fn direction_mutate(
    dir: &mut f32,
    keyboard: &Res<Input<KeyCode>>,
    key_up: KeyCode,
    key_down: KeyCode,
) {
    if keyboard.pressed(key_up) {
        *dir += 1.0;
    }

    if keyboard.pressed(key_down) {
        *dir -= 1.0;
    }
}

pub fn move_paddle(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Controls), With<Paddle>>,
) {
    for (mut paddle_transform, controls) in query.iter_mut() 
    {
        let mut direction = 0.0;

        direction_mutate(
            &mut direction,
            &keyboard_input,
            controls.up,
            controls.down,
        );

        // Calculate the new horizontal paddle position based on player input
        let new_paddle_position = paddle_transform.translation.y + direction * PADDLE_SPEED * TIME_STEP;

        // Update the paddle position,
        // making sure it doesn't cause the paddle to leave the arena
        let bottom_bound = BOTTOM_WALL + WALL_THICKNESS / 2.0 + PADDLE_SIZE.y / 2.0 + PADDLE_PADDING;
        let top_bound = TOP_WALL - WALL_THICKNESS / 2.0 - PADDLE_SIZE.y / 2.0 - PADDLE_PADDING;

        paddle_transform.translation.y = new_paddle_position.clamp(bottom_bound, top_bound);
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP;
        transform.translation.y += velocity.y * TIME_STEP;
    }
}

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Controls {
    pub up: KeyCode,
    pub down: KeyCode,
}

pub fn new_ball(commands: &mut Commands)
{
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(-0.5..0.5);
    let ball_direction = vec2(
        uniform.sample(&mut rng),
        uniform.sample(&mut rng),
    );

    commands
        .spawn()
        .insert(Ball)
        .insert_bundle(SpriteBundle {
            transform: Transform {
                scale: BALL_SIZE,
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                ..default()
            },
            ..default()
        })
        .insert(Velocity(ball_direction.normalize() * BALL_SPEED));
}