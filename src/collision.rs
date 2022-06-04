use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::{scoreboard::Scoreboard, movement::{Velocity, Ball, Paddle, new_ball}, LEFT_WALL, RIGHT_WALL, BOTTOM_WALL, TOP_WALL, WALL_THICKNESS, WALL_COLOR};

pub fn play_collision_sound(
    mut collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    sound: Res<CollisionSound>,
) {
    // Play a sound once per frame if a collision occurred. `count` consumes the
    // events, preventing them from triggering a sound on the next frame.
    if collision_events.iter().count() > 0 {
        audio.play(sound.0.clone());
    }
}

pub fn check_for_collisions(
    mut commands: Commands,
    mut scoreboard: ResMut<Scoreboard>,
    mut ball_query: Query<(Entity, &mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(&Transform, Option<&Paddle>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (entity, mut ball_velocity, ball_transform) in ball_query.iter_mut()
    {
        let ball_size = ball_transform.scale.truncate();

        // check collision with walls
        for (transform, paddle) in collider_query.iter() {
            let collision = collide(
                ball_transform.translation,
                ball_size,
                transform.translation,
                transform.scale.truncate(),
            );
            if let Some(collision) = collision {
                // Sends a collision event so that other systems can react to the collision
                collision_events.send_default();

                // Scoreboard should be increment when point
                // scoreboard.score += 1;

                // reflect the ball when it collides
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the ball's velocity is going in the opposite direction of the
                // collision
                match collision {
                    Collision::Left => {
                        if paddle.is_some() {
                            reflect_x = ball_velocity.x > 0.0;
                        } else {
                            commands.entity(entity).despawn();
                            new_ball(&mut commands);
                            scoreboard.right += 1;
                        }
                    },
                    Collision::Right => {
                        if paddle.is_some() {
                            reflect_x = ball_velocity.x < 0.0;
                        } else {
                            commands.entity(entity).despawn();
                            new_ball(&mut commands);
                            scoreboard.left += 1;
                        }
                    },
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => { /* do nothing */ }
                }

                // reflect velocity on the x-axis if we hit something on the x-axis
                if reflect_x {
                    ball_velocity.x = -ball_velocity.x;
                }

                // reflect velocity on the y-axis if we hit something on the y-axis
                if reflect_y {
                    ball_velocity.y = -ball_velocity.y;
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Collider;

#[derive(Default)]
pub struct CollisionEvent;

pub struct CollisionSound(pub Handle<AudioSource>);


#[derive(Component)]
pub struct Side(pub WallLocation);

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
pub struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
pub enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    pub fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    pub fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left => Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS),
            WallLocation::Right => Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS),
            WallLocation::Bottom => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
            WallLocation::Top => Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS),
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    pub fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
                    // This is used to determine the order of our sprites
                    translation: location.position().extend(0.0),
                    // The z-scale of 2D objects must always be 1.0,
                    // or their ordering will be affected in surprising ways.
                    // See https://github.com/bevyengine/bevy/issues/4149
                    scale: location.size().extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}