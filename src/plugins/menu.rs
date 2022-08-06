use bevy::{app::AppExit, prelude::*};

use crate::{
    font::{get_font, Fonts, MenuFontWeight},
    AppState,
};

#[derive(Component)]
enum MenuButton {
    Play,
    Settings,
    Quit,
}

pub trait Title {
    fn title(&self) -> String;
}

impl Title for MenuButton {
    fn title(&self) -> String {
        match self {
            MenuButton::Play => "Play".to_string(),
            MenuButton::Settings => "Settings".to_string(),
            MenuButton::Quit => "Quit".to_string(),
        }
    }
}

impl MenuButton {
    fn get_button_style() -> Style {
        const HORIZONTAL: Val = Val::Px(32.0);
        const VERTICAL: Val = Val::Px(16.0);

        Style {
            // size: Size::new(Val::Px(400.0), Val::Px(100.0)),
            padding: UiRect::new(HORIZONTAL, HORIZONTAL, VERTICAL, VERTICAL),
            margin: UiRect::all(Val::Px(20.0)),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }
}

#[derive(Component)]
struct MenuUI;

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let buttons: Vec<MenuButton> = vec![MenuButton::Play, MenuButton::Settings, MenuButton::Quit];

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgb(0.05, 0.05, 0.05).into(),
            ..default()
        })
        .with_children(|parent| {
            for button in buttons {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: MenuButton::get_button_style(),
                        color: Color::RED.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle {
                            text: Text::from_section(
                                button.title(),
                                TextStyle {
                                    font: get_font(
                                        &asset_server,
                                        Fonts::MENU(MenuFontWeight::Bold),
                                    ),
                                    font_size: 32.0,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            ),
                            ..default()
                        });
                    })
                    .insert(button);
            }
        })
        .insert(MenuUI);
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    println!("Despawning {} menus", query.iter().len());
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn button_press(
    query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<MenuButton>)>,
    mut state: ResMut<State<AppState>>,
    mut exit_events: EventWriter<AppExit>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => state
                    .set(AppState::Game)
                    .expect("Couldn't switch state to MakeMap"),
                MenuButton::Settings => {}
                MenuButton::Quit => exit_events.send(AppExit),
            };
        }
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_update(AppState::Menu).with_system(button_press))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(despawn_menu));
    }
}
