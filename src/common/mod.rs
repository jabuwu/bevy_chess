use bevy::prelude::*;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(asset_library::AssetLibraryPlugin)
            .add_plugin(game_state::GameStatePlugin)
            .add_plugin(mouse::MousePlugin)
            .add_plugin(wasm::WasmPlugin)
            .add_plugin(background::BackgroundPlugin);
    }
}

pub mod asset_library;
pub mod background;
pub mod game_state;
pub mod mouse;
pub mod prelude;
pub mod wasm;
