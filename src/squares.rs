

#[derive(Debug, Clone, Copy)]
pub enum Square {
    A8=0, B8=1, C8=2, D8=3, E8=4, F8=5, G8=6, H8=7,
    A7=8, B7=9, C7=10, D7=11, E7=12, F7=13, G7=14, H7=15,
    A6=16, B6=17, C6=18, D6=19, E6=20, F6=21, G6=22, H6=23,
    A5=24, B5=25, C5=26, D5=27, E5=28, F5=29, G5=30, H5=31,
    A4=32, B4=33, C4=34, D4=35, E4=36, F4=37, G4=38, H4=39,
    A3=40, B3=41, C3=42, D3=43, E3=44, F3=45, G3=46, H3=47,
    A2=48, B2=49, C2=50, D2=51, E2=52, F2=53, G2=54, H2=55,
    A1=56, B1=57, C1=58, D1=59, E1=60, F1=61, G1=62, H1=63,
}

impl From<Square> for u64 {
    fn from(value: Square) -> Self {
        value as u64
    }
}

impl Square {
    pub fn name(index: u64) -> String {
        String::from(SQUARE_NAMES[index as usize])
    }
}


pub(crate) const SQUARE_NAMES: [&str; 64] = [
    "A8", "B8", "C8", "D8", "E8", "F8", "G8", "H8",
    "A7", "B7", "C7", "D7", "E7", "F7", "G7", "H7",
    "A6", "B6", "C6", "D6", "E6", "F6", "G6", "H6",
    "A5", "B5", "C5", "D5", "E5", "F5", "G5", "H5",
    "A4", "B4", "C4", "D4", "E4", "F4", "G4", "H4",
    "A3", "B3", "C3", "D3", "E3", "F3", "G3", "H3",
    "A2", "B2", "C2", "D2", "E2", "F2", "G2", "H2",
    "A1", "B1", "C1", "D1", "E1", "F1", "G1", "H1",
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


pub(crate) const BIT_TABLE: [u64; 64] = [
  63, 30, 3, 32, 25, 41, 22, 33, 15, 50, 42, 13, 11, 53, 19, 34, 61, 29, 2,
  51, 21, 43, 45, 10, 18, 47, 1, 54, 9, 57, 0, 35, 62, 31, 40, 4, 49, 5, 52,
  26, 60, 6, 23, 44, 46, 27, 56, 16, 7, 39, 48, 24, 59, 14, 12, 55, 38, 28,
  58, 20, 37, 17, 36, 8
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


// pub(crate) const ROOK_MAGIC_NUMBERS: [u64; 64] = [
//     648537039114895360,
//     9277425403402653696,
//     36063981659521032,
//     1188967893946171652,
//     432349996904350208,
//     1297054293458946049,
//     288269967706556417,
//     72061035389976586,
//     144256064076939272,
//     36099234486820864,
//     291889623326588992,
//     288652622977565056,
//     1153062259275595904,
//     2307110196398277632,
//     2594214127148859520,
//     9511883892278182050,
//     142386759991344,
//     27021872645275713,
//     4583864261419008,
//     9289773945454626,
//     1126449730289793,
//     288415098938081316,
//     9225909709826982433,
//     283674209951809,
//     2342012683307466760,
//     90072289975996416,
//     1585284663176339456,
//     29282195826806784,
//     563095982573584,
//     9236887235931013248,
//     5836701418136797832,
//     71476845772868,
//     180144122571522112,
//     153157573858574338,
//     580999537393406209,
//     2599140003833462816,
//     28288269527548928,
//     9241667944751039488,
//     5228819913463824640,
//     9229005952923992193,
//     70370893791232,
//     35185714282497,
//     1152941296361537600,
//     577025077183905808,
//     36037593179127936,
//     4657285036730613764,
//     144194499212738561,
//     1152939663802105861,
//     281752002113792,
//     6917670890448629248,
//     9295447223615651968,
//     9232388032471007360,
//     60799213528680704,
//     432349962307666048,
//     149542439814144,
//     9367525210762314240,
//     598719515918850,
//     18647717561532673,
//     791925431033866,
//     4723643691339777,
//     288793395362006018,
//     281509370265601,
//     9368067771736885252,
//     4611832403936411778,
// ];

// pub(crate) const BISHOP_MAGIC_NUMBERS: [u64; 64] = [
//     54061064748303490,
//     3467774479306883328,
//     180997210463404182,
//     1129199054094352,
//     1157574778475855872,
//     312016259371794432,
//     5909288139796512768,
//     1153062794009542656,
//     70377477832960,
//     4611721488750887042,
//     2254016054657024,
//     10467105509646270496,
//     3462160390380060672,
//     585769256432451844,
//     72066974380851712,
//     145300480149948416,
//     11817588531738543112,
//     2314859013185088016,
//     1346502816,
//     9371990867554877440,
//     4611691809386004544,
//     297246388948041760,
//     648538431823087120,
//     2309220711820165721,
//     11529514664230713344,
//     54976186419776,
//     1297046598085775392,
//     144119586155930115,
//     288231492844273672,
//     198168313610670096,
//     288302952786199552,
//     20269050265600128,
//     9223399802759825922,
//     8813560250752,
//     2305843155780501632,
//     9385501795238838816,
//     2918333662343729216,
//     1152947893456470528,
//     576751032500519442,
//     144279023898477056,
//     2305846325465334304,
//     18015533991462410,
//     14994736471829644312,
//     1152923704720625697,
//     162692674279710976,
//     72066564099145888,
//     10394450947366454536,
//     27022698635067969,
//     1161937680377389184,
//     9260548736984293392,
//     288230668444438032,
//     289110057428746240,
//     1153202983962673216,
//     563001765929216,
//     1157429811652610048,
//     1139256918024,
//     5557442515718636097,
//     2252367035105828,
//     281518482655264,
//     33686019,
//     23098548873339937,
//     10376645394041931144,
//     36031032302239872,
//     36031067464009858,
// ];