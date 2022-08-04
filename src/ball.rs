use bevy::prelude::*;
use rand::{prelude::thread_rng, Rng};

use crate::{collision::Collider, Velocity};

const BALL_SIZE: f32 = 20.0;
pub const BALL_SPEED: f32 = 800.0;

#[derive(Component)]
pub struct Ball;

impl Ball {
    pub fn create(commands: &mut Commands) {
        let translation = Vec3::new(0.0, 0.0, 0.0);
        println!("translation {:?}", translation);

        let mut rng = thread_rng();
        let angle = rng.gen_range(135..225) as f32;
        let sign = if rng.gen_range(0.0..=1.0) > 0.5 {
            1.0
        } else {
            -1.0
        };

        let vel = Vec2 {
            x: BALL_SPEED * f32::cos(f32::to_radians(sign * angle)),
            y: BALL_SPEED * f32::sin(f32::to_radians(sign * angle)),
        };

        commands
            .spawn()
            .insert(Ball)
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    translation,
                    scale: Vec3::new(BALL_SIZE, BALL_SIZE, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Collider)
            .insert(Velocity(vel));
    }
}
