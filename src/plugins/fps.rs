use std::fmt::Write;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    utils::Duration,
};

const FONT_SIZE: f32 = 32.0;
const FONT_COLOR: Color = Color::WHITE;
const UPDATE_INTERVAL: Duration = Duration::from_millis(250);

const STRING_FORMAT: &str = "FPS: ";
const STRING_INITIAL: &str = "FPS: ...";
const STRING_MISSING: &str = "FPS: ???";

/// A plugin that draws diagnostics on-screen with Bevy UI.
/// Currently only the FPS is displayed.
pub struct ScreenDiagsPlugin;

impl Plugin for ScreenDiagsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(spawn_text)
            .add_system(update)
            .init_resource::<ScreenDiagsState>();
    }
}

/// The diagnostics state resource.
///
/// To disable the FPS counter, get a [ResMut](bevy::prelude::ResMut) reference to this struct and
/// pause the timer. Unpause the timer to re-enable the counter.
pub struct ScreenDiagsState {
    /// The timer that triggers a diagnostics reading.
    /// Public, to allow flexible use, but in general use the methods to interact.
    pub timer: Timer,
    /// A flag to indicate to update the display, even if the timer has not popped.
    /// Public, to allow flexible use, but in general use the methods to interact.
    pub update_now: bool,
}

impl Default for ScreenDiagsState {
    fn default() -> Self {
        Self {
            timer: Timer::new(UPDATE_INTERVAL, true),
            update_now: false,
        }
    }
}

/// The marker on the text to be updated.
#[derive(Component)]
pub struct ScreenDiagsText;

fn update(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    state_resource: Option<ResMut<ScreenDiagsState>>,
    mut text_query: Query<&mut Text, With<ScreenDiagsText>>,
) {
    if let Some(mut state) = state_resource {
        if state.update_now || state.timer.tick(time.delta()).just_finished() {
            if state.timer.paused() {
                // Time is paused so remove text
                for mut text in text_query.iter_mut() {
                    let value = &mut text.sections[0].value;
                    value.clear();
                }
                return;
            }

            let fps_diags = extract_fps(&diagnostics);

            for mut text in text_query.iter_mut() {
                let value = &mut text.sections[0].value;
                value.clear();

                if let Some(fps) = fps_diags {
                    write!(value, "{}{:.1}", STRING_FORMAT, fps).unwrap();
                } else {
                    value.clear();
                    write!(value, "{}", STRING_MISSING).unwrap();
                }
            }
        }
    }
}

fn extract_fps(diagnostics: &Res<Diagnostics>) -> Option<f64> {
    diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts//JetBrainsMono-Medium.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                display: Display::Flex,
                padding: UiRect::new(Val::Undefined, Val::Px(32.0), Val::Px(32.0), Val::Undefined),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: STRING_INITIAL.to_string(),
                            style: TextStyle {
                                font,
                                font_size: FONT_SIZE,
                                color: FONT_COLOR,
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(ScreenDiagsText);
        });
}
