mod ball;
mod collision;
mod paddle;
mod plugins;
mod score;
mod wall;

use ball::Ball;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResizeConstraints},
};
use paddle::Paddle;
use score::{Score, ScoreEvent};
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
                .with_system(paddle::move_paddle)
                .with_system(ball::ball_movement.after(paddle::move_paddle))
                .with_system(collision::handle_collisions.after(ball::ball_movement))
                .with_system(score::handle_score.after(collision::handle_collisions)),
        )
        .run();
}

fn get_screen_dimensions(windows: &Res<Windows>) -> (f32, f32) {
    let (mut width, mut height) = (0., 0.);

    for window in windows.iter() {
        (width, height) = (window.width(), window.height());
    }

    (width, height)
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
