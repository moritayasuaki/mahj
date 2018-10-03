use tile::*;
use table::*;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct MeldType(u8);
impl MeldType {
    const CHOW: Self = MeldType(0);
    const PUNG: Self = MeldType(1);
    const KONG: Self = MeldType(2);
    const ADDED_KONG: Self = MeldType(3);
    pub fn id(self) -> usize {
        self.0 as usize
    }
    pub fn from_id(id: usize) -> Self{
        MeldType((id % 4) as u8)
    }
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Meld(u32);
impl Meld {
    pub fn from_raw(raw: usize) -> Meld {
        Meld((raw & 0xffffffff) as u32)
    }
    pub fn raw(self) -> usize {
        self.0 as usize
    }
    pub fn suitranks(self) -> SuitRanks {
        SuitRanks::from_suitranks(self.suit(), self.ranks())
    }
    pub fn suit(self) -> Suit {
        Suit::from_id((self.raw() >> 18) & 0x3)
    }
    pub fn ranks(self) -> Ranks {
        let r = (self.raw() >> 16) & 0x3;
        match self.meldtype() {
            MeldType::CHOW => Ranks::from_raw(0o111 << r),
            MeldType::PUNG => Ranks::from_raw(0o3 << r),
            MeldType::KONG => Ranks::from_raw(0o4 << r),
            MeldType::ADDED_KONG => Ranks::from_raw(0o4 << r),
            _ => unreachable!()
        }
    }
    pub fn meldtype(self) -> MeldType {
        MeldType::from_id((self.raw() >> 14) & 0x3)
    }
    pub fn robbed_from(self) -> usize {
        self.raw() & 0x7f
    }
    pub fn added_from(self) -> usize {
        (self.raw() >> 7) & 0x7f
    }
}