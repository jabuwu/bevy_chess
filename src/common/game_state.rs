use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameState {
    Loading,
    MainMenu,
    Game,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(cleanup_entities))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(cleanup_entities))
            .add_system_set(SystemSet::on_exit(GameState::Game).with_system(cleanup_entities));
    }
}

#[derive(Component)]
pub struct Persistent;

fn cleanup_entities(mut commands: Commands, query: Query<Entity, Without<Persistent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
