pub mod ai;
pub mod ball;
pub mod collision;
pub mod paddle;
pub mod score;
pub mod wall;

use bevy::prelude::*;

use crate::{util::get_screen_dimensions, AppState, Velocity};

use self::{
    ai::enemy_movement,
    ball::Ball,
    paddle::Paddle,
    score::{Score, ScoreEvent},
    wall::Wall,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .insert_resource(Score::default())
            .add_event::<ScoreEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(paddle::player_input)
                    .with_system(collision::handle_collisions.after(paddle::player_input))
                    .with_system(enemy_movement.after(collision::handle_collisions))
                    .with_system(move_moveables.after(enemy_movement))
                    .with_system(score::handle_score.after(move_moveables)),
            );
    }
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

    // commands.spawn_bundle(Camera2dBundle::default());
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
