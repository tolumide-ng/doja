use std::str::SplitWhitespace;

use super::UciError;

#[derive(Debug, Default, Clone, PartialEq)]
pub(crate) enum Counter {
    /// Only search up to depth x
    Depth(u8),
    /// Search for a move in x msec
    Time(u64),
    /// Search x nodes
    Nodes(u64),
    #[default]
    Infinite,
    /// Search for a mate in x plys
    Mate(u64),
    Dynamic {
        wtime: u64,
        btime: u64,
        winc: Option<u64>,
        binc: Option<u64>,
        movestogo: Option<u64>
    }
    // Search
}

impl Counter {
    fn parse_dynamic<F>(mut input: SplitWhitespace, name: &'static str, f: F) -> Result<Counter, UciError> 
        where F: Fn((&mut Self, u64)) {
        let Some(pre_value) = input.next() else { return Err(UciError::NoValue(&name)) };
        let Ok(value) = u64::from_str_radix(pre_value, 10) else {return Err(UciError::InvalidIntegerArgument(pre_value.to_string()))};

        let counter = match Self::try_from(input) {
            Ok(mut mgr) if matches!(mgr, Counter::Dynamic { .. })=> {
                f((&mut mgr, value));
                mgr
            },
            Err(e) if e == UciError::EmptyArgument => { (Self::Dynamic { wtime: 0, btime: 0, winc: None, binc: None, movestogo: None }) },
            Err(e) => return Err(e),
            _ => return Err(UciError::NoValue(""))
        };
        
        Ok(counter)
    }
}

impl<'a> TryFrom<SplitWhitespace<'a>> for Counter {
    type Error = UciError;
    fn try_from(mut input: SplitWhitespace) -> Result<Self, Self::Error> {
        // let depth
        match input.next() {
            Some("infinite") => { return Ok(Counter::Infinite) }
            // Some("searchmoves") => {}
            Some("depth") => {
                let Some(d) = input.next() else {return Err(UciError::NoValue("depth"))};
                let Ok(depth) = u8::from_str_radix(d, 10) else {return Err(UciError::InvalidIntegerArgument(d.to_string()))};
                return Ok(Counter::Depth(depth))
            }
            Some("time") => {
                let Some(t) = input.next() else { return Err(UciError::NoValue("time"))};
                let Ok(time) = u64::from_str_radix(t, 10) else { return Err(UciError::InvalidIntegerArgument(t.to_string()))};
                return Ok(Counter::Time(time))
            }
            Some("nodes") => {
                let Some(n) = input.next() else { return Err(UciError::NoValue("time"))};
                let Ok(nodes) = u64::from_str_radix(n, 10) else { return Err(UciError::InvalidIntegerArgument(n.to_string()))};
                return Ok(Counter::Nodes(nodes))
            }
            Some("mate") => {
                let Some(m) = input.next() else { return Err(UciError::NoValue("mate"))};
                let Ok(mate) = u64::from_str_radix(m, 10) else { return Err(UciError::InvalidIntegerArgument(m.to_string()))};
                let ply = mate * 2;
                return Ok(Counter::Mate(ply))
            }
            Some("binc") => {
                Self::parse_dynamic(input, "binc", |mut arg: ( &mut Counter, u64)| { if let Counter::Dynamic {binc, .. } = &mut arg.0 { *binc = Some(arg.1) }})
            }
            Some("winc") => {
                Self::parse_dynamic(input, "winc", |mut arg: ( &mut Counter, u64)| { if let Counter::Dynamic {winc, .. } = &mut arg.0 { *winc = Some(arg.1) }})
            }
            Some("btime") => {
                Self::parse_dynamic(input, "btime", |mut arg: ( &mut Counter, u64)| { if let Counter::Dynamic {btime, .. } = &mut arg.0 { *btime = arg.1 }})
            }
            Some("wtime") => {
                Self::parse_dynamic(input, "wtime", |mut arg: ( &mut Counter, u64)| { if let Counter::Dynamic {wtime, .. } = &mut arg.0 { *wtime = arg.1 }})
            }
            Some("movestogo") => {
                Self::parse_dynamic(input, "movestogo", |mut arg: ( &mut Counter, u64)| { if let Counter::Dynamic {movestogo, .. } = &mut arg.0 { *movestogo = Some(arg.1) }})
            }
            _ => Err(UciError::EmptyArgument)
        }
        // Err("")
    }
}
