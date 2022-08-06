use bevy::prelude::*;

use crate::{
    ball::Ball,
    paddle::{Paddle, Player, PADDLE_MAX_SPEED},
    util, Velocity,
};

const DIFFICULTY: Difficulty = Difficulty::LOW;

enum Difficulty {
    HARD,
    MEDIUM,
    LOW,
}

pub fn enemy_movement(
    windows: Res<Windows>,
    mut enemy_query: Query<(&mut Velocity, &Transform), (With<Paddle>, Without<Player>)>,
    ball_query: Query<&Transform, With<Ball>>,
) {
    let (width, _) = util::get_screen_dimensions(&windows);
    let (mut enemy_vel, enemy_trans) = enemy_query.single_mut();
    let ball_trans = ball_query.single();

    let ball_distance = ball_trans.translation.distance(enemy_trans.translation);

    let (half, third, quarter) = (width / 2.0, width / 3.0, width / 4.0);

    let ball_visible = match DIFFICULTY {
        Difficulty::HARD => ball_distance < half,
        Difficulty::MEDIUM => ball_distance < third,
        Difficulty::LOW => ball_distance < quarter,
    };

    let ball_x = ball_trans.translation.x;
    let enemy_x = enemy_trans.translation.x;

    if !ball_visible || ball_x > enemy_x + (enemy_trans.scale.x / 1.8) {
        return enemy_vel.0.y = 0.;
    }

    let ball_y = ball_trans.translation.y;
    let enemy_y = enemy_trans.translation.y;

    if ball_y > enemy_y {
        return enemy_vel.0.y = PADDLE_MAX_SPEED;
    }

    if ball_y < enemy_y {
        return enemy_vel.0.y = -PADDLE_MAX_SPEED;
    }

    enemy_vel.0.y = 0.;
}
