use crate::common::prelude::*;
use bevy::prelude::*;

use super::{
    end_game::EndGameSpawnEvent,
    settings::{GameControl, GameSettings},
};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BoardSpawnEvent>()
            .add_system(board_spawn)
            .add_system(board_update_pieces)
            .add_system(board_update_ai)
            .add_system(board_update_player)
            .add_system(board_resize);
    }
}

#[derive(Default, Clone, Copy)]
pub struct BoardSpawnEvent;

#[derive(Component)]
pub struct Board {
    board: chess::Board,
    dirty: bool,
    turn: chess::PieceColor,
    stopped: bool,
}

#[derive(Component)]
pub struct BoardSquare {
    position: chess::Position,
}

#[derive(Component)]
pub struct BoardPiece {
    board_entity: Entity,
    position: chess::Position,
}

impl BoardPiece {
    fn texture(piece: &chess::Piece, asset_library: &AssetLibrary) -> Handle<Image> {
        match piece.color() {
            chess::PieceColor::White => match piece.kind() {
                chess::PieceKind::Pawn => asset_library.image_white_pawn.clone(),
                chess::PieceKind::Rook => asset_library.image_white_rook.clone(),
                chess::PieceKind::Knight => asset_library.image_white_knight.clone(),
                chess::PieceKind::Bishop => asset_library.image_white_bishop.clone(),
                chess::PieceKind::Queen => asset_library.image_white_queen.clone(),
                chess::PieceKind::King => asset_library.image_white_king.clone(),
            },
            chess::PieceColor::Black => match piece.kind() {
                chess::PieceKind::Pawn => asset_library.image_black_pawn.clone(),
                chess::PieceKind::Rook => asset_library.image_black_rook.clone(),
                chess::PieceKind::Knight => asset_library.image_black_knight.clone(),
                chess::PieceKind::Bishop => asset_library.image_black_bishop.clone(),
                chess::PieceKind::Queen => asset_library.image_black_queen.clone(),
                chess::PieceKind::King => asset_library.image_black_king.clone(),
            },
        }
    }
}

pub fn board_spawn(
    mut ev_board_spawn: EventReader<BoardSpawnEvent>,
    mut command: Commands,
    game_settings: Res<GameSettings>,
) {
    for _ in ev_board_spawn.iter() {
        command
            .spawn_bundle(TransformBundle {
                local: Transform::from_translation(Vec3::ZERO)
                    .with_scale(Vec3::new(512., 512., 1.)),
                ..Default::default()
            })
            .insert_bundle(VisibilityBundle::default())
            .insert(Board {
                board: chess::Board::new(),
                dirty: true,
                turn: chess::PieceColor::White,
                stopped: false,
            })
            .with_children(|parent| {
                let col_count = chess::Position::col_count() as f32;
                let row_count = chess::Position::col_count() as f32;
                let width = 1. / col_count;
                let height = 1. / row_count;
                for position in chess::Position::all() {
                    let flip_board = if let GameControl::Player =
                        game_settings.control(chess::PieceColor::Black)
                    {
                        if let GameControl::Ai = game_settings.control(chess::PieceColor::White) {
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    };
                    let col = if flip_board {
                        (chess::Position::col_count() - position.col() - 1) as f32
                    } else {
                        position.col() as f32
                    };
                    let row = if flip_board {
                        position.row() as f32
                    } else {
                        (chess::Position::row_count() - position.row() - 1) as f32
                    };
                    let color = if position.col() % 2 != position.row() % 2 {
                        Color::rgb_u8(118, 134, 171)
                    } else {
                        Color::rgb_u8(200, 200, 200)
                    };
                    parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                custom_size: Vec2::new(1., 1.).into(),
                                color,
                                ..Default::default()
                            },
                            transform: Transform::from_translation(
                                Vec2::new(
                                    col / col_count - 0.5 + width / 2.,
                                    row / row_count - 0.5 + height / 2.,
                                )
                                .extend(0.1),
                            )
                            .with_scale(Vec3::new(width, height, 1.)),
                            ..Default::default()
                        })
                        .insert(BoardSquare {
                            position: *position,
                        });
                }
            });
    }
}

pub fn board_update_pieces(
    mut commands: Commands,
    mut board_query: Query<(Entity, &mut Board, &Children)>,
    square_query: Query<(&BoardSquare, Option<&Children>)>,
    asset_library: Res<AssetLibrary>,
    mut ev_end_game_spawn: EventWriter<EndGameSpawnEvent>,
) {
    for (board_entity, mut board, board_children) in board_query.iter_mut() {
        if board.dirty {
            if board.board.valid_moves(board.turn).is_empty() {
                if board.board.check(board.turn) {
                    match board.turn {
                        chess::PieceColor::White => {
                            ev_end_game_spawn
                                .send(EndGameSpawnEvent("Checkmate\nBlack Won".to_owned()));
                        }
                        chess::PieceColor::Black => {
                            ev_end_game_spawn
                                .send(EndGameSpawnEvent("Checkmate\nWhite Won".to_owned()));
                        }
                    }
                } else {
                    ev_end_game_spawn.send(EndGameSpawnEvent("Stalemate".to_owned()));
                }
                board.stopped = true;
            }
            for square_entity in board_children.iter() {
                if let Ok((square, square_children)) = square_query.get(*square_entity) {
                    if let Some(children) = square_children {
                        for child in children.iter() {
                            commands.entity(*square_entity).remove_children(&[*child]);
                            commands.entity(*child).despawn();
                        }
                    }
                    if let Some(piece) = &board.board.piece(square.position) {
                        let piece = commands
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Vec2::new(0.9, 0.9).into(),
                                    color: Color::WHITE,
                                    ..Default::default()
                                },
                                transform: Transform::from_translation(Vec2::ZERO.extend(0.1)),
                                texture: BoardPiece::texture(piece, asset_library.as_ref()),
                                ..Default::default()
                            })
                            .insert(BoardPiece {
                                board_entity,
                                position: square.position,
                            })
                            .id();
                        commands.entity(*square_entity).add_child(piece);
                    }
                }
            }
            board.dirty = false;
        }
    }
}

#[derive(Default)]
pub struct BoardUpdateAiState {
    timer: f32,
}

pub fn board_update_ai(
    mut board_query: Query<&mut Board>,
    mut state: Local<BoardUpdateAiState>,
    time: Res<Time>,
    settings: Res<GameSettings>,
) {
    let turn_timer = {
        state.timer += time.delta_seconds();
        if state.timer > 0.5 {
            state.timer = 0.;
            true
        } else {
            false
        }
    };
    if turn_timer {
        for mut board in board_query.iter_mut() {
            if let GameControl::Ai = settings.control(board.turn) {
                if board.stopped {
                    continue;
                }
                if let Some(ai_move) = chess_ai::plan_move(&board.board, board.turn) {
                    board.board.apply_move(ai_move);
                    board.turn = board.turn.opposite();
                    board.dirty = true;
                }
            }
        }
    }
}

#[derive(Default)]
pub struct BoardUpdatePlayerState {
    drag: Option<Entity>,
}

pub fn board_update_player(
    mut state: Local<BoardUpdatePlayerState>,
    square_query: Query<(&BoardSquare, &GlobalTransform, Option<&Children>)>,
    mut piece_query: Query<(&BoardPiece, &Parent, &mut Transform)>,
    mut board_query: Query<&mut Board>,
    input: Res<Input<MouseButton>>,
    mouse: Res<Mouse>,
    game_settings: Res<GameSettings>,
) {
    let mut hovered_position = None;
    let mut hovered_piece_entity = None;
    for (square, transform, children) in square_query.iter() {
        let (world_scale, _, world_position) = transform.to_scale_rotation_translation();
        if mouse.position.x > world_position.x - world_scale.x * 0.5
            && mouse.position.x < world_position.x + world_scale.x * 0.5
            && mouse.position.y > world_position.y - world_scale.x * 0.5
            && mouse.position.y < world_position.y + world_scale.x * 0.5
        {
            hovered_position = Some(square.position);
            if let Some(children) = children {
                hovered_piece_entity = children.get(0).cloned();
            }
            break;
        }
    }
    if input.just_pressed(MouseButton::Left) {
        if let Some(hovered_piece_entity) = hovered_piece_entity {
            if let Ok((piece, _, _)) = piece_query.get_mut(hovered_piece_entity) {
                if let Ok(board) = board_query.get(piece.board_entity) {
                    if let Some(piece) = board.board.piece(piece.position) {
                        if piece.color() == board.turn && !board.stopped {
                            if let GameControl::Player = game_settings.control(piece.color()) {
                                state.drag = Some(hovered_piece_entity);
                            }
                        }
                    }
                }
            }
        }
    }
    let mut release_drag = false;
    if input.just_released(MouseButton::Left) {
        release_drag = true;
    }
    if let Some(drag_entity) = state.drag {
        if let Ok((piece, parent, mut transform)) = piece_query.get_mut(drag_entity) {
            if release_drag {
                let mut reset_position = true;
                if let Some(hovered_position) = hovered_position {
                    let board_move = chess::BoardMove {
                        from: piece.position,
                        to: hovered_position,
                    };
                    if let Ok(mut board) = board_query.get_mut(piece.board_entity) {
                        if board.board.apply_move(board_move) {
                            reset_position = false;
                            board.turn = board.turn.opposite();
                            board.dirty = true;
                        }
                    }
                }
                if reset_position {
                    transform.translation = Vec3::new(0., 0., 0.1);
                }
            } else {
                if let Ok((_, parent_transform, _)) = square_query.get(parent.get()) {
                    let local_mouse_pos = parent_transform
                        .compute_matrix()
                        .inverse()
                        .project_point3(mouse.position.extend(0.))
                        .truncate();
                    transform.translation = local_mouse_pos.extend(0.2);
                }
            }
        }
    }
    if release_drag {
        state.drag = None;
    }
}

fn board_resize(mut board_query: Query<&mut Transform, With<Board>>, windows: Res<Windows>) {
    if let Some(window) = windows.get_primary() {
        let width = window.width();
        let height = window.height();
        let mut board_width = width;
        let mut board_height = width;
        if width > height {
            board_width = height;
            board_height = height;
        }
        for mut transform in board_query.iter_mut() {
            transform.scale = Vec3::new(board_width, board_height, 1.);
        }
    }
}
