use crate::{
    constants::{COL_COUNT, ROW_COUNT, SQUARE_COUNT},
    piece::PieceMovePlanner,
    rules::{
        castle::Castle, en_passant::EnPassant, pawn_first_move::PawnFirstMove, promotion::Promotion,
    },
    Ic, Piece, PieceColor, PieceKind, Position, Uc,
};
use std::fmt;

#[derive(Clone, Copy)]
pub struct Board {
    pieces: [Option<Piece>; SQUARE_COUNT],
    castle: Castle,
    en_passant: EnPassant,
    pawn_first_move: PawnFirstMove,
    promotion: Promotion,
}

impl Board {
    pub fn new() -> Self {
        let mut pieces = [None; SQUARE_COUNT];
        let back_row = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];
        assert_eq!(COL_COUNT, 8);
        assert_eq!(ROW_COUNT, 8);
        assert_eq!(back_row.len(), 8);
        for (i, kind) in back_row.iter().enumerate() {
            pieces[i] = Some(Piece::new(*kind, PieceColor::Black));
            pieces[i + 56] = Some(Piece::new(*kind, PieceColor::White));
        }
        for i in 0..8 {
            pieces[i + 8] = Some(Piece::new(PieceKind::Pawn, PieceColor::Black));
            pieces[i + 48] = Some(Piece::new(PieceKind::Pawn, PieceColor::White));
        }
        Self {
            pieces,
            castle: Castle::new(),
            en_passant: EnPassant::new(),
            pawn_first_move: PawnFirstMove::new(),
            promotion: Promotion::new(),
        }
    }

    pub fn set_piece(&mut self, position: Position, piece: Option<Piece>) {
        self.pieces[position.index()] = piece;
    }

    pub fn piece(&self, position: Position) -> Option<Piece> {
        self.pieces[position.index()]
    }

    pub(crate) fn moves_internal(&self, color: PieceColor, king_check: bool) -> Vec<BoardMove> {
        let mut all_moves = vec![];
        for position in Position::all().iter() {
            if let Some(piece) = &self.piece(*position) {
                if piece.color() == color {
                    let mut planner = PieceMovePlanner::new(self, *position, color);
                    piece.moves(&mut planner);
                    self.castle.moves(piece, &mut planner, king_check);
                    self.pawn_first_move.moves(piece, &mut planner);
                    self.en_passant.moves(piece, &mut planner);
                    let mut moves = planner.moves();
                    while let Some(board_move) = moves.pop() {
                        if king_check {
                            let mut board_clone = self.clone();
                            board_clone.force_move(board_move);
                            if !board_clone.check(color) {
                                all_moves.push(board_move);
                            }
                        } else {
                            all_moves.push(board_move);
                        }
                    }
                }
            }
        }
        all_moves
    }

    pub fn valid_moves(&self, color: PieceColor) -> Vec<BoardMove> {
        self.moves_internal(color, true)
    }

    pub fn force_move(&mut self, board_move: BoardMove) {
        self.castle.apply_move(&board_move, &mut self.pieces);
        self.en_passant.apply_move(&board_move, &mut self.pieces);
        self.pieces[board_move.to.index()] = self.pieces[board_move.from.index()];
        self.pieces[board_move.from.index()] = None;
        self.promotion.apply_move(&mut self.pieces);
    }

    pub fn apply_move(&mut self, board_move: BoardMove) -> bool {
        if self.is_valid_move(board_move) {
            self.force_move(board_move);
            true
        } else {
            false
        }
    }

    pub fn is_valid_move(&self, board_move: BoardMove) -> bool {
        if let Some(piece) = &self.piece(board_move.from) {
            let valid_moves = self.valid_moves(piece.color());
            return valid_moves.into_iter().find(|m| *m == board_move).is_some();
        } else {
            false
        }
    }

    pub fn check(&self, color: PieceColor) -> bool {
        let opponents_moves = self.moves_internal(color.opposite(), false);
        for position in Position::all().iter() {
            if let Some(piece) = self.piece(*position) {
                if piece.color() == color && piece.kind() == PieceKind::King {
                    for opponents_move in opponents_moves.iter() {
                        if opponents_move.to == *position {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn score(&self, color: PieceColor) -> Ic {
        let mut score = 0;
        for position in Position::all().iter() {
            if let Some(piece) = self.piece(*position) {
                if piece.color() == color {
                    score += piece.value() as i8;
                } else {
                    score -= piece.value() as i8;
                }
            }
        }
        score
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Board) -> bool {
        for position in Position::all().iter() {
            if self.piece(*position) != other.piece(*position) {
                return false;
            }
        }
        true
    }
    fn ne(&self, other: &Board) -> bool {
        !self.eq(other)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        write!(f, " ")?;
        for col in 0..Position::col_count() {
            write!(f, " {}", Position::col_char(col))?;
        }
        let mut last_row: Option<Uc> = None;
        for position in Position::all().iter() {
            let new_row = if let Some(lr) = last_row {
                if lr != position.row() {
                    last_row = Some(position.row());
                    true
                } else {
                    false
                }
            } else {
                last_row = Some(position.row());
                true
            };
            if new_row {
                writeln!(f)?;
                write!(f, "{}|", Position::row_char(position.row()))?;
            }
            if let Some(piece) = self.piece(*position) {
                write!(f, "{}|", piece.char())?;
            } else {
                write!(f, " |")?;
            }
        }
        writeln!(f)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoardMove {
    pub from: Position,
    pub to: Position,
}

impl ToString for BoardMove {
    fn to_string(&self) -> String {
        format!("{} -> {}", self.from.to_string(), self.to.to_string(),)
    }
}
