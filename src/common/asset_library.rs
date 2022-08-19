use bevy::{
    asset::{Asset, AssetPath},
    prelude::*,
};

#[derive(Default)]
pub struct AssetLibrary {
    assets: Vec<HandleUntyped>,

    pub font: Handle<Font>,

    pub image_white_pawn: Handle<Image>,
    pub image_white_rook: Handle<Image>,
    pub image_white_knight: Handle<Image>,
    pub image_white_bishop: Handle<Image>,
    pub image_white_queen: Handle<Image>,
    pub image_white_king: Handle<Image>,

    pub image_black_pawn: Handle<Image>,
    pub image_black_rook: Handle<Image>,
    pub image_black_knight: Handle<Image>,
    pub image_black_bishop: Handle<Image>,
    pub image_black_queen: Handle<Image>,
    pub image_black_king: Handle<Image>,
}

impl AssetLibrary {
    fn load<'a, T: Asset, P: Into<AssetPath<'a>>>(
        &mut self,
        asset_server: &Res<AssetServer>,
        path: P,
    ) -> Handle<T> {
        let handle = asset_server.load(path);
        self.assets.push(handle.clone_untyped());
        handle
    }

    pub fn load_assets(&mut self, asset_server: &Res<AssetServer>) {
        self.font = self.load(asset_server, "fonts/Florence-Regular.ttf");

        self.image_white_pawn = self.load(asset_server, "images/white_pawn.png");
        self.image_white_rook = self.load(asset_server, "images/white_rook.png");
        self.image_white_knight = self.load(asset_server, "images/white_knight.png");
        self.image_white_bishop = self.load(asset_server, "images/white_bishop.png");
        self.image_white_queen = self.load(asset_server, "images/white_queen.png");
        self.image_white_king = self.load(asset_server, "images/white_king.png");

        self.image_black_pawn = self.load(asset_server, "images/black_pawn.png");
        self.image_black_rook = self.load(asset_server, "images/black_rook.png");
        self.image_black_knight = self.load(asset_server, "images/black_knight.png");
        self.image_black_bishop = self.load(asset_server, "images/black_bishop.png");
        self.image_black_queen = self.load(asset_server, "images/black_queen.png");
        self.image_black_king = self.load(asset_server, "images/black_king.png");
    }

    pub fn load_state(&self, asset_server: &Res<AssetServer>) -> bevy::asset::LoadState {
        asset_server.get_group_load_state(self.assets.iter().map(|h| h.id))
    }
}

pub struct AssetLibraryPlugin;

impl Plugin for AssetLibraryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetLibrary>();
    }
}
