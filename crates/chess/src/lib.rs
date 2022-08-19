pub type Uc = u8;
pub type Ic = i8;

pub use board::{Board, BoardMove};
pub use chess_macros::{board, board_move};
pub use piece::{Piece, PieceColor, PieceKind};
pub use position::{Position, PositionOffset};

mod board;
mod constants;
mod piece;
mod position;
mod rules;
