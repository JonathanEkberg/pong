use bevy::prelude::*;

use crate::{collision::Collider, Side, Velocity};

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Scoreable(pub Side);

#[derive(PartialEq, Eq)]
enum WallPosition {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

impl Wall {
    pub fn create_walls(commands: &mut Commands, screen_width: f32, screen_height: f32) {
        Wall::create(
            commands,
            WallPosition::TOP,
            screen_width,
            screen_height,
            false,
        );
        Wall::create(
            commands,
            WallPosition::BOTTOM,
            screen_width,
            screen_height,
            false,
        );

        Wall::create(
            commands,
            WallPosition::RIGHT,
            screen_width,
            screen_height,
            true,
        );
        Wall::create(
            commands,
            WallPosition::LEFT,
            screen_width,
            screen_height,
            true,
        );
    }

    fn create(
        commands: &mut Commands,
        position: WallPosition,
        screen_width: f32,
        screen_height: f32,
        scoreable: bool,
    ) {
        let (translation, scale) = Wall::calculate_wall(&position, screen_width, screen_height);

        let mut entity = commands.spawn();

        let color = if !scoreable {
            Color::rgb(1.0, 1.0, 1.0)
        } else {
            Color::NONE
        };

        entity
            .insert(Wall)
            .insert_bundle(SpriteBundle {
                transform: Transform {
                    translation,
                    scale,
                    ..default()
                },
                sprite: Sprite { color, ..default() },
                ..default()
            })
            .insert(Collider)
            .insert(Velocity(Vec2::default()));

        if scoreable {
            let score = if position == WallPosition::LEFT {
                Side::ENEMY
            } else {
                Side::PLAYER
            };
            entity.insert(Scoreable(score));
        }
    }

    /// Calculates wall translation and scale from position and screen dimensions.
    fn calculate_wall(
        position: &WallPosition,
        screen_width: f32,
        screen_height: f32,
    ) -> (Vec3, Vec3) {
        const WALL_SIZE: f32 = 20.0;
        const SCOREABLE_WALL_SIZE: f32 = 5.0;

        let w_half = screen_width / 2.0;
        let h_half = screen_height / 2.0;

        match position {
            WallPosition::TOP => {
                let translation = Vec3::new(0.0, h_half - WALL_SIZE / 2.0, 0.0);
                let scale = Vec3::new(screen_width, WALL_SIZE, 0.0);

                (translation, scale)
            }
            WallPosition::RIGHT => {
                let translation = Vec3::new(w_half - SCOREABLE_WALL_SIZE, 0.0, 0.0);
                let scale = Vec3::new(SCOREABLE_WALL_SIZE, screen_height, 0.0);

                (translation, scale)
            }
            WallPosition::BOTTOM => {
                let translation = Vec3::new(0.0, -h_half + WALL_SIZE / 2.0, 0.0);
                let scale = Vec3::new(screen_width, WALL_SIZE, 0.0);

                (translation, scale)
            }
            WallPosition::LEFT => {
                let translation = Vec3::new((-w_half) + SCOREABLE_WALL_SIZE, 0.0, 0.0);
                let scale = Vec3::new(SCOREABLE_WALL_SIZE, screen_height, 0.0);

                (translation, scale)
            }
        }
    }
}
