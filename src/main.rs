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

#[derive(Component)]
pub struct Score {
    pub player: usize,
    pub enemy: usize,
}

impl Default for Score {
    fn default() -> Self {
        Score {
            player: 0,
            enemy: 0,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
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
                .with_system(handle_score.after(collision::handle_collisions)),
        )
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

#[derive(Component)]
struct ScoreboardTextTag;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let (mut width, mut height) = get_screen_dimensions(&windows);

    commands.spawn_bundle(Camera2dBundle::default());

    spawn_moveables(&mut commands, width, height);

    Wall::create_walls(&mut commands, width, height);

    commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(
                    TextBundle::from_sections([TextSection::new(
                        "0  -  0",
                        TextStyle {
                            font_size: 64.0,
                            font: asset_server.load("fonts/JetBrainsMono-Medium.ttf"),
                            ..default()
                        },
                    )])
                    .with_style(Style {
                        margin: UiRect::new(Val::Auto, Val::Auto, Val::Percent(5.0), Val::Auto),
                        ..default()
                    }),
                )
                .insert(ScoreboardTextTag);
        });
}

fn handle_score(
    mut commands: Commands,
    mut ev_score: EventReader<ScoreEvent>,
    mut paddle_query: Query<Entity, With<Paddle>>,
    mut ball_query: Query<Entity, With<Ball>>,
    mut score: ResMut<Score>,
    mut text_query: Query<&mut Text, With<ScoreboardTextTag>>,
    windows: Res<Windows>,
) {
    let mut text = text_query.single_mut();
    text.sections[0].value = format!("{}  -  {}", score.player, score.enemy);

    if ev_score.is_empty() {
        return;
    }

    let (mut width, mut height) = get_screen_dimensions(&windows);

    commands.entity(ball_query.single()).despawn();

    for paddle in paddle_query.iter() {
        commands.entity(paddle).despawn();
    }

    spawn_moveables(&mut commands, width, height);

    for score_event in ev_score.iter() {
        if score_event.0 == Side::ENEMY {
            score.enemy += 1;
        } else {
            score.player += 1;
        }
    }
}
