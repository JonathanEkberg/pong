use bevy::prelude::*;

use crate::{ball::Ball, paddle::Paddle, spawn_moveables, ScoreboardTextTag, Side};

pub struct ScoreEvent(pub Side);

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

pub fn handle_score(
    mut commands: Commands,
    mut ev_score: EventReader<ScoreEvent>,
    paddle_query: Query<Entity, With<Paddle>>,
    ball_query: Query<Entity, With<Ball>>,
    mut score: ResMut<Score>,
    mut text_query: Query<&mut Text, With<ScoreboardTextTag>>,
    windows: Res<Windows>,
) {
    let mut text = text_query.single_mut();
    text.sections[0].value = format!("{}  -  {}", score.player, score.enemy);

    if ev_score.is_empty() {
        return;
    }

    let (width, _) = crate::get_screen_dimensions(&windows);

    commands.entity(ball_query.single()).despawn();

    for paddle in paddle_query.iter() {
        commands.entity(paddle).despawn();
    }

    spawn_moveables(&mut commands, width);

    for score_event in ev_score.iter() {
        if score_event.0 == Side::ENEMY {
            score.enemy += 1;
        } else {
            score.player += 1;
        }
    }
}

pub fn spawn_scoreboard_text(commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
