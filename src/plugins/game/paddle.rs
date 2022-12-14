use bevy::prelude::*;

use crate::{util, Velocity};

use super::collision::Collider;

pub const PADDLE_MAX_SPEED: f32 = 500.0;
pub const PADDLE_ACCELERATION: f32 = PADDLE_MAX_SPEED / 20.0;

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

pub fn player_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let delta = time.delta_seconds();
    let mut vel = query.single_mut();

    let up_pressed = keyboard_input.pressed(KeyCode::W);
    let down_pressed = keyboard_input.pressed(KeyCode::S);

    vel.0.y = util::lerp(
        vel.0.y,
        if up_pressed { PADDLE_MAX_SPEED } else { 0. }
            + if down_pressed { -PADDLE_MAX_SPEED } else { 0. },
        PADDLE_ACCELERATION * delta,
    );
}
