mod paddle;
mod plugins;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResizeConstraints},
};
use paddle::Paddle;

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
        .add_plugin(plugins::fps::ScreenDiagsPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_startup_system(setup)
        .add_system(paddle::move_paddle)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let (mut width, mut height) = (0.0f32, 0.0f32);

    for window in windows.iter() {
        (width, height) = (window.width(), window.height());
        println!("width: {:?}, height: {:?}", width, height);
    }

    commands.spawn_bundle(Camera2dBundle::default());

    Paddle::create(
        &mut commands,
        paddle::PaddlePosition::LEFT,
        true,
        width,
        height,
    );
    Paddle::create(
        &mut commands,
        paddle::PaddlePosition::RIGHT,
        false,
        width,
        height,
    );
}
