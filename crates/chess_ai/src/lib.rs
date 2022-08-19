use chess::{Board, BoardMove, PieceColor};

struct ScoredMove {
    board: Board,
    board_move: BoardMove,
    score: i8,
}

fn plan_move_depth(board: &Board, color: PieceColor, depth: u32) -> Vec<ScoredMove> {
    let mut board_moves = board.valid_moves(color);
    let mut scored_moves = vec![];
    let mut highest_score = -100;
    while let Some(board_move) = board_moves.pop() {
        let mut board = board.clone();
        board.force_move(board_move);
        let mut follow_up_score = 0;
        if depth > 0 {
            for moves in plan_move_depth(&board, color, 0) {
                let mut board = board.clone();
                board.force_move(moves.board_move);
                let score = board.score(color);
                if score > follow_up_score {
                    follow_up_score = score;
                }
            }
        }
        let mut score = board.score(color);
        if depth > 0 {
            let responses = plan_move_depth(&board, color.opposite(), depth - 1);
            for response in responses.iter() {
                let res_score = response.board.score(color);
                if response.board.score(color) < score {
                    score = res_score;
                }
            }
        }
        score += follow_up_score / 5;
        scored_moves.push(ScoredMove {
            board,
            board_move,
            score,
        });
        if score > highest_score {
            highest_score = score;
        }
    }
    while rand::random::<f32>() < 0.25 {
        highest_score -= 2;
    }
    scored_moves = scored_moves
        .into_iter()
        .filter(|board| board.score >= highest_score)
        .collect();
    scored_moves
}

pub fn plan_move(board: &Board, color: PieceColor) -> Option<BoardMove> {
    let moves = plan_move_depth(board, color, 2);
    if !moves.is_empty() {
        Some(moves[rand::random::<usize>() % moves.len()].board_move)
    } else {
        None
    }
}
