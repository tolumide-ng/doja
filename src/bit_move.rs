use crate::squares::Square;

pub struct BitMove(u16);


impl BitMove {
    pub(crate) fn new(src: u8, target: u8) -> Self {
        print!("the src, and t {} ... {} |||| ", src, target);
        let src = (src as u16) << 8;
        Self(src | target as u16)
    }

    pub(crate) fn get_src(&self) -> Square  {
        let src = (self.0 >> 8) as u64;
        // println!("xsrc {src}");
        Square::from(src)
    }

    pub(crate) fn get_target(&self) -> Square {
        let target = (self.0 as u8) as u64;
        // println!("target {}", target);
        Square::from(target)
    }
}