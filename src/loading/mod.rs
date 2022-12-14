use crate::common::prelude::*;
use bevy::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(loading_init))
            .add_system_set(SystemSet::on_update(GameState::Loading).with_system(loading_update));
    }
}

fn loading_init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
) {
    asset_library.load_assets(&asset_server);
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Relative,
                    position: UiRect {
                        top: Val::Px(-50.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::from_section(
                    "Loading!",
                    TextStyle {
                        font: asset_library.font.clone(),
                        font_size: 42.0,
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                }),
                ..Default::default()
            });
        });
}

fn loading_update(
    asset_library: Res<AssetLibrary>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<State<GameState>>,
) {
    use bevy::asset::LoadState;
    match asset_library.load_state(&asset_server) {
        LoadState::Failed => {
            panic!("Failed to load assets.");
        }
        LoadState::Loaded => {
            game_state.set(GameState::MainMenu).unwrap();
        }
        _ => {}
    }
}
