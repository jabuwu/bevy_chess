use crate::{constants::SQUARE_COUNT, Piece, PieceColor, PieceKind, Position};

#[derive(Clone, Copy)]
pub struct Promotion;

impl Promotion {
    pub fn new() -> Self {
        Self
    }

    pub fn apply_move(&mut self, pieces: &mut [Option<Piece>; SQUARE_COUNT]) {
        for position in Position::all().iter() {
            if let Some(piece) = &mut pieces[position.index()] {
                if let PieceKind::Pawn = piece.kind() {
                    let mut promote = false;
                    match piece.color() {
                        PieceColor::White => {
                            if position.row() == 0 {
                                promote = true;
                            }
                        }
                        PieceColor::Black => {
                            if position.row() == Position::row_count() - 1 {
                                promote = true;
                            }
                        }
                    }
                    if promote {
                        pieces[position.index()] =
                            Some(Piece::new(PieceKind::Queen, piece.color()));
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
    fn promotion_white() {
        let mut board = board!(
            _ _ _ _ _ _ _ P
            p _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
        );
        assert!(board.apply_move(board_move!(a7 a8)));
        assert_eq!(
            board,
            board!(
                q _ _ _ _ _ _ P
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
    fn promotion_black() {
        let mut board = board!(
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ P _ _ _
            _ _ _ _ _ _ _ p
        );
        assert!(board.apply_move(board_move!(e2 e1)));
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
                _ _ _ _ Q _ _ p
            )
        );
    }
}
