use crate::Uc;

pub const COL_COUNT: Uc = 8;
pub const COL_CHARS: [char; COL_COUNT as usize] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

pub const ROW_COUNT: Uc = 8;
pub const ROW_CHARS: [char; ROW_COUNT as usize] = ['8', '7', '6', '5', '4', '3', '2', '1'];

pub const SQUARE_COUNT: usize = (COL_COUNT * ROW_COUNT) as usize;
