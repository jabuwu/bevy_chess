use crate::common::prelude::*;
use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(bg_spawn);
    }
}

fn bg_spawn(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(50000., 50000.).into(),
                color: Color::BLACK,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Persistent);
}
