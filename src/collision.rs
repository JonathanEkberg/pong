use crate::{
    score::{self, ScoreEvent},
    wall::Scoreable,
    Ball, Side, Velocity,
};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

#[derive(Component)]
pub struct Collider;

pub fn handle_collisions(
    time: Res<Time>,
    mut ev_score: EventWriter<ScoreEvent>,
    mut ball_query: Query<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<(&Transform, Option<&Scoreable>), (With<Collider>, Without<Ball>)>,
) {
    let delta = time.delta_seconds();
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();

    for (transform, scoreable) in collider_query.iter() {
        if let Some(collision) = collide(
            ball_transform.translation,
            ball_transform.scale.truncate(),
            transform.translation,
            transform.scale.truncate(),
        ) {
            if let Some(score) = scoreable {
                ev_score.send(score::ScoreEvent(score.0));
            }

            let mut flip_y = false;
            let mut flip_x = false;

            match collision {
                Collision::Top => flip_y = ball_velocity.0.y < 0.0,
                Collision::Right => flip_x = ball_velocity.0.x < 0.0,
                Collision::Bottom => flip_y = ball_velocity.0.y > 0.0,
                Collision::Left => flip_x = ball_velocity.0.x > 0.0,
                Collision::Inside => println!("Inside collision!"),
            }

            if flip_x {
                ball_velocity.0.x = -ball_velocity.0.x;
            }

            if flip_y {
                ball_velocity.0.y = -ball_velocity.0.y;
            }
        }
    }
}
