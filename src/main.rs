mod ball;
mod collision;
mod paddle;
mod plugins;
mod wall;

use ball::Ball;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResizeConstraints},
};
use paddle::{Paddle, Player};
use wall::Wall;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Clone, Copy)]
pub enum Side {
    PLAYER,
    ENEMY,
}

pub struct ScoreEvent(Side);

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
        .add_event::<ScoreEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(plugins::fps::ScreenDiagsPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup)
        .add_system(paddle::move_paddle)
        .add_system(ball::ball_movement)
        .add_system(collision::handle_collisions)
        .add_system(handle_score)
        .run();
}

fn get_screen_dimensions(windows: &Res<Windows>) -> (f32, f32) {
    let (mut width, mut height) = (0.0f32, 0.0f32);

    for window in windows.iter() {
        (width, height) = (window.width(), window.height());
    }

    (width, height)
}

fn spawn_moveables(mut commands: &mut Commands, screen_width: f32, screen_height: f32) {
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let (mut width, mut height) = get_screen_dimensions(&windows);

    commands.spawn_bundle(Camera2dBundle::default());

    spawn_moveables(&mut commands, width, height);

    Wall::create_walls(&mut commands, width, height);
}

fn handle_score(
    mut commands: Commands,
    mut ev_score: EventReader<ScoreEvent>,
    mut paddle_query: Query<Entity, With<Paddle>>,
    mut ball_query: Query<Entity, With<Ball>>,
    windows: Res<Windows>,
) {
    if ev_score.is_empty() {
        return;
    }

    let (mut width, mut height) = get_screen_dimensions(&windows);

    commands.entity(ball_query.single()).despawn();

    for paddle in paddle_query.iter() {
        commands.entity(paddle).despawn();
    }

    spawn_moveables(&mut commands, width, height);
}
