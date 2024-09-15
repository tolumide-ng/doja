/**
 * https://www.chessprogramming.org/Kogge-Stone_Algorithm
 * */


 
use crate::shift::Shift;

pub struct KoggeStone;

impl KoggeStone {
    fn rotate_left(x: u64, s: i8) -> u64 {
        if s >= 0 {
            // return bits.rotate_left(s as u32)
            return (x << s) | (x >> (64-s))
        }
        return x.rotate_right((-s) as u32);
    }
    fn rotate_right(x: u64, s: i8) -> u64 {(x >> s) | (x << (64-s))}


    /// Generates the fill in the provided direction, including the attacker's square
    /// e.g. if the attacker is on F3, and we want to generate east attacks, this method
    /// would return a u64 with squares F3, G3, and H3 filled
    pub(crate) fn occluded_fill(gen: u64, pro: u64, shift: Shift) -> u64 {
        let mut pro = pro; let mut gen = gen;
        let r = shift.amount;
        
        pro &= shift.mask;

        // because rust only supports rotate_left and right by u32
        gen |= pro & Self::rotate_left(gen, r);
        pro &= Self::rotate_left(pro, r);
        gen |= pro & Self::rotate_left(gen, 2*r);
        pro &= Self::rotate_left(pro, 2*r);
        gen |= pro & Self::rotate_left(gen, 4*r);
        gen
    }

    /// Removes the originating square of the chosen direction
    /// e.g given a bitboard with F3, E2, D1 filled, and the shift as SouthWest,
    /// this board returns a bitboard with only E2, and D1 filled, since F3 would
    /// be the originating square of provided fills
    pub(crate) fn shift_one(b: u64, shift: Shift) -> u64 {
        let r = shift.amount;
        Self::rotate_left(b, r) & shift.mask
    }

    pub(crate) fn sliding_attacks(sliders: u64, empty: u64, shift: Shift) -> u64 {
        let fill = Self::occluded_fill(sliders, empty, shift);
        Self::shift_one(fill, shift)
    }

    
}


#[cfg(test)]
mod koggestone_tests {
    use crate::{bitboard::Bitboard, squares::Square};
    const BOARD_EXCLUDING_ATTACKER: u64 = 0x20001024000290u64; // E1, H1, D2, C4, F4, E5, F7

    use super::*;


    #[test]
    fn should_return_the_occluded_fill() {
        let attacker = (1 << (Square::F3 as u64)) as u64;
        let board_excluding_attacker = 0x20001024000290u64; // E1, H1, D2, C4, F4, E5, F7
        let opponents = !board_excluding_attacker;

        let result = KoggeStone::occluded_fill(attacker, opponents, Shift::SouthWest);

        let fills = [Square::F3, Square::E2, Square::D1];

        assert_eq!(result.count_ones() as usize, fills.len());
        for sq in fills {
            assert_eq!(Bitboard(result).get_bit(sq as u64), 1);
        }
    }

    #[test]
    fn shifts_the_provided_board_by_shift() {
        let board = (1 << Square::F3 as u8) | (1 << Square::E2 as u8) | (1 << Square::D1 as u8);

        let result = KoggeStone::shift_one(board, Shift::SouthWest);
        // println!("{:#?}", Bitboard::from(attacker).to_string());
        // println!("{:#?}", Bitboard::from(board).to_string());
        // println!("{:#?}", Bitboard::from(result).to_string());
        
        let fills = [Square::E2, Square::D1];

        assert_eq!(result.count_ones() as usize, fills.len());
        for sq in fills {
            assert_eq!(Bitboard(result).get_bit(sq as u64), 1);
        }

    }

    #[cfg(test)]
    mod sliding_attacks {
        use super::*;

        #[test]
        fn should_generate_sliding_attacks_northwest() {
            let attacker = (1 << (Square::E4 as u64)) as u64;
            let board_excluding_attacker = 0x20001024000290u64; // E1, H1, D2, C4, F4, E5, F7
            let opponents = !board_excluding_attacker;
            let result = KoggeStone::sliding_attacks(attacker, opponents, Shift::NorthWest);
            
            let north_west_attacks = [Square::D5, Square::C6, Square::B7, Square::A8];
    
            assert_eq!(result.count_ones() as usize, north_west_attacks.len());
            for sq in north_west_attacks {
                assert_eq!(Bitboard(result).get_bit(sq as u64), 1);
            }
    
            let board_excluding_attacker = 0x20041024000290u64; // E1, H1, D2, C4, F4, E5, C6 F7
            let opponents = !board_excluding_attacker;
            let result = KoggeStone::sliding_attacks(attacker, opponents, Shift::NorthWest);
            
            let clipped_north_west_attacks = [Square::D5, Square::C6];
            
            assert_eq!(result.count_ones() as usize, clipped_north_west_attacks.len());
            for sq in clipped_north_west_attacks {
                assert_eq!(Bitboard(result).get_bit(sq as u64), 1);
            }
        }
    
        #[test]
        fn should_generate_sliding_attacks_north() {
            let attacker = (1 << (Square::D2 as u64)) as u64;
            let board_excluding_attacker = 0x20001024000290u64; // E1, H1, D2, C4, F4, E5, F7
            let opponents = !board_excluding_attacker;
            let result = KoggeStone::sliding_attacks(attacker, opponents, Shift::North);
            
            let north_west_attacks = [Square::D3, Square::D4, Square::D5, Square::D6, Square::D7, Square::D8];
    
            assert_eq!(result.count_ones() as usize, north_west_attacks.len());
            for sq in north_west_attacks {
                assert_eq!(Bitboard(result).get_bit(sq as u64), 1);
            }
    
            let attacker_e4 = (1 << (Square::E4 as u64)) as u64;
            let board_excluding_attacker = 0x20041024000290u64; // E1, H1, D2, C4, F4, E5, C6 F7
            let opponents = !board_excluding_attacker;
            let result = KoggeStone::sliding_attacks(attacker_e4, opponents, Shift::North);
            
            let clipped_north_west_attacks = [Square::E5];
            
            assert_eq!(result.count_ones() as usize, clipped_north_west_attacks.len());
            for sq in clipped_north_west_attacks {
                assert_eq!(Bitboard(result).get_bit(sq as u64), 1);
            }
        }
    
        #[test]
        fn should_generate_sliding_attacks_northeast() {
            let attacker_d3 = (1 << (Square::D3) as u8) as u64; // square E6
            let board_excluding_attacker = 0x20001024000290u64; // E1, H1, D2, C4, F4, E5, F7
            let opponents = !board_excluding_attacker;
            let result = KoggeStone::sliding_attacks(attacker_d3, opponents, Shift::NorthEast);
            
            let northeast_attacks = [Square::E4, Square::F5, Square::G6, Square::H7];
    
            assert_eq!(result.count_ones() as usize, northeast_attacks.len());
            for sq in northeast_attacks {
                assert_eq!(Bitboard(result).get_bit(sq as u64), 1);
            }
    
            let attacker_e6 = 0x100000000000u64;
            let board_excluding_attacker = 0x20041024000290u64; // E1, H1, D2, C4, F4, E5, C6 F7
            let opponents = !board_excluding_attacker;
            let result = KoggeStone::sliding_attacks(attacker_e6, opponents, Shift::NorthEast);
            
            let clipped_north_west_attacks = [Square::F7];
    
            assert_eq!(result.count_ones() as usize, clipped_north_west_attacks.len());
            for sq in clipped_north_west_attacks {
                assert_eq!(Bitboard(result).get_bit(sq as u64), 1);
            }
        }
    
        #[test]
        fn should_generate_sliding_attacks_east() {
            let attacker_d3 = 1 << (Square::D3 as u64); // square D3
            let board_excluding_attacker = 0x20001024000290u64; // E1, H1, D2, C4, F4, E5, F7
            let opponents = !board_excluding_attacker;
            let result = KoggeStone::sliding_attacks(attacker_d3, opponents, Shift::East);
            
            let attacks = [Square::E3, Square::F3, Square::G3, Square::H3];
    
            
            assert_eq!(result.count_ones() as usize, attacks.len());
            for sq in attacks {
                assert_eq!(Bitboard(result).get_bit(sq as u64), 1);
            }
            
            let attacker_b1 = 1 << Square::B1 as u64;
            let board_excluding_attacker = 0x20041024000290u64; // E1, H1, D2, C4, F4, E5, C6 F7
            let opponents = !board_excluding_attacker;
            let result = KoggeStone::sliding_attacks(attacker_b1, opponents, Shift::East);
            
            let clipped_attacks = [Square::C1, Square::D1, Square::E1,];
    
            assert_eq!(result.count_ones() as usize, clipped_attacks.len());
            for sq in clipped_attacks {
                assert_eq!(Bitboard(result).get_bit(sq as u64), 1);
            }
        }
        
        
        #[test]
        fn should_generate_sliding_attacks_west() {
            let attacker_d3 = 1 << (Square::D3 as u64); // square E6
    
            let board_excluding_attacker = 0x20001024000290u64; // E1, H1, D2, C4, F4, E5, F7
            let opponents = !board_excluding_attacker;
            let result = KoggeStone::sliding_attacks(attacker_d3, opponents, Shift::West);
            
            let attacks = [Square::C3, Square::B3, Square::A3];

    
            assert_eq!(result.count_ones() as usize, attacks.len());
            for sq in attacks {
                assert_eq!(Bitboard(result).get_bit(sq as u64), 1);
            }
    
            let attacker_g6 = 1 << (Square::G6 as u64);
            let board_excluding_attacker = 0x20041024000290u64; // E1, H1, D2, C4, F4, E5, C6 F7
            let opponents = !board_excluding_attacker;
            let result = KoggeStone::sliding_attacks(attacker_g6, opponents, Shift::West);
            
            let clipped_attacks = [Square::F6, Square::E6, Square::D6, Square::C6];
            
            assert_eq!(result.count_ones() as usize, clipped_attacks.len());
            for sq in clipped_attacks {
                assert_eq!(Bitboard(result).get_bit(sq as u64), 1);
            }
        }
    }

}