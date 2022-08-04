use bevy::prelude::*;

use crate::{collision::Collider, Velocity};

const BALL_SIZE: f32 = 20.0;
const BALL_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Ball;

impl Ball {
    pub fn create(commands: &mut Commands) {
        let translation = Vec3::new(0.0, 0.0, 0.0);
        println!("translation {:?}", translation);

        let vel = Vec2 {
            x: BALL_SPEED * f32::cos(f32::to_radians(135.0)),
            y: BALL_SPEED * f32::sin(f32::to_radians(135.0)),
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
