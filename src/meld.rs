use tile::*;
use table::*;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct MeldType(u8);
impl MeldType {
    pub const CHOW: Self = MeldType(0);
    pub const PUNG: Self = MeldType(1);
    pub const KONG: Self = MeldType(2);
    pub const ADDED_KONG: Self = MeldType(3);
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
    pub fn from_raw(raw: usize) -> Self {
        Meld((raw & 0xffffffff) as u32)
    }
    pub fn raw(self) -> usize {
        self.0 as usize
    }
    pub fn from_rep_meldtype_robbed_added(rep: Figure, meld_type: MeldType, robbed_from: usize, added_from: usize) -> Self {
        let mut raw = 0;
        raw |= rep.suit().id() << 20;
        raw |= rep.rank().id() << 16;
        raw |= meld_type.id() << 14;
        raw |= robbed_from;
        raw |= added_from << 7;
        Self::from_raw(raw)
    }

    pub fn suitranks(self) -> SuitRanks {
        SuitRanks::from_suitranks(self.suit(), self.ranks())
    }
    pub fn wind(self, river: &Rivers) -> Option<Wind> {
        river.get(self.robbed_from())?.robbed_by()
    }
    pub fn suit(self) -> Suit {
        Suit::from_id((self.raw() >> 20) & 0x3)
    }
    pub fn reprank(self) -> Rank {
        Rank::from_id((self.raw() >> 16) & 0xf)
    }
    pub fn ranks(self) -> Ranks {
        let r = (self.raw() >> 16) & 0xf;
        match self.meldtype() {
            MeldType::CHOW => Ranks::from_raw(0o111 << r),
            MeldType::PUNG => Ranks::from_raw(0o3 << r),
            MeldType::KONG => Ranks::from_raw(0o4 << r),
            MeldType::ADDED_KONG => Ranks::from_raw(0o4 << r),
            _ => unreachable!()
        }
    }
    pub fn rep(self) -> Figure {
        Figure::from_suitrank(self.suit(), self.reprank())
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