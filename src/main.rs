use bevy::{
    prelude::*
};

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Collider;

fn main() {
    App::new().add_plugins(DefaultPlugins).insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0))).add_startup_system(setup).run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    
    commands.spawn().insert(Paddle).insert_bundle(SpriteBundle {
        transform: Transform { translation: Vec3::new(0.0, 0.0, 0.0), scale: Vec3::new(120.0, 360.0, 0.0), ..default() },
        sprite: Sprite {color: Color::rgb(1.0, 1.0, 1.0), ..default()},
        ..default()
    }).insert(Collider);
}