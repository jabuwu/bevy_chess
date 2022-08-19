use chess::{board, board_move};

fn main() {
    let mut board = board!(
        R B N Q K N B R
        P P P P P P P P
        _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _
        _ _ _ _ _ _ _ _
        p p p p p p p p
        r b n q k n b r
    );
    board.apply_move(board_move!(a7 a5));
    println!("{:?}", board);
}
