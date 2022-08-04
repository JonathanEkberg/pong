mod ai;
mod ball;
mod collision;
mod paddle;
mod plugins;
mod score;
mod util;
mod wall;

use ai::enemy_movement;
use ball::Ball;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResizeConstraints},
};
use paddle::Paddle;
use score::{Score, ScoreEvent};
use util::get_screen_dimensions;
use wall::Wall;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Side {
    PLAYER,
    ENEMY,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            present_mode: PresentMode::AutoVsync,
            mode: WindowMode::BorderlessFullscreen,
            resizable: true,
            resize_constraints: WindowResizeConstraints {
                min_width: 800.0,
                min_height: 450.0,
                ..default()
            },
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Score::default())
        .add_startup_system(setup)
        .add_plugin(plugins::fps::ScreenDiagsPlugin)
        .add_event::<ScoreEvent>()
        .add_system_set(
            SystemSet::new()
                .with_system(paddle::player_input)
                // .with_system(ball::ball_movement.after(paddle::player_input))
                .with_system(collision::handle_collisions.after(paddle::player_input))
                .with_system(enemy_movement.after(collision::handle_collisions))
                .with_system(move_moveables.after(enemy_movement))
                .with_system(score::handle_score.after(move_moveables)),
        )
        .run();
}

fn spawn_moveables(mut commands: &mut Commands, screen_width: f32) {
    Paddle::create(
        &mut commands,
        paddle::PaddlePosition::LEFT,
        true,
        screen_width,
    );
    Paddle::create(
        &mut commands,
        paddle::PaddlePosition::RIGHT,
        false,
        screen_width,
    );

    Ball::create(&mut commands);
}

#[derive(Component)]
pub struct ScoreboardTextTag;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let (width, height) = get_screen_dimensions(&windows);

    commands.spawn_bundle(Camera2dBundle::default());
    spawn_moveables(&mut commands, width);
    Wall::create_walls(&mut commands, width, height);
    score::spawn_scoreboard_text(&mut commands, &asset_server);
}

/// Applies all entities Velocity component values to its Transform component.
fn move_moveables(
    time: Res<Time>,
    mut moveable_query: Query<(&mut Transform, &Velocity), With<Velocity>>,
) {
    let delta = time.delta_seconds();
    for (mut transform, velocity) in moveable_query.iter_mut() {
        transform.translation.x += velocity.0.x * delta;
        transform.translation.y += velocity.0.y * delta;
    }
}
