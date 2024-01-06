use crate::{
    constants::SQUARE_COUNT, piece::PieceMovePlanner, BoardMove, Piece, PieceColor, PieceKind,
    PositionOffset,
};
use chess_macros::board_move;

#[derive(Clone, Copy)]
pub struct Castle {
    moved: [bool; SQUARE_COUNT],
}

impl Castle {
    pub fn new() -> Self {
        Self {
            moved: [false; SQUARE_COUNT],
        }
    }

    pub(crate) fn moves<'a>(
        &self,
        piece: &Piece,
        planner: &mut PieceMovePlanner<'a>,
        king_check: bool,
    ) {
        if let PieceKind::King = piece.kind() {
            if planner.is_empty(PositionOffset(1, 0))
                && planner.is_empty(PositionOffset(2, 0))
                && planner.is_my_piece(PositionOffset(3, 0), PieceKind::Rook)
                && !planner.is_under_attack(PositionOffset(0, 0))
            {
                let rook_position = planner.position().offset(PositionOffset(3, 0)).unwrap();
                if !self.moved[planner.position().index()] && !self.moved[rook_position.index()] {
                    if king_check && !planner.is_under_attack(PositionOffset(1, 0)) {
                        planner.try_add_no_take(PositionOffset(2, 0));
                    }
                }
            }
            if planner.is_empty(PositionOffset(-1, 0))
                && planner.is_empty(PositionOffset(-2, 0))
                && planner.is_empty(PositionOffset(-3, 0))
                && planner.is_my_piece(PositionOffset(-4, 0), PieceKind::Rook)
                && !planner.is_under_attack(PositionOffset(0, 0))
            {
                let rook_position = planner.position().offset(PositionOffset(-4, 0)).unwrap();
                if !self.moved[planner.position().index()] && !self.moved[rook_position.index()] {
                    if king_check && !planner.is_under_attack(PositionOffset(-1, 0)) {
                        planner.try_add_no_take(PositionOffset(-2, 0));
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
        if let Some(moved_piece) = pieces[board_move.from.index()] {
            if let PieceKind::King = moved_piece.kind() {
                if moved_piece.color() == PieceColor::White && *board_move == board_move!(e1 g1) {
                    if let Some(rook_position) = board_move.from.offset(PositionOffset(3, 0)) {
                        if let Some(rook_new_position) =
                            board_move.from.offset(PositionOffset(1, 0))
                        {
                            pieces[rook_new_position.index()] = pieces[rook_position.index()];
                            pieces[rook_position.index()] = None;
                        }
                    }
                }
                if moved_piece.color() == PieceColor::White && *board_move == board_move!(e1 c1) {
                    if let Some(rook_position) = board_move.from.offset(PositionOffset(-4, 0)) {
                        if let Some(rook_new_position) =
                            board_move.from.offset(PositionOffset(-1, 0))
                        {
                            pieces[rook_new_position.index()] = pieces[rook_position.index()];
                            pieces[rook_position.index()] = None;
                        }
                    }
                }
                if moved_piece.color() == PieceColor::Black && *board_move == board_move!(e8 g8) {
                    if let Some(rook_position) = board_move.from.offset(PositionOffset(3, 0)) {
                        if let Some(rook_new_position) =
                            board_move.from.offset(PositionOffset(1, 0))
                        {
                            pieces[rook_new_position.index()] = pieces[rook_position.index()];
                            pieces[rook_position.index()] = None;
                        }
                    }
                }
                if moved_piece.color() == PieceColor::Black && *board_move == board_move!(e8 c8) {
                    if let Some(rook_position) = board_move.from.offset(PositionOffset(-4, 0)) {
                        if let Some(rook_new_position) =
                            board_move.from.offset(PositionOffset(-1, 0))
                        {
                            pieces[rook_new_position.index()] = pieces[rook_position.index()];
                            pieces[rook_position.index()] = None;
                        }
                    }
                }
            }
        }
        self.moved[board_move.from.index()] = true;
        self.moved[board_move.to.index()] = true;
    }
}

#[cfg(test)]
mod tests {
    use chess_macros::{board, board_move};

    #[test]
    fn castle_white_king_side() {
        let mut board = board!(
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            r _ _ _ k _ _ r
        );
        assert!(board.apply_move(board_move!(e1 g1)));
        assert_eq!(
            board,
            board!(
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                r _ _ _ _ r k _
            )
        );
    }

    #[test]
    fn castle_white_queen_side() {
        let mut board = board!(
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            r _ _ _ k _ _ r
        );
        assert!(board.apply_move(board_move!(e1 c1)));
        assert_eq!(
            board,
            board!(
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ k r _ _ _ r
            )
        );
    }

    #[test]
    fn castle_black_king_side() {
        let mut board = board!(
            R _ _ _ K _ _ R
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
        );
        assert!(board.apply_move(board_move!(e8 g8)));
        assert_eq!(
            board,
            board!(
                R _ _ _ _ R K _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
            )
        );
    }

    #[test]
    fn castle_black_queen_side() {
        let mut board = board!(
            R _ _ _ K _ _ R
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
        );
        assert!(board.apply_move(board_move!(e8 c8)));
        assert_eq!(
            board,
            board!(
                _ _ K R _ _ _ R
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
                _ _ _ _ _ _ _ _
            )
        );
    }

    #[test]
    fn castle_requires_rook() {
        let mut board = board!(
            _ _ _ _ K _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ k _ _ _
        );
        assert!(!board.is_valid_move(board_move!(e1 c1)));
        assert!(!board.is_valid_move(board_move!(e1 g1)));
        assert!(!board.is_valid_move(board_move!(e8 c8)));
        assert!(!board.is_valid_move(board_move!(e8 g8)));
    }

    #[test]
    fn castle_forbidden_after_king_move() {
        let mut board = board!(
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            r _ _ _ k _ _ r
        );
        assert!(board.is_valid_move(board_move!(e1 c1)));
        assert!(board.is_valid_move(board_move!(e1 g1)));
        assert!(board.apply_move(board_move!(e1 e2)));
        assert!(board.apply_move(board_move!(e2 e1)));
        assert!(!board.is_valid_move(board_move!(e1 c1)));
        assert!(!board.is_valid_move(board_move!(e1 g1)));
    }

    #[test]
    fn castle_forbidden_after_rook_move() {
        let mut board = board!(
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            r _ _ _ k _ _ r
        );
        assert!(board.is_valid_move(board_move!(e1 c1)));
        assert!(board.is_valid_move(board_move!(e1 g1)));
        assert!(board.apply_move(board_move!(a1 a2)));
        assert!(board.apply_move(board_move!(a2 a1)));
        assert!(!board.is_valid_move(board_move!(e1 c1)));
        assert!(board.is_valid_move(board_move!(e1 g1)));
        assert!(board.apply_move(board_move!(h1 h2)));
        assert!(board.apply_move(board_move!(h2 h1)));
        assert!(!board.is_valid_move(board_move!(e1 c1)));
        assert!(!board.is_valid_move(board_move!(e1 g1)));
    }

    #[test]
    fn castle_forbidden_if_blocked() {
        let mut board = board!(
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            n _ _ _ _ _ _ n
            _ _ _ _ _ _ _ _
            r _ _ _ k _ _ r
        );
        assert!(board.is_valid_move(board_move!(e1 c1)));
        assert!(board.is_valid_move(board_move!(e1 g1)));
        assert!(board.apply_move(board_move!(a3 b1)));
        assert!(!board.is_valid_move(board_move!(e1 c1)));
        assert!(board.is_valid_move(board_move!(e1 g1)));
        assert!(board.apply_move(board_move!(h3 g1)));
        assert!(!board.is_valid_move(board_move!(e1 c1)));
        assert!(!board.is_valid_move(board_move!(e1 g1)));
    }

    #[test]
    fn castle_forbidden_if_movement_check() {
        let mut board = board!(
            R _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            r _ _ _ k _ _ r
        );

        assert!(board.is_valid_move(board_move!(e1 c1)));
        assert!(board.is_valid_move(board_move!(e1 g1)));

        assert!(board.apply_move(board_move!(a8 c8)));
        assert!(!board.is_valid_move(board_move!(e1 c1)));
        assert!(board.is_valid_move(board_move!(e1 g1)));

        assert!(board.apply_move(board_move!(c8 d8)));
        assert!(!board.is_valid_move(board_move!(e1 c1)));
        assert!(board.is_valid_move(board_move!(e1 g1)));

        assert!(board.apply_move(board_move!(d8 f8)));
        assert!(board.is_valid_move(board_move!(e1 c1)));
        assert!(!board.is_valid_move(board_move!(e1 g1)));

        assert!(board.apply_move(board_move!(f8 g8)));
        assert!(board.is_valid_move(board_move!(e1 c1)));
        assert!(!board.is_valid_move(board_move!(e1 g1)));
    }

    #[test]
    fn castle_forbidden_if_check() {
        let board = board!(
            _ _ _ _ K _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ N _ _ _ _ _
            r _ _ _ k _ _ r
        );
        assert!(!board.is_valid_move(board_move!(e1 c1)));
        assert!(!board.is_valid_move(board_move!(e1 g1)));
    }
}
