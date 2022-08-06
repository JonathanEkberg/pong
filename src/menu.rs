use bevy::prelude::*;

use crate::AppState;

struct Button {
    text: &'static str,
}

impl Button {
    fn get_button_style() -> Style {
        Style {
            size: Size::new(Val::Px(400.0), Val::Px(100.0)),
            margin: UiRect::all(Val::Px(20.0)),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }

    fn new(text: &'static str) -> Button {
        Button { text }
    }
}

#[derive(Component)]
struct MenuUI;

fn setup_menu(mut commands: Commands) {
    let buttons: Vec<Button> = vec![
        Button::new("Play"),
        Button::new("Settings"),
        Button::new("Quit"),
    ];

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb(33.0, 33.0, 33.0).into(),
            ..default()
        })
        .with_children(|parent| {
            for button in buttons {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Button::get_button_style(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::from_section(
                                button.text,
                                TextStyle {
                                    color: Color::WHITE,
                                    ..default()
                                },
                            ),
                            ..default()
                        });
                    });
            }
        });
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn button_press() {}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(button_press))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(despawn_menu));
    }
}
