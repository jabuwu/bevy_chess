#[derive(Default)]
pub struct GameSettings {
    white_control: GameControl,
    black_control: GameControl,
}

impl GameSettings {
    pub fn control(&self, color: chess::PieceColor) -> &GameControl {
        match color {
            chess::PieceColor::White => &self.white_control,
            chess::PieceColor::Black => &self.black_control,
        }
    }

    pub fn control_mut(&mut self, color: chess::PieceColor) -> &mut GameControl {
        match color {
            chess::PieceColor::White => &mut self.white_control,
            chess::PieceColor::Black => &mut self.black_control,
        }
    }
}

#[derive(Default)]
pub enum GameControl {
    #[default]
    Player,
    Ai,
}
