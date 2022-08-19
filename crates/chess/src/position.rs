use crate::{
    constants::{COL_CHARS, COL_COUNT, ROW_CHARS, ROW_COUNT, SQUARE_COUNT},
    Ic, Uc,
};
use lazy_static::lazy_static;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position(pub Uc, pub Uc);

impl Position {
    pub fn all() -> &'static [Position; SQUARE_COUNT] {
        &POSITIONS
    }

    pub fn col_count() -> Uc {
        COL_COUNT
    }

    pub fn row_count() -> Uc {
        ROW_COUNT
    }

    pub fn col_char(col: Uc) -> char {
        COL_CHARS[col as usize]
    }

    pub fn row_char(row: Uc) -> char {
        ROW_CHARS[row as usize]
    }

    pub fn index(&self) -> usize {
        return (self.0 + self.1 * COL_COUNT) as usize;
    }

    pub fn col(&self) -> Uc {
        self.0
    }

    pub fn row(&self) -> Uc {
        self.1
    }

    pub fn offset(&self, offset: PositionOffset) -> Option<Position> {
        let new_col = self.col() as Ic + offset.col();
        let new_row = self.row() as Ic + offset.row();
        if new_col >= 0
            && new_col < Position::col_count() as Ic
            && new_row >= 0
            && new_row < Position::row_count() as Ic
        {
            Some(Position(new_col as Uc, new_row as Uc))
        } else {
            None
        }
    }
}

impl ToString for Position {
    fn to_string(&self) -> String {
        format!(
            "{}{}",
            Position::col_char(self.col()),
            Position::row_char(self.row())
        )
    }
}

lazy_static! {
    static ref POSITIONS: [Position; SQUARE_COUNT] = {
        let mut positions = [Position(0, 0); SQUARE_COUNT];
        for col in 0..COL_COUNT {
            for row in 0..ROW_COUNT {
                positions[(col + row * COL_COUNT) as usize] = Position(col, row);
            }
        }
        positions
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PositionOffset(pub Ic, pub Ic);

impl PositionOffset {
    pub fn col(&self) -> Ic {
        self.0
    }

    pub fn row(&self) -> Ic {
        self.1
    }
}
