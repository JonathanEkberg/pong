use bevy::prelude::*;

const MAX_SPEED: f32 = 400.0;
const ACCELERATION: f32 = MAX_SPEED / 20.0;

const PADDLE_WIDTH: f32 = 30.0;
const PADDLE_HEIGHT: f32 = 200.0;
const PADDLE_MARGIN: f32 = 200.0;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Collider;

#[derive(Component)]
pub struct YVelocity(pub f32);

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
        width: f32,
        height: f32,
    ) {
        let (sign, offset) = match position {
            PaddlePosition::RIGHT => (1.0f32, -80.0f32),
            PaddlePosition::LEFT => (-1.0f32, 0.0f32),
        };

        let translation = Vec3::new(
            sign * ((width + offset) / 2.0) + -sign * PADDLE_MARGIN,
            0.0,
            0.0,
        );
        println!("translation {:?}", translation);

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
            .insert(YVelocity(0.0));
    }
}

pub fn move_paddle(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut YVelocity), With<Player>>,
) {
    let delta = time.delta_seconds();
    let (mut trans, mut vel) = query.single_mut();

    let mut velocity = vel.0;

    let up_pressed = keyboard_input.pressed(KeyCode::W);
    let down_pressed = keyboard_input.pressed(KeyCode::S);

    if up_pressed || down_pressed {
        if up_pressed {
            velocity += ACCELERATION;
        }

        if down_pressed {
            velocity -= ACCELERATION;
        }
    } else {
        // Slowly decrease velocity
        if velocity != 0.0 {
            if velocity > 0.0 {
                velocity -= ACCELERATION;
                velocity = velocity.clamp(0.0, MAX_SPEED);
            } else {
                velocity += ACCELERATION;
                velocity = velocity.clamp(-MAX_SPEED, 0.0);
            }
        }
    }

    vel.0 = velocity.clamp(-MAX_SPEED, MAX_SPEED);

    trans.translation.y += vel.0 * delta;
}
