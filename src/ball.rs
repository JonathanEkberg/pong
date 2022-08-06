use bevy::prelude::*;
use rand::{prelude::thread_rng, Rng};

use crate::{collision::Collider, Velocity};

const BALL_SIZE: f32 = 20.0;
pub const BALL_SPEED: f32 = 800.0;

struct BallAngle {
    lower: f32,
    upper: f32,
}

impl BallAngle {
    fn new(lower: f32, upper: f32) -> BallAngle {
        BallAngle {
            lower: f32::to_radians(lower),
            upper: f32::to_radians(upper),
        }
    }

    fn get_random_angle() -> f32 {
        const ANGLE_DIFF: f32 = 25.0;
        let mut rng = thread_rng();
        let mult = rng.gen_range(1u8..5);

        let range = match mult {
            1 => BallAngle::new(0. + ANGLE_DIFF, 90. - ANGLE_DIFF),
            2 => BallAngle::new(90. + ANGLE_DIFF, 180. - ANGLE_DIFF),
            3 => BallAngle::new(180. + ANGLE_DIFF, 270. - ANGLE_DIFF),
            4 => BallAngle::new(270. + ANGLE_DIFF, 360. - ANGLE_DIFF),
            _ => unreachable!(),
        };

        rng.gen_range(range.lower..=range.upper) as f32
    }
}

#[derive(Component)]
pub struct Ball;

impl Ball {
    pub fn create(commands: &mut Commands) {
        let translation = Vec3::new(0.0, 0.0, 0.0);

        let angle = BallAngle::get_random_angle();

        let vel = Vec2 {
            x: BALL_SPEED * f32::cos(angle),
            y: BALL_SPEED * f32::sin(angle),
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
