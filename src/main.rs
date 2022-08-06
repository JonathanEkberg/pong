mod font;
mod plugins;
mod util;

use bevy::{
    app::AppExit,
    prelude::*,
    window::{PresentMode, WindowMode, WindowResizeConstraints},
};

use plugins::game::{ball::Ball, GamePlugin};
use plugins::menu::MenuPlugin;
use util::get_screen_dimensions;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Menu,
    Game,
}

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
        .add_state(AppState::Menu)
        .add_startup_system(setup)
        .add_system(exit_on_escape)
        .add_plugins(DefaultPlugins)
        // Fps counter
        .add_plugin(plugins::fps::ScreenDiagsPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn exit_on_escape(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
