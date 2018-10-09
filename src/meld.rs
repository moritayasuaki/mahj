use tile::*;
use table::*;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Meld(u32);
impl Meld {
    pub fn from_raw(raw: usize) -> Self {
        Meld((raw & 0xffffffff) as u32)
    }
    pub fn raw(self) -> usize {
        self.0 as usize
    }
    pub fn from_set_robinfo(set: Set, robbed_from: usize, added_from: usize) -> Self {
        let mut raw = 0;
        raw |= set.raw();
        raw |= robbed_from << 8;
        raw |= added_from << 16;
        Self::from_raw(raw)
    }

    pub fn set(self) -> Set {
        Set::from_raw(self.raw() & 0xff)
    }
    pub fn wind(self, river: &Rivers) -> Option<Wind> {
        river.get(self.robbed_from())?.robbed_by()
    }
    pub fn robbed_from(self) -> usize {
        (self.raw() & 0xff00) >> 8
    }
    pub fn added_from(self) -> usize {
        (self.raw() & 0xff0000) >> 16
    }
}