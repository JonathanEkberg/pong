use bevy::prelude::*;

use crate::collision::Collider;
use crate::Velocity;

const MAX_SPEED: f32 = 400.0;
const ACCELERATION: f32 = MAX_SPEED / 20.0;

const PADDLE_WIDTH: f32 = 30.0;
const PADDLE_HEIGHT: f32 = 200.0;
const PADDLE_MARGIN: f32 = 200.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Paddle;

#[derive(PartialEq)]
pub enum PaddlePosition {
    LEFT,
    RIGHT,
}

impl Paddle {
    pub fn create(
        commands: &mut Commands,
        position: PaddlePosition,
        player: bool,
        screen_width: f32,
    ) {
        let (sign, offset) = match position {
            PaddlePosition::RIGHT => (1.0f32, -80.0f32),
            PaddlePosition::LEFT => (-1.0f32, 0.0f32),
        };

        let translation = Vec3::new(
            sign * ((screen_width + offset) / 2.0) + -sign * PADDLE_MARGIN,
            0.0,
            0.0,
        );

        let mut entity = commands.spawn();

        if player == true {
            entity.insert(Player);
        }

        entity
            .insert(Paddle)
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    translation,
                    scale: Vec3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Collider)
            .insert(Velocity(Vec2::default()));
    }
}

pub fn move_paddle(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let delta = time.delta_seconds();
    let (mut trans, mut vel) = query.single_mut();

    let mut y_velocity = vel.0.y;

    let up_pressed = keyboard_input.pressed(KeyCode::W);
    let down_pressed = keyboard_input.pressed(KeyCode::S);

    if up_pressed || down_pressed {
        if up_pressed {
            y_velocity += ACCELERATION;
        }

        if down_pressed {
            y_velocity -= ACCELERATION;
        }
    } else {
        // Slowly decrease velocity
        if y_velocity != 0.0 {
            if y_velocity > 0.0 {
                y_velocity -= ACCELERATION;
                y_velocity = y_velocity.clamp(0.0, MAX_SPEED);
            } else {
                y_velocity += ACCELERATION;
                y_velocity = y_velocity.clamp(-MAX_SPEED, 0.0);
            }
        }
    }

    vel.0.y = y_velocity.clamp(-MAX_SPEED, MAX_SPEED);

    trans.translation.y += vel.0.y * delta;
}
