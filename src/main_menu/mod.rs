use crate::{
    common::prelude::*,
    game::settings::{GameControl, GameSettings},
};
use bevy::{app::AppExit, prelude::*};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(button_system));
    }
}

#[derive(Component)]
pub struct PlayAsWhiteButton;

#[derive(Component)]
pub struct PlayAsBlackButton;

#[derive(Component)]
pub struct AiVsAiButton;

#[derive(Component)]
pub struct QuitButton;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut UiColor,
            Option<&PlayAsWhiteButton>,
            Option<&PlayAsBlackButton>,
            Option<&AiVsAiButton>,
            Option<&QuitButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<State<GameState>>,
    mut exit: EventWriter<AppExit>,
    mut game_settings: ResMut<GameSettings>,
) {
    for (interaction, mut color, play_as_white, play_as_black, ai_vs_ai, quit) in
        &mut interaction_query
    {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                if color.0 == PRESSED_BUTTON {
                    if play_as_white.is_some() {
                        *game_settings.control_mut(chess::PieceColor::White) = GameControl::Player;
                        *game_settings.control_mut(chess::PieceColor::Black) = GameControl::Ai;
                        game_state.set(GameState::Game).unwrap();
                    }
                    if play_as_black.is_some() {
                        *game_settings.control_mut(chess::PieceColor::White) = GameControl::Ai;
                        *game_settings.control_mut(chess::PieceColor::Black) = GameControl::Player;
                        game_state.set(GameState::Game).unwrap();
                    }
                    if ai_vs_ai.is_some() {
                        *game_settings.control_mut(chess::PieceColor::White) = GameControl::Ai;
                        *game_settings.control_mut(chess::PieceColor::Black) = GameControl::Ai;
                        game_state.set(GameState::Game).unwrap();
                    }
                    if quit.is_some() {
                        exit.send(AppExit);
                    }
                }
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        size: Size::new(Val::Percent(50.0), Val::Px(350.)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .insert(PlayAsWhiteButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                "Play as White",
                                TextStyle {
                                    font: asset_library.font.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .insert(PlayAsBlackButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                "Play as Black",
                                TextStyle {
                                    font: asset_library.font.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .insert(AiVsAiButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                "AI vs AI",
                                TextStyle {
                                    font: asset_library.font.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                    #[cfg(not(target_arch = "wasm32"))]
                    parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(350.0), Val::Px(65.0)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .insert(QuitButton)
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                "Quit",
                                TextStyle {
                                    font: asset_library.font.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });
}
