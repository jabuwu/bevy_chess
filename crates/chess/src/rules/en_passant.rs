use crate::{
    constants::SQUARE_COUNT, piece::PieceMovePlanner, BoardMove, Piece, PieceColor, PieceKind,
    PositionOffset,
};

#[derive(Clone, Copy)]
pub struct EnPassant {
    last_pawn_move: Option<BoardMove>,
}

impl EnPassant {
    pub fn new() -> Self {
        Self {
            last_pawn_move: None,
        }
    }

    pub(crate) fn moves<'a>(&self, piece: &Piece, planner: &mut PieceMovePlanner<'a>) {
        if let Some(last_pawn_move) = self.last_pawn_move {
            if let PieceKind::Pawn = piece.kind() {
                match piece.color() {
                    PieceColor::White => {
                        if planner.position().row() == 3
                            && last_pawn_move.from.row() == 1
                            && last_pawn_move.to.row() == 3
                        {
                            if let Some(left_passant) =
                                planner.position().offset(PositionOffset(-1, -1))
                            {
                                if left_passant.col() == last_pawn_move.to.col() {
                                    planner.try_add_no_take(PositionOffset(-1, -1));
                                }
                            }
                            if let Some(right_passant) =
                                planner.position().offset(PositionOffset(1, -1))
                            {
                                if right_passant.col() == last_pawn_move.to.col() {
                                    planner.try_add_no_take(PositionOffset(1, -1));
                                }
                            }
                        }
                    }
                    PieceColor::Black => {
                        if planner.position().row() == 4
                            && last_pawn_move.from.row() == 6
                            && last_pawn_move.to.row() == 4
                        {
                            if let Some(left_passant) =
                                planner.position().offset(PositionOffset(-1, 1))
                            {
                                if left_passant.col() == last_pawn_move.to.col() {
                                    planner.try_add_no_take(PositionOffset(-1, 1));
                                }
                            }
                            if let Some(right_passant) =
                                planner.position().offset(PositionOffset(1, -1))
                            {
                                if right_passant.col() == last_pawn_move.to.col() {
                                    planner.try_add_no_take(PositionOffset(1, 1));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn apply_move(
        &mut self,
        board_move: &BoardMove,
        pieces: &mut [Option<Piece>; SQUARE_COUNT],
    ) {
        if let Some(last_pawn_move) = self.last_pawn_move {
            if let Some(piece) = &pieces[board_move.from.index()] {
                if piece.kind() == PieceKind::Pawn {
                    if board_move.from.col() != board_move.to.col()
                        && pieces[board_move.to.index()].is_none()
                    {
                        pieces[last_pawn_move.to.index()] = None;
                    }
                }
            }
        }
        self.last_pawn_move = {
            if let Some(piece) = &pieces[board_move.from.index()] {
                if piece.kind() == PieceKind::Pawn {
                    Some(*board_move)
                } else {
                    None
                }
            } else {
                None
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use chess_macros::{board, board_move};

    #[test]
    fn en_passant_white_left() {
        let mut board = board!(
            _ _ _ _ _ _ _ _
            _ _ _ P _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ p _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
        );
        assert!(board.apply_move(board_move!(d7 d5)));
        assert!(board.apply_move(board_move!(e5 d6)));
        assert_eq!(
            board,
            board!(
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ p _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
            )
        );
    }

    #[test]
    fn en_passant_white_right() {
        let mut board = board!(
            _ _ _ _ _ _ _ _
            _ _ _ _ P _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ p _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
        );
        assert!(board.apply_move(board_move!(e7 e5)));
        assert!(board.apply_move(board_move!(d5 e6)));
        assert_eq!(
            board,
            board!(
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ p _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
            )
        );
    }

    #[test]
    fn en_passant_black_left() {
        let mut board = board!(
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ P _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ p _ _ _ _
            _ _ _ _ _ _ _ _
        );
        assert!(board.apply_move(board_move!(d2 d4)));
        assert!(board.apply_move(board_move!(e4 d3)));
        assert_eq!(
            board,
            board!(
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ P _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
            )
        );
    }

    #[test]
    fn en_passant_black_right() {
        let mut board = board!(
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ P _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ p _ _ _
            _ _ _ _ _ _ _ _
        );
        assert!(board.apply_move(board_move!(e2 e4)));
        assert!(board.apply_move(board_move!(d4 e3)));
        assert_eq!(
            board,
            board!(
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ P _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
            )
        );
    }
}
