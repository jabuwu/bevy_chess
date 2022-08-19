use crate::{constants::SQUARE_COUNT, Board, BoardMove, Position, PositionOffset, Uc};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    kind: PieceKind,
    color: PieceColor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PieceColor {
    #[default]
    White,
    Black,
}

impl PieceColor {
    pub fn opposite(&self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl Piece {
    pub fn new(kind: PieceKind, color: PieceColor) -> Self {
        Self { kind, color }
    }

    pub fn kind(&self) -> PieceKind {
        self.kind
    }

    pub fn color(&self) -> PieceColor {
        self.color
    }

    pub fn value(&self) -> Uc {
        match self.color() {
            PieceColor::White => match self.kind {
                PieceKind::Pawn => 1,
                PieceKind::Rook => 5,
                PieceKind::Knight => 3,
                PieceKind::Bishop => 3,
                PieceKind::Queen => 8,
                PieceKind::King => 6,
            },
            PieceColor::Black => match self.kind {
                PieceKind::Pawn => 1,
                PieceKind::Rook => 5,
                PieceKind::Knight => 3,
                PieceKind::Bishop => 3,
                PieceKind::Queen => 8,
                PieceKind::King => 6,
            },
        }
    }

    pub fn char(&self) -> char {
        match self.color() {
            PieceColor::White => match self.kind {
                PieceKind::Pawn => 'p',
                PieceKind::Rook => 'r',
                PieceKind::Knight => 'n',
                PieceKind::Bishop => 'b',
                PieceKind::Queen => 'q',
                PieceKind::King => 'k',
            },
            PieceColor::Black => match self.kind {
                PieceKind::Pawn => 'P',
                PieceKind::Rook => 'R',
                PieceKind::Knight => 'N',
                PieceKind::Bishop => 'B',
                PieceKind::Queen => 'Q',
                PieceKind::King => 'K',
            },
        }
    }

    pub(crate) fn moves<'a>(&self, planner: &mut PieceMovePlanner<'a>) {
        match self.kind() {
            PieceKind::Pawn => match self.color() {
                PieceColor::White => {
                    planner.try_add_no_take(PositionOffset(0, -1));
                    planner.try_add_take_only(PositionOffset(-1, -1));
                    planner.try_add_take_only(PositionOffset(1, -1));
                }
                PieceColor::Black => {
                    planner.try_add_no_take(PositionOffset(0, 1));
                    planner.try_add_take_only(PositionOffset(-1, 1));
                    planner.try_add_take_only(PositionOffset(1, 1));
                }
            },
            PieceKind::Rook => {
                planner.try_add_directional_take(PositionOffset(0, -1));
                planner.try_add_directional_take(PositionOffset(0, 1));
                planner.try_add_directional_take(PositionOffset(-1, 0));
                planner.try_add_directional_take(PositionOffset(1, 0));
            }
            PieceKind::Knight => {
                planner.try_add_take(PositionOffset(-2, -1));
                planner.try_add_take(PositionOffset(-1, -2));
                planner.try_add_take(PositionOffset(-2, 1));
                planner.try_add_take(PositionOffset(-1, 2));
                planner.try_add_take(PositionOffset(2, -1));
                planner.try_add_take(PositionOffset(1, -2));
                planner.try_add_take(PositionOffset(2, 1));
                planner.try_add_take(PositionOffset(1, 2));
            }
            PieceKind::Bishop => {
                planner.try_add_directional_take(PositionOffset(-1, -1));
                planner.try_add_directional_take(PositionOffset(-1, 1));
                planner.try_add_directional_take(PositionOffset(1, -1));
                planner.try_add_directional_take(PositionOffset(1, 1));
            }
            PieceKind::Queen => {
                planner.try_add_directional_take(PositionOffset(0, -1));
                planner.try_add_directional_take(PositionOffset(0, 1));
                planner.try_add_directional_take(PositionOffset(-1, 0));
                planner.try_add_directional_take(PositionOffset(1, 0));
                planner.try_add_directional_take(PositionOffset(-1, -1));
                planner.try_add_directional_take(PositionOffset(-1, 1));
                planner.try_add_directional_take(PositionOffset(1, -1));
                planner.try_add_directional_take(PositionOffset(1, 1));
            }
            PieceKind::King => {
                planner.try_add_take(PositionOffset(0, -1));
                planner.try_add_take(PositionOffset(0, 1));
                planner.try_add_take(PositionOffset(-1, 0));
                planner.try_add_take(PositionOffset(1, 0));
                planner.try_add_take(PositionOffset(-1, -1));
                planner.try_add_take(PositionOffset(-1, 1));
                planner.try_add_take(PositionOffset(1, -1));
                planner.try_add_take(PositionOffset(1, 1));
            }
        }
    }
}

pub(crate) struct PieceMovePlanner<'a> {
    board: &'a Board,
    position: Position,
    color: PieceColor,
    moves: [Option<BoardMove>; SQUARE_COUNT],
}

impl<'a> PieceMovePlanner<'a> {
    pub(crate) fn new(board: &'a Board, position: Position, color: PieceColor) -> Self {
        Self {
            board,
            position,
            color,
            moves: [None; SQUARE_COUNT],
        }
    }

    pub(crate) fn position(&self) -> Position {
        self.position
    }

    pub(crate) fn add_move(&mut self, to: Position) {
        self.moves[to.index()] = Some(BoardMove {
            from: self.position,
            to,
        });
    }

    pub(crate) fn try_add_no_take(&mut self, offset: PositionOffset) -> bool {
        if let Some(to) = self.position.offset(offset) {
            if self.board.piece(to).is_none() {
                self.add_move(to);
                return true;
            }
        }
        false
    }

    pub(crate) fn try_add_take(&mut self, offset: PositionOffset) -> bool {
        if let Some(to) = self.position.offset(offset) {
            if let Some(piece) = self.board.piece(to) {
                if piece.color() != self.color {
                    self.add_move(to);
                    return true;
                }
            } else {
                self.add_move(to);
                return true;
            }
        }
        false
    }

    pub(crate) fn try_add_take_only(&mut self, offset: PositionOffset) -> bool {
        if let Some(to) = self.position.offset(offset) {
            if let Some(piece) = self.board.piece(to) {
                if piece.color() != self.color {
                    self.add_move(to);
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn try_add_directional_take(&mut self, direction: PositionOffset) {
        let mut new_position = self.position.offset(direction);
        while let Some(to) = new_position {
            if let Some(piece) = self.board.piece(to) {
                if piece.color() != self.color {
                    self.add_move(to);
                    break;
                } else {
                    break;
                }
            } else {
                self.add_move(to);
            }
            new_position = to.offset(direction);
        }
    }

    pub(crate) fn is_under_attack(&self, offset: PositionOffset) -> bool {
        let valid_moves = self.board.moves_internal(self.color.opposite(), false);
        if let Some(position) = self.position.offset(offset) {
            for valid_move in valid_moves.iter() {
                if valid_move.to == position {
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn is_my_piece(&self, offset: PositionOffset, kind: PieceKind) -> bool {
        if let Some(position) = self.position.offset(offset) {
            if let Some(piece) = self.board.piece(position) {
                return piece.color() == self.color && piece.kind() == kind;
            }
        }
        false
    }

    pub(crate) fn is_empty(&self, offset: PositionOffset) -> bool {
        if let Some(position) = self.position.offset(offset) {
            if let Some(..) = self.board.piece(position) {
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    pub(crate) fn moves(&self) -> Vec<BoardMove> {
        let mut moves = vec![];
        for i in 0..SQUARE_COUNT {
            if let Some(board_move) = &self.moves[i] {
                moves.push(*board_move);
            }
        }
        moves
    }
}
