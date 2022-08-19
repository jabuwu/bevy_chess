use crate::common::prelude::*;
use bevy::prelude::*;

pub struct EndGamePlugin;

impl Plugin for EndGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EndGameSpawnEvent>()
            .add_system(end_game_spawn)
            .add_system(end_game_resize);
    }
}

#[derive(Clone)]
pub struct EndGameSpawnEvent(pub String);

#[derive(Component)]
pub struct EndGameText;

fn end_game_spawn(
    mut ev_end_game_spawn: EventReader<EndGameSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_end_game_spawn.iter() {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(50000., 50000.).into(),
                color: Color::rgba(0., 0., 0., 0.93),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.5)),
            ..Default::default()
        });
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
                parent
                    .spawn_bundle(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            position_type: PositionType::Relative,
                            ..Default::default()
                        },
                        text: Text::from_section(
                            event.0.clone(),
                            TextStyle {
                                font: asset_library.font.clone(),
                                font_size: 62.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        }),
                        ..Default::default()
                    })
                    .insert(EndGameText);
            });
    }
}

fn end_game_resize(mut query: Query<&mut Text, With<EndGameText>>, windows: Res<Windows>) {
    if let Some(window) = windows.get_primary() {
        for mut text in query.iter_mut() {
            text.sections[0].style.font_size = window.width().min(window.height()) * 0.15;
        }
    }
}
