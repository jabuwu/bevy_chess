fn main() {
    let mut board = chess::Board::new();
    let mut turn = chess::PieceColor::White;
    loop {
        if let Some(ai_move) = chess_ai::plan_move(&board, turn) {
            board.apply_move(ai_move);
            println!("{:?}", board);
            println!("");
            turn = turn.opposite();
        } else {
            if board.check(turn) {
                println!("checkmate");
                match turn {
                    chess::PieceColor::White => {
                        println!("black won");
                    }
                    chess::PieceColor::Black => {
                        println!("white won");
                    }
                }
            } else {
                println!("stalemate");
            }
            break;
        }
    }
}
