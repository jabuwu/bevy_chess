use bevy::prelude::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: format!("Bevy Chess v{}", VERSION),
            width: 512.,
            height: 512.,
            resizable: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugin(common::CommonPlugin)
        .add_plugin(loading::LoadingPlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

pub mod common;
pub mod game;
pub mod loading;
pub mod main_menu;
