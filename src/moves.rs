use crate::color::Color;

pub trait Moves {
    fn enemy_or_empty() {}
}



trait Player: Moves {}

struct White {}
impl Player for White {}
impl Moves for White {}

struct Black {}
impl Player for Black {}
impl Moves for Black {}


