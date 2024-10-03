use std::ops::Deref;

use bindings::TB_LARGEST;

use crate::{board::position::Position, color::Color};

#[allow(dead_code, non_camel_case_types, non_upper_case_globals, non_snake_case)]
mod bindings;
pub(crate) mod probe;

#[derive(Debug)]
struct SyZyGyBoard(Position);


impl Deref for SyZyGyBoard {
    type Target = Position;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl TryFrom <Position> for SyZyGyBoard {
    type Error = &'static str;

    fn try_from(value: Position) -> Result<Self, Self::Error> {
        // Maximum number of pieces supported for this Syzygy tablebase
        let max_pieces = unsafe { TB_LARGEST };
        if value.fifty.iter().sum::<u8>() == 0 && value.castling_rights.is_empty() && value.get_occupancy(Color::Both).count_ones() <= max_pieces {
            return Ok(SyZyGyBoard(value))
        }

        Err("Board does not meet the requirements for Syzygy Tablebase probing")
    }
}
