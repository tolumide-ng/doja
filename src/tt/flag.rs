use crate::syzygy::probe::WDL;


#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum HashFlag {
    // NoBound = 0,
   /// PV-nodes: have scores inside the window i.e. alpha < score < beta
   #[default]
   Exact = 0,
   /// Beta-cutoff nodes (FailHigh) score >= beta
   LowerBound = 1,
   /// Alpha nodes (FailLow) score < alpha
   UpperBound = 2, // alpha
}

impl From<u8> for HashFlag {
   fn from(value: u8) -> Self {
       match value {
           0 => Self::Exact,
           1 => Self::LowerBound,
           2 => Self::UpperBound,
           _ => panic!("Unrecognized hashlag {value}")
       }
   }
}


impl From<WDL> for HashFlag {
    fn from(value: WDL) -> Self {
        match value {
            WDL::Win => HashFlag::LowerBound,
            WDL::Draw => HashFlag::Exact,
            WDL::Loss => HashFlag::UpperBound,
        }
    }
}