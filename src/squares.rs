use std::{fmt::Display, ops::Index};


// TODO! square should have u8 values not u64
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    NoSquare=64,
    A8=56, B8=57, C8=58, D8=59, E8=60, F8=61, G8=62, H8=63,
    A7=48, B7=49, C7=50, D7=51, E7=52, F7=53, G7=54, H7=55,
    A6=40, B6=41, C6=42, D6=43, E6=44, F6=45, G6=46, H6=47,
    A5=32, B5=33, C5=34, D5=35, E5=36, F5=37, G5=38, H5=39,
    A4=24, B4=25, C4=26, D4=27, E4=28, F4=29, G4=30, H4=31,
    A3=16, B3=17, C3=18, D3=19, E3=20, F3=21, G3=22, H3=23,
    A2=8,  B2=9,  C2=10, D2=11, E2=12, F2=13, G2=14, H2=15,
    A1=0,  B1=1,  C1=2,  D1=3,  E1=4,  F1=5,  G1=6,  H1=7, 
}

impl From<Square> for u64 {
    fn from(value: Square) -> Self {
        value as u64
    }
}


impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let square = SQUARE_NAMES[*self].to_lowercase();
        write!(f, "{square}")
    }
}


impl<T> Index<Square> for [T] {
    type Output = T;
    
    fn index(&self, index: Square) -> &Self::Output {
        // match index {
        //     Square::NoSquare => &self[64],
        //     Square::A8 => &self[56],
        //     Square::B8 => &self[57],
        //     Square::C8 => &self[58],
        //     Square::D8 => &self[59],
        //     Square::E8 => &self[60],
        //     Square::F8 => &self[61],
        //     Square::G8 => &self[62],
        //     Square::H8 => &self[63],
        //     Square::A7 => &self[48],
        //     Square::B7 => &self[49],
        //     Square::C7 => &self[50],
        //     Square::D7 => &self[51],
        //     Square::E7 => &self[52],
        //     Square::F7 => &self[53],
        //     Square::G7 => &self[54],
        //     Square::H7 => &self[55],
        //     Square::A6 => &self[40],
        //     Square::B6 => &self[41],
        //     Square::C6 => &self[42],
        //     Square::D6 => &self[43],
        //     Square::E6 => &self[44],
        //     Square::F6 => &self[45],
        //     Square::G6 => &self[46],
        //     Square::H6 => &self[47],
        //     Square::A5 => &self[32],
        //     Square::B5 => &self[33],
        //     Square::C5 => &self[34],
        //     Square::D5 => &self[35],
        //     Square::E5 => &self[36],
        //     Square::F5 => &self[37],
        //     Square::G5 => &self[38],
        //     Square::H5 => &self[39],
        //     Square::A4 => &self[24],
        //     Square::B4 => &self[25],
        //     Square::C4 => &self[26],
        //     Square::D4 => &self[27],
        //     Square::E4 => &self[28],
        //     Square::F4 => &self[29],
        //     Square::G4 => &self[30],
        //     Square::H4 => &self[31],
        //     Square::A3 => &self[16],
        //     Square::B3 => &self[17],
        //     Square::C3 => &self[18],
        //     Square::D3 => &self[19],
        //     Square::E3 => &self[20],
        //     Square::F3 => &self[21],
        //     Square::G3 => &self[22],
        //     Square::H3 => &self[23],
        //     Square::A2 => &self[8],
        //     Square::B2 => &self[9],
        //     Square::C2 => &self[10],
        //     Square::D2 => &self[11],
        //     Square::E2 => &self[12],
        //     Square::F2 => &self[13],
        //     Square::G2 => &self[14],
        //     Square::H2 => &self[15],
        //     Square::A1 => &self[0],
        //     Square::B1 => &self[1],
        //     Square::C1 => &self[2],
        //     Square::D1 => &self[3],
        //     Square::E1 => &self[4],
        //     Square::F1 => &self[5],
        //     Square::G1 => &self[6],
        //     Square::H1 => &self[7],
        // }
   


        let sq = index as u64;
        &self[sq as usize]  
    }
}



impl From<u64> for Square {
    fn from(value: u64) -> Self {
        match value {
            64 => Square::NoSquare,
            56 => Square::A8,
            57 => Square::B8,
            58 => Square::C8,
            59 => Square::D8,
            60 => Square::E8,
            61 => Square::F8,
            62 => Square::G8,
            63 => Square::H8,
            48 => Square::A7,
            49 => Square::B7,
            50 => Square::C7,
            51 => Square::D7,
            52 => Square::E7,
            53 => Square::F7,
            54 => Square::G7,
            55 => Square::H7,
            40 => Square::A6,
            41 => Square::B6,
            42 => Square::C6,
            43 => Square::D6,
            44 => Square::E6,
            45 => Square::F6,
            46 => Square::G6,
            47 => Square::H6,
            32 => Square::A5,
            33 => Square::B5,
            34 => Square::C5,
            35 => Square::D5,
            36 => Square::E5,
            37 => Square::F5,
            38 => Square::G5,
            39 => Square::H5,
            24 => Square::A4,
            25 => Square::B4,
            26 => Square::C4,
            27 => Square::D4,
            28 => Square::E4,
            29 => Square::F4,
            30 => Square::G4,
            31 => Square::H4,
            16 => Square::A3,
            17 => Square::B3,
            18 => Square::C3,
            19 => Square::D3,
            20 => Square::E3,
            21 => Square::F3,
            22 => Square::G3,
            23 => Square::H3,
            8 => Square::A2,
            9 => Square::B2,
            10 => Square::C2,
            11 => Square::D2,
            12 => Square::E2,
            13 => Square::F2,
            14 => Square::G2,
            15 => Square::H2,
            0 => Square::A1,
            1 => Square::B1,
            2 => Square::C1,
            3 => Square::D1,
            4 => Square::E1,
            5 => Square::F1,
            6 => Square::G1,
            7 => Square::H1,
            _ => panic!("Unrecognized value: {value}")
        }
    }
}

pub(crate) const BIT_TABLE: [u64; 64] = [
  63, 30, 3,  32, 25, 41, 22, 33,
  15, 50, 42, 13, 11, 53, 19, 34,
  61, 29, 2,  51, 21, 43, 45, 10,
  18, 47, 1,  54, 9,  57, 0,  35,
  62, 31, 40, 4, 49,  5,  52, 26,
  60, 6,  23, 44, 46, 27, 56, 16,
  7,  39, 48, 24, 59, 14, 12, 55,
  38, 28, 58, 20, 37, 17, 36, 8
];


impl Square {
    pub fn name(index: u64) -> String {
        String::from(SQUARE_NAMES[index as usize])
    }

    pub(crate) fn rank(&self) -> usize {
        let value = u64::from(*self);
        return ((value / 8) + 1) as usize;
    }

    /// flip the square on the board vertically
    pub(crate) fn flipv(&self) -> Self {
        Self::from((*self as u8 ^ 56 & 63) as u64)
    }
}


// impl From<Square> for u8 {
//     fn from(value: Square) -> Self {
//         return SQUARE_INDEX[]
//     }
// }

// pub(crate) const SQUARE_INDEX: [u8; 64] = [
//     56, 57, 58, 59, 60, 61, 62, 63,
//     48, 49, 50, 51, 52, 53, 54, 55,
//     40, 41, 42, 43, 44, 45, 46, 47,
//     32, 33, 34, 35, 36, 37, 38, 39,
//     24, 25, 26, 27, 28, 29, 30, 31,
//     16, 17, 18, 19, 20, 21, 22, 23, 
//     8,  9,  10, 11, 12, 13, 14, 15,
//     0,  1,  2,  3,  4,  5,  6,  7,
// ];


pub(crate) const SQUARE_NAMES: [&str; 64] = [
    "A1", "B1", "C1", "D1", "E1", "F1", "G1", "H1",
    "A2", "B2", "C2", "D2", "E2", "F2", "G2", "H2",
    "A3", "B3", "C3", "D3", "E3", "F3", "G3", "H3",
    "A4", "B4", "C4", "D4", "E4", "F4", "G4", "H4",
    "A5", "B5", "C5", "D5", "E5", "F5", "G5", "H5",
    "A6", "B6", "C6", "D6", "E6", "F6", "G6", "H6",
    "A7", "B7", "C7", "D7", "E7", "F7", "G7", "H7",
    "A8", "B8", "C8", "D8", "E8", "F8", "G8", "H8",
];




// Relevant bishop occupancy bit count for every square on board
pub(crate) const BISHOP_RELEVANT_BITS: [u32; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 
    5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 9, 9, 7, 5, 5,
    5, 5, 7, 7, 7, 7, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5,
    6, 5, 5, 5, 5, 5, 5, 6,
];


// Relevant rook occupancy bit count for every square on board
pub(crate) const ROOK_RELEVANT_BITS: [u32; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    11, 10, 10, 10, 10, 10, 10, 11,
    12, 11, 11, 11, 11, 11, 11, 12,
];


pub(crate) const ROOK_MAGIC_NUMBERS: [u64; 64] = [
    0x8a80104000800020,
    0x140002000100040,
    0x2801880a0017001,
    0x100081001000420,
    0x200020010080420,
    0x3001c0002010008,
    0x8480008002000100,
    0x2080088004402900,
    0x800098204000,
    0x2024401000200040,
    0x100802000801000,
    0x120800800801000,
    0x208808088000400,
    0x2802200800400,
    0x2200800100020080,
    0x801000060821100,
    0x80044006422000,
    0x100808020004000,
    0x12108a0010204200,
    0x140848010000802,
    0x481828014002800,
    0x8094004002004100,
    0x4010040010010802,
    0x20008806104,
    0x100400080208000,
    0x2040002120081000,
    0x21200680100081,
    0x20100080080080,
    0x2000a00200410,
    0x20080800400,
    0x80088400100102,
    0x80004600042881,
    0x4040008040800020,
    0x440003000200801,
    0x4200011004500,
    0x188020010100100,
    0x14800401802800,
    0x2080040080800200,
    0x124080204001001,
    0x200046502000484,
    0x480400080088020,
    0x1000422010034000,
    0x30200100110040,
    0x100021010009,
    0x2002080100110004,
    0x202008004008002,
    0x20020004010100,
    0x2048440040820001,
    0x101002200408200,
    0x40802000401080,
    0x4008142004410100,
    0x2060820c0120200,
    0x1001004080100,
    0x20c020080040080,
    0x2935610830022400,
    0x44440041009200,
    0x280001040802101,
    0x2100190040002085,
    0x80c0084100102001,
    0x4024081001000421,
    0x20030a0244872,
    0x12001008414402,
    0x2006104900a0804,
    0x1004081002402
];

// bishop magic numbers
pub(crate) const BISHOP_MAGIC_NUMBERS: [u64; 64] = [
    0x40040844404084,
    0x2004208a004208,
    0x10190041080202,
    0x108060845042010,
    0x581104180800210,
    0x2112080446200010,
    0x1080820820060210,
    0x3c0808410220200,
    0x4050404440404,
    0x21001420088,
    0x24d0080801082102,
    0x1020a0a020400,
    0x40308200402,
    0x4011002100800,
    0x401484104104005,
    0x801010402020200,
    0x400210c3880100,
    0x404022024108200,
    0x810018200204102,
    0x4002801a02003,
    0x85040820080400,
    0x810102c808880400,
    0xe900410884800,
    0x8002020480840102,
    0x220200865090201,
    0x2010100a02021202,
    0x152048408022401,
    0x20080002081110,
    0x4001001021004000,
    0x800040400a011002,
    0xe4004081011002,
    0x1c004001012080,
    0x8004200962a00220,
    0x8422100208500202,
    0x2000402200300c08,
    0x8646020080080080,
    0x80020a0200100808,
    0x2010004880111000,
    0x623000a080011400,
    0x42008c0340209202,
    0x209188240001000,
    0x400408a884001800,
    0x110400a6080400,
    0x1840060a44020800,
    0x90080104000041,
    0x201011000808101,
    0x1a2208080504f080,
    0x8012020600211212,
    0x500861011240000,
    0x180806108200800,
    0x4000020e01040044,
    0x300000261044000a,
    0x802241102020002,
    0x20906061210001,
    0x5a84841004010310,
    0x4010801011c04,
    0xa010109502200,
    0x4a02012000,
    0x500201010098b028,
    0x8040002811040900,
    0x28000010020204,
    0x6000020202d0240,
    0x8918844842082200,
    0x4010011029020020
];
