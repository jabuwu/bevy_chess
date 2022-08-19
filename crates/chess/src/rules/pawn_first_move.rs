use crate::{piece::PieceMovePlanner, Piece, PieceColor, PieceKind, PositionOffset};

#[derive(Clone, Copy)]
pub struct PawnFirstMove;

impl PawnFirstMove {
    pub fn new() -> Self {
        Self
    }

    pub(crate) fn moves<'a>(&self, piece: &Piece, planner: &mut PieceMovePlanner<'a>) {
        if let PieceKind::Pawn = piece.kind() {
            match piece.color() {
                PieceColor::White => {
                    if planner.position().row() == 6 {
                        if planner.try_add_no_take(PositionOffset(0, -1)) {
                            planner.try_add_no_take(PositionOffset(0, -2));
                        }
                    }
                }
                PieceColor::Black => {
                    if planner.position().row() == 1 {
                        if planner.try_add_no_take(PositionOffset(0, 1)) {
                            planner.try_add_no_take(PositionOffset(0, 2));
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chess_macros::{board, board_move};

    #[test]
    fn pawn_first_move_white() {
        let mut board = board!(
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            p p _ _ _ _ _ _
            _ _ _ _ _ _ _ _
        );

        assert!(board.is_valid_move(board_move!(a2 a3)));
        assert!(board.is_valid_move(board_move!(a2 a4)));
        assert!(board.is_valid_move(board_move!(b2 b3)));
        assert!(board.is_valid_move(board_move!(b2 b3)));

        assert!(board.apply_move(board_move!(a2 a3)));
        assert!(board.apply_move(board_move!(b2 b4)));

        assert!(board.is_valid_move(board_move!(a3 a4)));
        assert!(!board.is_valid_move(board_move!(a3 a5)));
        assert!(board.is_valid_move(board_move!(b4 b5)));
        assert!(!board.is_valid_move(board_move!(b4 b6)));
    }

    #[test]
    fn pawn_first_move_black() {
        let mut board = board!(
            _ _ _ _ _ _ _ _
            P P _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
        );

        assert!(board.is_valid_move(board_move!(a7 a6)));
        assert!(board.is_valid_move(board_move!(a7 a5)));
        assert!(board.is_valid_move(board_move!(b7 b6)));
        assert!(board.is_valid_move(board_move!(b7 b5)));

        assert!(board.apply_move(board_move!(a7 a6)));
        assert!(board.apply_move(board_move!(b7 b5)));

        assert!(board.is_valid_move(board_move!(a6 a5)));
        assert!(!board.is_valid_move(board_move!(a6 a4)));
        assert!(board.is_valid_move(board_move!(b5 b4)));
        assert!(!board.is_valid_move(board_move!(b5 b3)));
    }
}
