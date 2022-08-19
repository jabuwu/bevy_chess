use crate::common::prelude::*;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<settings::GameSettings>()
            .add_plugin(board::BoardPlugin)
            .add_plugin(end_game::EndGamePlugin)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(game_init))
            .add_system_set(SystemSet::on_update(GameState::Game).with_system(esc_to_menu));
    }
}

fn game_init(mut commands: Commands, mut ev_board_spawn: EventWriter<board::BoardSpawnEvent>) {
    commands.spawn_bundle(Camera2dBundle::default());
    ev_board_spawn.send_default();
}

fn esc_to_menu(mut keys: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<GameState>>) {
    if keys.just_pressed(KeyCode::Escape) {
        app_state.set(GameState::MainMenu).unwrap();
        keys.reset(KeyCode::Escape);
    }
}

pub mod board;
pub mod end_game;
pub mod settings;
